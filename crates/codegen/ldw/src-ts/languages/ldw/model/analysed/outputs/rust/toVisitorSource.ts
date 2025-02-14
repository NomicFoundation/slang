import { snakeCase } from 'literal-case';
import pluralize from 'pluralize';
import { Registry } from '../../../../../../nanopass/registry';
import * as Model from '../../model';
import { RustGenericHandlerGenerator } from './genericHandlerGenerator';
import { rustName } from './util';

export class AnalysedModelToRustVisitorSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        return new RustVisitorGenerator(model, this.registry).generate();
    }
}

class RustVisitorGenerator extends RustGenericHandlerGenerator {
    traitName(): string {
        return 'Visitor';
    }

    visitType(node: Model.Type): void {
        if (this.isVisitable(node)) {
            super.visitType(node);
        }
    }

    visitProductMember(productMember: Model.ProductMember): void {
        const oldValueName = this.valueName;
        let newValueName: string = snakeCase(productMember.name);
        if (Model.isSequenceType(productMember.type)) {
            newValueName = pluralize(newValueName);
        }
        this.valueName = `${this.valueName}.${rustName(newValueName)}`;
        this.visitType(productMember.type);
        this.valueName = oldValueName;
    }

    visitOptionType(optionType: Model.OptionType): void {
        this.output.writeLine(`if let Some(value) = &${this.valueName} {`);
        let oldValueName = this.valueName;
        this.valueName = 'value';
        this.visitType(optionType.type);
        this.valueName = oldValueName;
        this.output.writeLine(`}`);
    }

    visitSequenceType(sequenceType: Model.SequenceType): void {
        this.output.writeLine(`for value in &${this.valueName} {`);
        let oldValueName = this.valueName;
        this.valueName = 'value';
        this.visitType(sequenceType.elementType);
        this.valueName = oldValueName;
        this.output.writeLine(`}`);
    }

    visitSetType(setType: Model.SetType): void {
        this.output.writeLine(`for key in &${pluralize(this.valueName)} {`);
        let oldValueName = this.valueName;
        this.valueName = 'value';
        this.visitType(setType.keyType);
        this.valueName = oldValueName;
        this.output.writeLine(`}`);
    }

    visitMapType(mapType: Model.MapType): void {
        const keyVar = this.isVisitable(mapType.keyType) ? 'key' : '_';
        const valueVar = this.isVisitable(mapType.valueType) ? 'value' : '_';
        this.output.writeLine(`for (${keyVar}, ${valueVar}) in &${pluralize(this.valueName)} {`);
        let oldValueName = this.valueName;
        this.valueName = 'key';
        this.visitType(mapType.keyType);
        this.valueName = 'value';
        this.visitType(mapType.valueType);
        this.valueName = oldValueName;
        this.output.writeLine(`}`);
    }

    visitNamedTypeReference(namedTypeReference: Model.NamedTypeReference): void {
        this.output.writeLine(`self.handle_${snakeCase(namedTypeReference.fqn.join('_'))}(&${this.valueName});`);
    }

    isVisitable(type: Model.Type): boolean {
        switch (type.discriminator) {
            case Model.Discriminator.ProductType:
                return true;
            case Model.Discriminator.SumType:
                return type.members.some((m) => this.isVisitable(m));
            case Model.Discriminator.MapType:
                return this.isVisitable(type.keyType) || this.isVisitable(type.valueType);
            case Model.Discriminator.SetType:
                return this.isVisitable(type.keyType);
            case Model.Discriminator.SequenceType:
                return this.isVisitable(type.elementType);
            case Model.Discriminator.OptionType:
                return this.isVisitable(type.type);
            case Model.Discriminator.NamedTypeReference: {
                return true;
            }
            case Model.Discriminator.EnumType:
            case Model.Discriminator.PrimitiveType:
                return false;
            default:
                throw new Error(`Unexpected type: ${type.discriminator}`);
        }
    }
}
