import { camelCase, pascalCase } from 'literal-case';
import pluralize from 'pluralize';
import { Registry } from '../../../../../../nanopass/registry';
import { IndentingOutputStream } from '../../../../../../output/indentingOutputStream';
import * as Model from '../../model';
import { forEachForeignEntity, forEachForeignNamespace } from '../util';
import { Visitor } from '../../visitor';

export class AnalysedModelToTypescriptVisitorSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        const generator = new TopLevelGenerator(this.registry);
        generator.visitModel(model);
        return generator.output.toString().trim();
    }
}

class TopLevelGenerator extends Visitor {
    definitions = new Map<string, Model.Definition>();
    visitableDefinitions = new Map<string, Model.Definition>();
    output = new IndentingOutputStream();

    constructor(public registry: Registry) {
        super();
    }

    collectVisitableDefinitions(model: Model.Model) {
        model.definitions.forEach((definition) => {
            if (definition.type instanceof Model.ProductType) {
                this.visitableDefinitions.set(definition.name, definition);
            } else if (definition.type instanceof Model.SumType) {
                this.visitableDefinitions.set(definition.name, definition);
            } else if (definition.type instanceof Model.EnumType) {
                this.visitableDefinitions.set(definition.name, definition);
            }
        });

        // Fixed point reachability
        let changes;
        do {
            changes = false;
            model.definitions.forEach((definition) => {
                if (definition.type instanceof Model.SequenceType || definition.type instanceof Model.OptionType) {
                    if (!this.visitableDefinitions.has(definition.name)) {
                        this.visitableDefinitions.set(definition.name, definition);
                        changes = true;
                    }
                }
            });
        } while (changes);
    }

    visitModel(model: Model.Model): void {
        model.definitions.forEach((definition) => {
            this.definitions.set(definition.name, definition);
        });
        this.collectVisitableDefinitions(model);

        this.output.writeLine('import * as Model from "./model";');
        forEachForeignNamespace(model, this.registry, (namespace) => {
            const module = pascalCase(namespace.join('_'));
            this.output.writeLine(
                `import * as ${module} from '${'../'.repeat(model.name.length)}/${namespace.join('/')}/model';`
            );
        });
        this.output.writeLine();

        this.output.writeLine('export class Visitor {');
        this.output.writeLine();
        this.output.indentDuring(() => {
            model.definitions.forEach((definition) => {
                if (this.visitableDefinitions.has(definition.name)) {
                    this.visitDefinition(definition);
                }
            });
            forEachForeignEntity(model, (namespace, entity) => {
                const module = pascalCase(namespace.join('_'));
                const type = pascalCase(entity);
                this.output.writeLine(`visit${module}${type}(node: ${module}.${type}): void {`);
                this.output.writeLine('}');
                this.output.writeLine();
            });
        });
        this.output.writeLine('}');
    }

    valueName: string = '';

    visitDefinition(definition: Model.Definition): void {
        this.output.writeLine(
            `visit${pascalCase(definition.name)}(node: Model.${pascalCase(definition.name)}): void {`
        );
        this.valueName = 'node';
        this.output.indentDuring(() => super.visitDefinition(definition));
        this.output.writeLine('}');
        this.output.writeLine();
    }

    visitSumType(sumType: Model.SumType): void {
        this.output.writeLine(`switch (${this.valueName}.discriminator) {`);
        this.output.indentDuring(() => {
            sumType.members.forEach((member) => {
                if (member instanceof Model.NamedTypeReference) {
                    const name = member.fqn[member.fqn.length - 1];
                    const definition = this.definitions.get(name)!;
                    if (definition.discriminationMembers) {
                        definition.discriminationMembers.forEach((member) => {
                            this.output.writeLine(`case Model.Discriminator.${pascalCase(member)}:`);
                        });
                    } else {
                        this.output.writeLine(`case Model.Discriminator.${pascalCase(name)}:`);
                    }
                    this.output.indentDuring(() => {
                        this.output.writeLine(`this.visit${pascalCase(name)}(${this.valueName});`);
                        this.output.writeLine(`break`);
                    });
                }
            });
        });
        this.output.writeLine(`}`);
    }

    visitProductMember(productMember: Model.ProductMember): void {
        const oldValueName = this.valueName;
        if (productMember.type.discriminator === Model.Discriminator.SequenceType) {
            this.valueName = `${this.valueName}.${pluralize(camelCase(productMember.name))}`;
        } else {
            this.valueName = `${this.valueName}.${camelCase(productMember.name)}`;
        }
        this.visitType(productMember.type);
        this.valueName = oldValueName;
    }

    visitOptionType(optionType: Model.OptionType): void | Model.OptionType {
        this.output.writeLine(`if (${this.valueName} != undefined) {`);
        this.visitType(optionType.type);
        this.output.writeLine(`}`);
    }

    visitSequenceType(sequenceType: Model.SequenceType): void | Model.SequenceType {
        this.output.writeLine(`${this.valueName}.forEach(x => {`);
        let oldValueName = this.valueName;
        this.valueName = 'x';
        this.output.indentDuring(() => {
            this.visitType(sequenceType.elementType);
        });
        this.valueName = oldValueName;
        this.output.writeLine(`})`);
    }

    visitMapType(mapType: Model.MapType): void | Model.MapType {
        if (
            mapType.valueType instanceof Model.NamedTypeReference &&
            this.visitableDefinitions.has(mapType.valueType.fqn[mapType.valueType.fqn.length - 1])
        ) {
            this.output.writeLine(`${pluralize(this.valueName)}.forEach(x => {`);
            let oldValueName = this.valueName;
            this.valueName = 'x';
            this.output.indentDuring(() => {
                this.visitType(mapType.valueType);
            });
            this.valueName = oldValueName;
            this.output.writeLine(`})`);
        }
    }

    visitNamedTypeReference(namedTypeReference: Model.NamedTypeReference): void | Model.NamedTypeReference {
        if (
            (namedTypeReference.fqn.length == 1 && this.visitableDefinitions.has(namedTypeReference.fqn[0])) ||
            namedTypeReference.fqn.length > 1
        ) {
            this.output.writeLine(`this.visit${pascalCase(namedTypeReference.fqn.join('_'))}(${this.valueName});`);
        }
    }
}
