import assert from 'assert';
import { pascalCase } from 'literal-case';
import { singular } from 'pluralize';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { typeAsTypescript, typeAsTypescriptDescription } from './util';
import { forEachForeignNamespace } from '../util';

export class AnalysedModelToTypescriptBuilderSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        const output = new IndentingOutputStream();
        new Generator(model, this.registry, output).generate();
        return output.toString().trim();
    }
}

class Generator {
    constructor(
        public model: Model.Model,
        public registry: Registry,
        public output: IndentingOutputStream
    ) {}

    generate() {
        this.output.writeLine("import assert from 'assert';");
        this.output.writeLine('import * as Model from "./model";');
        forEachForeignNamespace(this.model, this.registry, (namespace) => {
            this.output.writeLine(
                `import * as ${pascalCase(namespace.join('_'))} from '${'../'.repeat(this.model.name.length)}/${namespace.join('/')}/model';`
            );
        });
        this.output.writeLine();
        this.output.writeLine();

        this.generateBuildableTypes();
        this.generateFieldEnums();
        this.generateBuilder();
    }

    forEachBuildable(callback: (definition: Model.Definition) => void): void {
        this.model.definitions.forEach((definition) => {
            if (definition.type.discriminator === Model.Discriminator.ProductType) {
                callback(definition);
            }
        });
    }

    forEachBuildableField(definition: Model.Definition, callback: (fieldName: string, type: Model.Type) => void): void {
        assert(definition.type.discriminator === Model.Discriminator.ProductType);
        definition.type.members.forEach((member) => {
            if (Model.isSequenceType(member.type)) {
                callback(singular(member.name), member.type.elementType);
            }
            callback(member.name, member.type);
        });
    }

    forEachBuildableFieldName(callback: (fieldName: string) => void): void {
        const names = new Set<string>();
        this.forEachBuildable((definition) => {
            this.forEachBuildableField(definition, (fieldName, type) => {
                if (!names.has(fieldName)) {
                    names.add(fieldName);
                    callback(fieldName);
                }
            });
        });
    }

    forEachBuildableFieldType(callback: (type: Model.Type, description: string) => void): void {
        const descriptions = new Set<string>();
        this.forEachBuildable((definition) => {
            this.forEachBuildableField(definition, (fieldName, inputType) => {
                let type = inputType;
                if (Model.isSequenceType(type)) {
                    type = type.elementType;
                }
                if (Model.isOptionType(type)) {
                    type = type.type;
                }
                switch (type.discriminator) {
                    case Model.Discriminator.SumType:
                        for (const member of type.members) {
                            if (Model.isNamedTypeReference(member)) {
                                const description = `${pascalCase(member.fqn.join('_'))}`;
                                if (!descriptions.has(description)) {
                                    descriptions.add(description);
                                    callback(member, description);
                                }
                            } else {
                                let description = typeAsTypescriptDescription(type);
                                if (!descriptions.has(description)) {
                                    descriptions.add(description);
                                    callback(type, description);
                                }
                            }
                        }
                        break;
                    case Model.Discriminator.ProductType:
                    case Model.Discriminator.EnumType:
                        {
                            const description = `${pascalCase(definition.name)}${pascalCase(singular(fieldName))}`;
                            if (!descriptions.has(description)) {
                                descriptions.add(description);
                                callback(type, description);
                            }
                        }
                        break;
                    case Model.Discriminator.OptionType:
                        throw new Error('Option field should have been unwrapped');
                    case Model.Discriminator.SequenceType:
                        throw new Error('Sequence field should have been unwrapped');
                    default:
                        let description = typeAsTypescriptDescription(type);
                        if (!descriptions.has(description)) {
                            descriptions.add(description);
                            callback(type, description);
                        }
                        break;
                }
            });
        });
    }

    generateBuildableTypes() {
        this.output.writeLine('export type Buildable =');
        this.forEachBuildable((definition) => {
            this.output.writeLine(`| Model.${pascalCase(definition.name)}`);
        });
        this.output.writeLine(';');
        this.output.writeLine();

        this.output.writeLine('export type BuildableType =');
        this.forEachBuildable((definition) => {
            this.output.writeLine(`| typeof Model.${pascalCase(definition.name)}`);
        });
        this.output.writeLine(';');
        this.output.writeLine();
    }

    generateFieldEnums() {
        this.output.writeLine(`export enum Field {`);
        this.forEachBuildableFieldName((fieldName) => {
            this.output.writeLine(`${pascalCase(fieldName)},`);
        });
        this.output.writeLine('}');
        this.output.writeLine();
    }

    generateBuilder() {
        this.output.writeLine('export class Builder {');
        this.output.writeLine();

        this.generateMarkAndRestoreMethods();
        this.generateStartMethod();
        this.generateFinaliseMethod();
        this.generateBuildMethod();
        this.generateSetMethods();

        this.output.writeLine('instructions: Instruction[] = [];');
        this.output.writeLine('types: BuildableType[] = [];');
        this.output.writeLine();

        this.output.writeLine('}');
        this.output.writeLine();

        this.generateMarkType();
        this.generateInstructionTypes();
    }

