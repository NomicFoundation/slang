import { pascalCase, snakeCase } from 'literal-case';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { Visitor } from '../../visitor';
import { forEachForeignEntity } from '../util';
import { rustName, typeRequiresBox } from './util';
import pluralize from 'pluralize';

export class AnalysedModelToRustInPlaceTransformerSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        return new RustInPlaceTransformerGenerator(model, this.registry).generate();
    }
}

class RustInPlaceTransformerGenerator extends Visitor {
    output = new IndentingOutputStream();

    constructor(
        public model: Model.Model,
        public registry: Registry
    ) {
        super();
    }

    generate(): string {
        this.output.writeLine('use super::model::*;');
        this.output.writeLine();

        this.output.writeLine(`pub trait InPlaceTransformer {`);

        this.model.definitions.forEach((definition) => {
            this.visitDefinition(definition);
        });

        forEachForeignEntity(this.model, (namespace, entity) => {
            const module = `${'super::'.repeat(namespace.length)}${snakeCase(namespace.join('::'))}`;
            this.output.writeLine('#[allow(unused_mut)]');
            this.output.writeLine(
                `fn transform_${module}_${snakeCase(entity)}(self: &mut Self, mut node: ${module}::${pascalCase(entity)}) -> ${module}::${pascalCase(entity)} {`
            );
            this.output.writeLine('node');
            this.output.writeLine('}');
            this.output.writeLine();
        });

        this.output.writeLine('}');

        return this.output.toString().trim();
    }

    valueName: string = '';
    definitionName: string = '';

    visitDefinition(definition: Model.Definition): void {
        this.definitionName = definition.name;
        this.valueName = 'node';

        let typeName: string = pascalCase(definition.name);
        if (Model.isProductType(definition.type)) {
            typeName = `Box<${typeName}>`;
        }
        this.output.writeLine('#[allow(unused_mut)]');
        this.output.writeLine(
            `fn transform_${snakeCase(definition.name)}(self: &mut Self, mut node: ${typeName}) -> ${typeName} {`
        );
        super.visitDefinition(definition);
        this.output.writeLine('}');
        this.output.writeLine();
    }

    visitSumType(sumType: Model.SumType): void {
        this.output.writeLine(`match ${this.valueName} {`);
        sumType.members.forEach((member) => {
            // This only works for simple sum types of named type references
            if (Model.isNamedTypeReference(member)) {
                const name = member.fqn[member.fqn.length - 1];
                const valueName = 'value';
                this.output.write(`${pascalCase(this.definitionName)}::${pascalCase(name)}(value) =>`);
                this.output.writeLine(
                    `${pascalCase(this.definitionName)}::${pascalCase(name)}(self.transform_${snakeCase(name)}(${valueName})),`
                );
            }
        });
        this.output.writeLine(`}`);
    }

    visitProductType(node: Model.ProductType): void {
        for (const productMember of node.members) {
            if (this.requiresTransform(productMember.type)) {
                const oldValueName = this.valueName;
                let newValueName: string = snakeCase(productMember.name);
                if (Model.isSequenceType(productMember.type)) {
                    newValueName = pluralize(newValueName);
                }
                this.valueName = `${this.valueName}.${rustName(newValueName)}`;
                this.output.write(`${this.valueName} = `);
                this.visitType(productMember.type);
                this.output.writeLine(`;`);
                this.valueName = oldValueName;
            }
        }
        this.output.writeLine('node');
    }

    visitPrimitiveType(node: Model.PrimitiveType): void {
        this.output.writeLine(this.valueName);
    }

    visitEnumType(node: Model.EnumType): void {
        this.output.writeLine(this.valueName);
    }

    visitOptionType(optionType: Model.OptionType): void {
        this.output.writeLine(`${this.valueName}.map(|value|`);
        let oldValueName = this.valueName;
        this.valueName = 'value';
        this.visitType(optionType.type);
        this.valueName = oldValueName;
        this.output.writeLine(`)`);
    }

    visitSequenceType(sequenceType: Model.SequenceType): void {
        this.output.writeLine(`${this.valueName}.into_iter().map(|value|`);
        let oldValueName = this.valueName;
        this.valueName = 'value';
        this.visitType(sequenceType.elementType);
        this.valueName = oldValueName;
        this.output.writeLine(').collect()');
    }

    visitSetType(setType: Model.SetType): void {
        this.output.writeLine(`${this.valueName}.into_iter().map(|key|`);
        let oldValueName = this.valueName;
        this.valueName = 'key';
        this.visitType(setType.keyType);
        this.valueName = oldValueName;
        this.output.writeLine(').collect()');
    }

    visitMapType(mapType: Model.MapType): void {
        this.output.writeLine(`${pluralize(this.valueName)}.into_iter().map(|(key, value)|`);
        this.output.write('(');

        let oldValueName = this.valueName;
        this.valueName = 'key';
        this.visitType(mapType.keyType);
        this.output.write(',');
        this.valueName = 'value';
        this.visitType(mapType.valueType);
        this.valueName = oldValueName;
        this.output.write(')');
        this.output.writeLine(').collect()');
    }

    visitNamedTypeReference(namedTypeReference: Model.NamedTypeReference): void {
        this.output.writeLine(`self.transform_${snakeCase(namedTypeReference.fqn.join('_'))}(${this.valueName})`);
    }

    requiresTransform(type: Model.Type): boolean {
        switch (type.discriminator) {
            case Model.Discriminator.ProductType:
                return true;
            case Model.Discriminator.SumType:
                return type.members.some((m) => this.requiresTransform(m));
            case Model.Discriminator.MapType:
                return this.requiresTransform(type.keyType) || this.requiresTransform(type.valueType);
            case Model.Discriminator.SetType:
                return this.requiresTransform(type.keyType);
            case Model.Discriminator.SequenceType:
                return this.requiresTransform(type.elementType);
            case Model.Discriminator.OptionType:
                return this.requiresTransform(type.type);
            case Model.Discriminator.NamedTypeReference:
                return true;
            case Model.Discriminator.EnumType:
            case Model.Discriminator.PrimitiveType:
                return false;
            default:
                throw new Error(`Unexpected type: ${type.discriminator}`);
        }
    }
}
