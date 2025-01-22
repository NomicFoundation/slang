import { pascalCase, snakeCase } from 'literal-case';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { Visitor } from '../../visitor';
import { forEachForeignEntity } from '../util';

export abstract class RustGenericHandlerGenerator extends Visitor {
    output = new IndentingOutputStream();

    constructor(
        public model: Model.Model,
        public registry: Registry
    ) {
        super();
    }

    abstract traitName(): string;

    generate(): string {
        this.output.writeLine('use super::model::*;');
        this.output.writeLine();

        this.output.writeLine(`pub trait ${this.traitName()} {`);

        this.model.definitions.forEach((definition) => {
            this.visitDefinition(definition);
        });

        forEachForeignEntity(this.model, (namespace, entity) => {
            const module = `${'super::'.repeat(namespace.length)}${snakeCase(namespace.join('::'))}`;
            this.output.writeLine('#[allow(unused_variables)]');
            this.output.writeLine(
                `fn handle_${module}_${snakeCase(entity)}(self: &mut Self, node: &${module}::${pascalCase(entity)}) {`
            );
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
        this.output.writeLine('#[allow(unused_variables)]');
        this.output.writeLine(`fn handle_${snakeCase(definition.name)}(self: &mut Self, node: &${typeName}) {`);
        super.visitDefinition(definition);
        this.output.writeLine('}');
        this.output.writeLine();
    }

    visitSumType(sumType: Model.SumType): void {
        this.output.writeLine(`match ${this.valueName} {`);
        sumType.members.forEach((member) => {
            if (Model.isNamedTypeReference(member)) {
                const name = member.fqn[member.fqn.length - 1];
                this.output.writeLine(
                    `${pascalCase(this.definitionName)}::${pascalCase(name)}(value) => self.handle_${snakeCase(name)}(value),`
                );
            }
        });
        this.output.writeLine(`}`);
    }
}