    private generateMarkAndRestoreMethods() {
        this.output.writeLine('mark(): Mark {');
        this.output.writeLine('return {');
        this.output.writeLine('instructionsLength: this.instructions.length,');
        this.output.writeLine('typesLength: this.types.length');
        this.output.writeLine('};');
        this.output.writeLine('}');
        this.output.writeLine();

        this.output.writeLine('restore(mark: Mark) {');
        this.output.writeLine('assert(mark.instructionsLength <= this.instructions.length);');
        this.output.writeLine(
            'this.instructions.splice(mark.instructionsLength, this.instructions.length-mark.instructionsLength);'
        );
        this.output.writeLine('this.types.splice(mark.typesLength, this.types.length-mark.typesLength);');
        this.output.writeLine('}');
        this.output.writeLine();
    }

    private generateStartMethod() {
        this.output.writeLine('start(type: BuildableType): void {');
        this.output.writeLine('this.types.push(type);');
        this.output.writeLine('this.instructions.push({');
        this.output.writeLine('type: "start",');
        this.output.writeLine('buildable: type');
        this.output.writeLine('});');
        this.output.writeLine('}');
        this.output.writeLine();
    }

    private generateFinaliseMethod() {
        this.output.writeLine(`finalise(): Buildable {`);
        // TODO: this is the meat of the builder
        this.output.writeLine(`throw new Error("Not implemented");`);
        this.output.writeLine(`}`);
        this.output.writeLine();
    }

    private generateBuildMethod() {
        this.output.writeLine('build(field: Field): void {');
        // TODO: validate type of field
        this.output.writeLine('this.instructions.push({');
        this.output.writeLine('type: "build",');
        this.output.writeLine('field: field');
        this.output.writeLine('});');
        this.output.writeLine('this.types.pop();');
        this.output.writeLine('}');
        this.output.writeLine();
    }

    private generateSetMethods() {
        this.output.writeLine('setUndefined(field: Field): void {');
        this.output.writeLine('this.instructions.push({');
        this.output.writeLine('type: "set-undefined",');
        this.output.writeLine('field: field');
        this.output.writeLine('});');
        this.output.writeLine('}');
        this.output.writeLine();

        this.forEachBuildableFieldType((type, description) => {
            if (Model.isOptionType(type)) {
                // Ignore options
            } else if (Model.isSequenceType(type)) {
                // Ignore arrays
            } else {
                this.output.write(`set${description}`);
                this.output.writeLine(`(field: Field, value: ${typeAsTypescript(type, 'Model.')}): void {`);
                // TODO: validate type of field
                this.output.writeLine('this.instructions.push({');
                this.output.writeLine(`type: "set-${description}",`);
                this.output.writeLine('field: field,');
                this.output.writeLine('value: value');
                this.output.writeLine('});');
                this.output.writeLine('}');
                this.output.writeLine();
            }
        });
    }

    generateMarkType() {
        this.output.writeLine('export interface Mark {');
        this.output.writeLine('instructionsLength: number;');
        this.output.writeLine('typesLength: number;');
        this.output.writeLine('};');
        this.output.writeLine();
    }

    generateInstructionTypes() {
        this.output.writeLine('type Instruction = StartInstruction | BuildInstruction | SetInstruction;');
        this.output.writeLine();
        this.generateStartInstructions();
        this.generateBuildInstructions();
        this.generateSetInstructions();
    }

    generateStartInstructions() {
        this.output.writeLine('interface StartInstruction {');
        this.output.writeLine('type: "start";');
        this.output.writeLine('buildable: BuildableType;');
        this.output.writeLine('}');
        this.output.writeLine();
    }

    generateBuildInstructions() {
        this.output.writeLine('interface BuildInstruction {');
        this.output.writeLine('type: "build";');
        this.output.writeLine('field: Field;');
        this.output.writeLine('}');
        this.output.writeLine();
    }

    generateSetInstructions() {
        let needUndefinedType = false;
        this.output.writeLine('type SetInstruction =');
        this.output.writeLine('SetUndefinedInstruction');
        this.forEachBuildableFieldType((type, description) => {
            if (Model.isOptionType(type)) {
                // Ignore arrays
            } else if (Model.isSequenceType(type)) {
                // Ignore arrays
            } else {
                this.output.writeLine(`| Set${description}Instruction`);
            }
        });
        this.output.writeLine();

        this.output.writeLine(`interface SetUndefinedInstruction {`);
        this.output.writeLine(`type: "set-undefined";`);
        this.output.writeLine('field: Field;');
        this.output.writeLine('}');
        this.output.writeLine();

        this.forEachBuildableFieldType((type, description) => {
            if (Model.isOptionType(type)) {
                // Ignore options
            } else if (Model.isSequenceType(type)) {
                // Ignore arrays
            } else {
                this.output.writeLine(`interface Set${description}Instruction {`);
                this.output.writeLine(`type: "set-${description}";`);
                this.output.writeLine('field: Field;');
                this.output.writeLine(`value: ${typeAsTypescript(type, 'Model.')};`);
                this.output.writeLine('}');
                this.output.writeLine();
            }
        });
    }
}
