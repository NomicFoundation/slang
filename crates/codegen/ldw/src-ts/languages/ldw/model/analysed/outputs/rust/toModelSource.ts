import { pascalCase, snakeCase } from 'literal-case';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { rustName, typeAsRust, typeAsRustDescription } from './util';

export class AnalysedModelToRustModelSource {
    private output: IndentingOutputStream;
    model: Model.Model | undefined;

    constructor(public registry: Registry) {
        this.output = new IndentingOutputStream();
    }

    transform(model: Model.Model): string {
        this.model = model;
        model.definitions.forEach((definition) => {
            switch (definition.type.discriminator) {
                case Model.Discriminator.SumType:
                    this.generateSumTypeDefinition(definition, definition.type);
                    break;
                case Model.Discriminator.ProductType:
                    this.generateProductTypeDefinition(definition, definition.type);
                    break;
                case Model.Discriminator.EnumType:
                    this.generateEnumTypeDefinition(definition, definition.type);
                    break;
                default:
                    this.generateAliasDefinition(definition);
                    break;
            }
            this.output.writeLine();
        });

        return this.output.toString().trim();
    }

    generateAliasDefinition(definition: Model.Definition) {
        this.output.write(`pub type ${rustName(pascalCase(definition.name))} = `);
        this.output.write(typeAsRust(this.model!, definition.type));
        this.output.writeLine(';');
    }

    generateSumTypeDefinition(definition: Model.Definition, sumType: Model.SumType) {
        this.output.write(`pub enum ${pascalCase(definition.name)} {`);
        sumType.members.forEach((member) => {
            this.output.writeLine(`${typeAsRustDescription(member)}(${typeAsRust(this.model!, member)}),`);
        });
        this.output.writeLine('}');
    }

    generateProductTypeDefinition(definition: Model.Definition, productType: Model.ProductType) {
        this.output.writeLine(`pub type ${pascalCase(definition.name)}Ref = Box<${pascalCase(definition.name)}>;`);
        this.output.writeLine(`pub struct ${pascalCase(definition.name)} {`);
        productType.members.forEach((member) => {
            this.output.write(`pub ${rustName(snakeCase(member.name))}: `);
            this.output.write(typeAsRust(this.model!, member.type));
            this.output.writeLine(',');
        });
        this.output.writeLine('}');
    }

    generateEnumTypeDefinition(definition: Model.Definition, enumType: Model.EnumType) {
        this.output.writeLine(`pub enum ${pascalCase(definition.name)} {`);
        this.output.join(enumType.members, ',\n', (member) => {
            this.output.write(`${pascalCase(member)}`);
        });
        this.output.writeLine();
        this.output.writeLine('}');
    }
}
