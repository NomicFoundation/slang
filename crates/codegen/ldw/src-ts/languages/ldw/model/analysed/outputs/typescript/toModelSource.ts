import { camelCase, pascalCase } from 'literal-case';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { typeAsTypescript } from './util';
import { forEachForeignNamespace } from '../util';

export class AnalysedModelToTypescriptModelSource {
    private output: IndentingOutputStream;

    constructor(public registry: Registry) {
        this.output = new IndentingOutputStream();
    }

    transform(model: Model.Model): string {
        this.importForeignReferences(model);
        this.generateDiscriminatorEnum(model);

        model.definitions.forEach((definition) => {
            switch (definition.type.discriminator) {
                case Model.Discriminator.SumType:
                    this.generateSumDefinition(definition, definition.type);
                    break;
                case Model.Discriminator.ProductType:
                    this.generateProductDefinition(definition, definition.type);
                    break;
                case Model.Discriminator.EnumType:
                    this.generateEnumDefinition(definition, definition.type);
                    break;
                default:
                    this.output.write(`export type ${pascalCase(definition.name)} = `);
                    this.output.write(typeAsTypescript(definition.type));
                    this.output.writeLine(';');
                    break;
            }
            this.output.writeLine();
        });

        return this.output.toString().trim();
    }

    importForeignReferences(model: Model.Model) {
        forEachForeignNamespace(model, this.registry, (namespace) => {
            this.output.writeLine(
                `import * as ${pascalCase(namespace.join('_'))} from '${'../'.repeat(model.name.length)}/${namespace.join('/')}/model';`
            );
        });
        this.output.writeLine();
    }

    generateDiscriminatorEnum(model: Model.Model) {
        this.output.writeLine('export enum Discriminator {');
        this.output.indentDuring(() => {
            model.definitions.forEach((definition) => {
                if (definition.discriminationPeers) {
                    switch (definition.type.discriminator) {
                        case Model.Discriminator.ProductType:
                        case Model.Discriminator.EnumType:
                            this.output.writeLine(`${pascalCase(definition.name)} = '${pascalCase(definition.name)}',`);
                            break;
                        default:
                            break;
                    }
                }
            });
        });
        this.output.writeLine('}');
        this.output.writeLine();
    }

    generateSumDefinition(definition: Model.Definition, sumType: Model.SumType) {
        this.output.write(`export type ${pascalCase(definition.name)} = `);
        this.output.write(typeAsTypescript(sumType));
        this.output.writeLine(';');

        this.generateDiscriminatorFunction(definition);
    }

    generateProductDefinition(definition: Model.Definition, productType: Model.ProductType) {
        this.output.writeLine(`export class ${pascalCase(definition.name)} {`);
        this.output.indentDuring(() => {
            this.generateDiscriminatorField(definition);

            productType.members.forEach((member) => {
                this.output.write(`public ${camelCase(member.name)}: `);
                this.output.write(typeAsTypescript(member.type));
                this.output.writeLine(';');
            });
            this.output.writeLine();
            if (productType.members.length > 0) {
                this.output.writeLine('constructor(init: {');
                this.output.indentDuring(() => {
                    this.output.join(productType.members, ',\n', (member) => {
                        const opt = member.type.discriminator === Model.Discriminator.OptionType ? '?' : '';
                        this.output.write(`${camelCase(member.name)}${opt}: `);
                        this.output.write(typeAsTypescript(member.type));
                    });
                    this.output.writeLine();
                });
                this.output.writeLine('}) {');
                this.output.indentDuring(() => {
                    productType.members.forEach((member) => {
                        this.output.writeLine(`this.${camelCase(member.name)} = init.${camelCase(member.name)};`);
                    });
                });
                this.output.writeLine('}');
            }
        });
        this.output.writeLine('}');

        this.generateDiscriminatorFunction(definition);
    }

    generateEnumDefinition(definition: Model.Definition, enumType: Model.EnumType) {
        this.output.writeLine(`export enum ${pascalCase(definition.name)}Enum {`);
        this.output.indentDuring(() => {
            this.output.join(enumType.members, ',\n', (member, index) => {
                this.output.write(`${pascalCase(member)} = '${pascalCase(member)}'`);
            });
            this.output.writeLine();
        });
        this.output.writeLine('}');

        this.output.writeLine(`export class ${pascalCase(definition.name)} {`);
        this.output.indentDuring(() => {
            this.generateDiscriminatorField(definition);

            enumType.members.forEach((member) => {
                this.output.writeLine(
                    `static ${pascalCase(member)}: ${pascalCase(definition.name)} = new ${pascalCase(definition.name)}(${pascalCase(definition.name)}Enum.${pascalCase(member)});`
                );
            });
            this.output.writeLine();
            this.output.writeLine(`private constructor(public readonly value: ${pascalCase(definition.name)}Enum) {}`);
        });
        this.output.writeLine('}');

        this.generateDiscriminatorFunction(definition);
    }

    generateDiscriminatorField(definition: Model.Definition) {
        if (definition.discriminationPeers) {
            this.output.writeLine(`readonly discriminator = Discriminator.${pascalCase(definition.name)};`);
            this.output.writeLine();
        }
    }

    generateDiscriminatorFunction(definition: Model.Definition) {
        if (definition.discriminationPeers) {
            const values = [...definition.discriminationPeers.values()];
            const valueType = values.map((value) => pascalCase(value)).join(' | ');
            this.output.writeLine(
                `export function is${pascalCase(definition.name)}(value: ${valueType} ): value is ${pascalCase(
                    definition.name
                )} {`
            );

            if (definition.discriminationMembers) {
                this.output.indentDuring(() => {
                    this.output.writeLine('switch (value.discriminator) {');
                    this.output.indentDuring(() => {
                        this.output.indentDuring(() => {
                            definition.discriminationMembers!.forEach((member) => {
                                this.output.writeLine(`case Discriminator.${pascalCase(member)}:`);
                            });
                            this.output.writeLine('return true;');
                        });
                        this.output.writeLine('default:');
                        this.output.indentDuring(() => {
                            this.output.writeLine('return false;');
                        });
                    });
                    this.output.writeLine('}');
                });
            } else {
                this.output.indentDuring(() => {
                    this.output.writeLine(
                        `return value.discriminator === Discriminator.${pascalCase(definition.name)};`
                    );
                });
            }
            this.output.writeLine('}');
        }
    }
}
