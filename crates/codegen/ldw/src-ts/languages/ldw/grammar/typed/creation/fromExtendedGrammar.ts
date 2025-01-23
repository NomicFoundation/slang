import pluralize from 'pluralize';
import * as In from '../../extended/model';
import {
    EnumType,
    NamedTypeReference,
    OptionType,
    PrimitiveType,
    ProductMember,
    ProductType,
    SequenceType,
    SumType
} from '../../../model/parsed/model';
import { baseType, typesAreEqual } from '../../../model/parsed/util';
import * as Out from '../model';
import { Visitor as ModelVisitor } from '../../../model/parsed/visitor';
import { Transformer } from '../transformer';
import { Visitor } from '../visitor';

export class TypedGrammarFromExtendedGrammar extends Transformer {
    transform(input: In.Grammar): Out.Grammar {
        return new TransformToTypedGrammar().transformGrammar(input);
    }
}

class TransformToTypedGrammar extends Transformer {
    transformGrammar(input: In.Grammar): Out.Grammar {
        const grammar = super.transformGrammar(input);

        // Any rule whose type is a ProductType with a single field, that is not referenced
        // from a SumType, can have its type Hoisted

        const sumMembers = new Set<string>();
        const visitor = new (class extends ModelVisitor {
            visitSumType(type: SumType) {
                for (const member of type.members) {
                    if (member instanceof NamedTypeReference) {
                        sumMembers.add(member.fqn[member.fqn.length - 1]);
                    }
                }
                super.visitSumType(type);
            }
        })();
        for (const rule of grammar.rules) {
            visitor.visitType(rule.type);
        }

        for (const rule of grammar.rules) {
            if (!sumMembers.has(rule.name) && rule.type instanceof ProductType && rule.type.members.length === 1) {
                rule.type = rule.type.members[0].type;
            }
        }

        return grammar;
    }

    transformRule(input: In.Rule): Out.Rule {
        if (input.annotation === In.RuleAnnotation.Atomic) {
            // Default Transformer will just map the types
            const body = new Transformer().transformRuleBody(input.body);
            return new Out.Rule({
                name: input.name,
                body: body,
                annotation: input.annotation && this.transformRuleAnnotation(input.annotation),
                versionAnnotations: input.versionAnnotations.map((x) => this.transformVersionAnnotation(x)),
                type: new ProductType({ members: [new ProductMember({ name: 'value', type: PrimitiveType.String })] })
            });
        }

        const body = this.transformRuleBody(input.body);

        // Propagate multiplicity down through the tree
        const counts: Out.Count[] = [];
        new (class extends Visitor {
            visitField(field: Out.Field) {
                if (field.type) {
                    for (let i = counts.length - 1; i >= 0; i--) {
                        if (counts[i] === Out.Count.Optional) {
                            if (
                                field.type === PrimitiveType.Boolean ||
                                field.type instanceof SequenceType ||
                                field.type instanceof OptionType
                            ) {
                                // No change
                            } else {
                                field.type = new OptionType({ type: field.type });
                            }
                        } else {
                            field.type = new SequenceType({ elementType: field.type });
                        }
                    }
                }
            }
            visitSeparatedByRule(rule: Out.SeparatedByRule) {
                counts.push(Out.Count.ZeroOrMore);
                super.visitSeparatedByRule(rule);
                counts.pop();
            }
            visitCountedRuleElement(element: Out.CountedRuleElement) {
                if (element.count) counts.push(element.count);
                super.visitCountedRuleElement(element);
                if (element.count) counts.pop();
            }
        })().visitRuleBody(body);

        const fields: Out.Field[] = [];
        new CollectFields(fields).visitRuleBody(body);

        /*
            Merge fields that have the same name.

            if they have equal base types
                replace the type with a SequenceType of that baseType
            else 
               replace the type with a SumType of all the types.

            // TODO: (maybe) if they are all OptionTypes, make a SequenceType of the SumType of the base types

            Do the merging by changing the types of all those fields that have
            the same name, so that all other references to the Fields see the
            change
        */

        let fieldsGroupedByName = fields.reduce(
            (acc, f) => {
                (acc[f.name ?? ''] = acc[f.name ?? ''] || []).push(f);
                return acc;
            },
            {} as Record<string, Out.Field[]>
        );
        const duplicatedFields = Object.values(fieldsGroupedByName).filter((v) => v.length > 1);

        for (const fields of duplicatedFields) {
            const theBaseType = baseType(fields[0].type);
            let theNewType;
            if (fields.every((f) => typesAreEqual(baseType(f.type), theBaseType))) {
                theNewType = new SequenceType({ elementType: theBaseType });
            } else if (fields.every((f) => f instanceof SequenceType)) {
                theNewType = new SequenceType({ elementType: new SumType({ members: fields.map((f) => f.type) }) });
            } else {
                theNewType = new SumType({ members: fields.map((f) => f.type) });
            }
            for (const field of fields) {
                field.type = theNewType;
            }
        }

        let uniqueFields = Object.values(fieldsGroupedByName).map((fields) => fields[0]);

        let ruleType = undefined;

        if (uniqueFields.length === 1) {
            let field = uniqueFields[0];
            if (field.name === undefined || field.name === '_') {
                ruleType = field.type;
            } else if (!field.isExplicit && field.type instanceof SumType) {
                ruleType = field.type;
            }
        }

        if (ruleType === undefined) {
            ruleType = new ProductType({
                members: uniqueFields
                    .filter((f) => f.name != undefined)
                    .map((f) => {
                        const name = f.type instanceof SequenceType ? pluralize(f.name!) : f.name!;
                        return new ProductMember({ name, type: f.type });
                    })
            });
        }

        return new Out.Rule({
            name: input.name,
            body: body,
            annotation: input.annotation && this.transformRuleAnnotation(input.annotation),
            versionAnnotations: input.versionAnnotations.map((x) => this.transformVersionAnnotation(x)),
            type: ruleType
        });
    }

