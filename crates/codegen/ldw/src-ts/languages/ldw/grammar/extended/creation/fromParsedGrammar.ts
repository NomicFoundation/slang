import * as In from '../../parsed/model';
import * as Out from '../model';
import { Transformer } from '../transformer';

export class ExtendedGrammarFromParsedGrammar extends Transformer {
    transform(input: In.Grammar): Out.Grammar {
        return this.transformGrammar(input);
    }

    transformRule(input: In.Rule): Out.Rule {
        if (input.annotation === In.RuleAnnotation.Atomic) {
            // Default transformer will just map the types
            return new Transformer().transformRule(input);
        }
        return super.transformRule(input);
    }

    transformChoiceRule(input: In.ChoiceRule): Out.RuleBody {
        // Check for possible EnumRule replacements

        const enumMembers = input.choices.map((sequenceRule) => {
            if (sequenceRule.elements.length === 1) {
                const element = sequenceRule.elements[0];
                if (!(element instanceof In.CountedRuleElement)) return undefined;
                if (element.count) return undefined;
                const cre = element.countableRuleElement;
                if (!(cre instanceof In.StringElement)) return undefined;
                let name = element.label;
                let value = cre.value;
                if (!name) name = value.replace(/[^a-zA-Z0-9]/g, '');
                if (name.length === 0) return undefined;
                return name;
            }
            return undefined;
        });
        if (enumMembers.every((m) => m !== undefined)) {
            return new Out.EnumRule({ members: enumMembers });
        }

        return super.transformChoiceRule(input);
    }

    transformSequenceRule(input: In.SequenceRule): Out.SequenceRule {
        // Check for possible SeparatedByRule replacements

        const result = super.transformSequenceRule(input);

        for (let i = 1; i < result.elements.length; i++) {
            const firstReference = result.elements[i - 1];
            if (!(firstReference instanceof Out.CountedRuleElement)) continue;
            if (firstReference.count) continue;
            if (!(firstReference.countableRuleElement instanceof Out.RuleReference)) continue;

            const repeatingPair = result.elements[i];
            if (!(repeatingPair instanceof Out.CountedRuleElement)) continue;
            if (repeatingPair.count !== Out.Count.ZeroOrMore && repeatingPair.count !== Out.Count.OneOrMore) continue;
            if (!(repeatingPair.countableRuleElement instanceof Out.SequenceRule)) continue;
            if (repeatingPair.countableRuleElement.elements.length !== 2) continue;

            const separatorStringElement = repeatingPair.countableRuleElement.elements[0];
            if (!(separatorStringElement instanceof Out.CountedRuleElement)) continue;
            if (separatorStringElement.count) continue;
            if (!(separatorStringElement.countableRuleElement instanceof Out.StringElement)) continue;

            const secondReference = repeatingPair.countableRuleElement.elements[1];
            if (!(secondReference instanceof Out.CountedRuleElement)) continue;
            if (secondReference.count) continue;
            if (!(secondReference.countableRuleElement instanceof Out.RuleReference)) continue;

            if (!areSameRuleReference(firstReference, secondReference)) continue;

            firstReference.countableRuleElement = new Out.SeparatedByRule({
                element: secondReference,
                separator: separatorStringElement.countableRuleElement.value,
                minCount: repeatingPair.count === Out.Count.OneOrMore ? 1 : 0
            });
            firstReference.label = undefined;
            result.elements.splice(i, 1);
            i--;
        }

        return result;
    }
}

function areSameRuleReference(a: Out.CountedRuleElement, b: Out.CountedRuleElement): boolean {
    if (a.count !== b.count) return false;
    if (a.label !== b.label) return false;
    const arr = a.countableRuleElement;
    const brr = b.countableRuleElement;
    if (!(arr instanceof In.RuleReference)) return false;
    if (!(brr instanceof In.RuleReference)) return false;
    if (arr.names.length !== brr.names.length) return false;
    if (!arr.names.every((name, index) => name === brr.names[index])) return false;
    return true;
}
