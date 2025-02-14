import { EnumType, NamedTypeReference, SumType, Type } from '../../model/parsed/model';
import { ChoiceRule, CountedRuleElement, RuleReference, SequenceRule, StringElement } from './model';

export function choiceRuleAsEnumType(choiceRule: ChoiceRule): EnumType | undefined {
    const enumMembers = choiceRule.choices.map(toEnumMember);
    if (enumMembers.every((name) => name !== undefined)) {
        return new EnumType({ members: Array.from(new Set(enumMembers)) });
    }
    return undefined;
}

export function toEnumMember(rule: SequenceRule): string | undefined {
    if (rule.elements.length !== 1) return undefined;
    let element = rule.elements[0];
    if (!(element instanceof CountedRuleElement)) return undefined;
    let cre = element.countableRuleElement;
    if (!(cre instanceof StringElement)) return undefined;
    if (element.label) return element.label;
    const name = cre.value.replace(/[^a-zA-Z0-9]/g, '');
    return name.length > 0 ? name : undefined;
}

export function choiceRuleAsSimpleSumType(alternativeRules: ChoiceRule): SumType | undefined {
    const sumElements = alternativeRules.choices.map(toSimpleSumElement);
    if (sumElements.every((ty) => ty !== undefined)) {
        return new SumType({ members: sumElements });
    }
    return undefined;
}

export function toSimpleSumElement(rule: SequenceRule): Type | undefined {
    const elements = rule.elements.filter(
        (e) =>
            !(
                e instanceof CountedRuleElement &&
                e.label == undefined &&
                e.countableRuleElement instanceof StringElement
            )
    );
    if (elements.length !== 1) return undefined;
    let element = elements[0];
    if (!(element instanceof CountedRuleElement)) return undefined;
    let cre = element.countableRuleElement;
    if (!(cre instanceof RuleReference)) return undefined;
    return new NamedTypeReference({ fqn: [cre.names[cre.names.length - 1]] });
}

export class Counter {
    constructor(public count: number = 0) {}

    public next(): number {
        return this.count++;
    }
}