    explicitFieldName: string | undefined;
    forceFieldName: boolean = false;

    transformCountedRuleElement(input: In.CountedRuleElement): Out.CountedRuleElement {
        const oldExplicitFieldName = this.explicitFieldName;
        const oldForceFieldName = this.forceFieldName;

        if (input.label) {
            this.forceFieldName = true;
            this.explicitFieldName = input.label;
        }

        const result = super.transformCountedRuleElement(input);

        this.explicitFieldName = oldExplicitFieldName;
        this.forceFieldName = oldForceFieldName;

        return result;
    }

    transformRuleBody(input: In.RuleBody): Out.RuleBody {
        const oldForceFieldName = this.forceFieldName;
        this.forceFieldName = false;

        const result = super.transformRuleBody(input);

        this.forceFieldName = oldForceFieldName;

        return result;
    }

    transformEnumRule(input: In.EnumRule): Out.EnumRule {
        return new Out.EnumRule({
            members: input.members,
            field: this.forceFieldName
                ? new Out.Field({
                      type: new EnumType({ members: input.members }),
                      name: this.explicitFieldName,
                      isExplicit: true
                  })
                : new Out.Field({ type: new EnumType({ members: input.members }), name: undefined, isExplicit: false })
        });
    }

    transformCharSet(input: In.CharSet): Out.CharSet {
        return new Out.CharSet({
            negated: input.negated,
            startChars: input.startChars,
            endChars: input.endChars,
            field: this.forceFieldName
                ? new Out.Field({ type: PrimitiveType.String, name: this.explicitFieldName, isExplicit: true })
                : undefined
        });
    }

    transformRuleReference(input: In.RuleReference): Out.RuleReference {
        return new Out.RuleReference({
            names: input.names,
            field: new Out.Field({
                type: new NamedTypeReference({ fqn: input.names }),
                name: this.explicitFieldName ?? input.names[input.names.length - 1],
                isExplicit: this.explicitFieldName !== undefined
            })
        });
    }

    transformStringElement(input: In.StringElement): Out.StringElement {
        return new Out.StringElement({
            value: input.value,
            field: this.forceFieldName
                ? new Out.Field({ type: PrimitiveType.Boolean, name: this.explicitFieldName, isExplicit: true })
                : undefined
        });
    }

    transformAnyElement(input: In.AnyElement): Out.AnyElement {
        return new Out.AnyElement({
            field: this.forceFieldName
                ? new Out.Field({ type: PrimitiveType.String, name: this.explicitFieldName, isExplicit: true })
                : undefined
        });
    }
}

class CollectFields extends Visitor {
    constructor(public fields: Out.Field[] = []) {
        super();
    }

    visitChoiceRule(rule: Out.ChoiceRule) {
        const choicesFields = rule.choices.map((choice) => {
            const collector = new CollectFields();
            collector.visitSequenceRule(choice);
            return collector.fields;
        });

        const firstField = choicesFields[0][0];
        if (
            choicesFields.every(
                (fields) =>
                    fields.length === 1 && fields[0].name === firstField.name && fields[0].type instanceof SequenceType
            )
        ) {
            // convert F = seq<A> | F = seq<B> to F = seq<A | B>
            const newType = new SequenceType({
                elementType: new SumType({ members: choicesFields.map((fields) => baseType(fields[0].type)) })
            });
            for (const fields of choicesFields) {
                fields[0].type = newType;
            }
            this.fields.push(firstField);
        } else {
            for (const field of choicesFields.flat()) {
                this.fields.push(field);
            }
        }
    }

    visitField(field: Out.Field) {
        this.fields.push(field);
    }
}
