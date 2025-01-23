import * as In from '../../grammar/extended/model';
import * as Out from './model';

export class Transformer {
    transformGrammar(input: In.Grammar): Out.Grammar {
        return new Out.Grammar({
            names: input.names,
            rules: input.rules.map((r) => this.transformRule(r)),
            prattRules: input.prattRules.map((r) => this.transformPrattRule(r)),
            definitions: []
        });
    }

    transformRuleBody(input: In.RuleBody): Out.RuleBody {
        if (input instanceof In.SequenceRule) {
            return this.transformSequenceRule(input);
        } else if (input instanceof In.ChoiceRule) {
            return this.transformChoiceRule(input);
        } else if (input instanceof In.EnumRule) {
            return this.transformEnumRule(input);
        } else {
            return this.transformSeparatedByRule(input);
        }
    }

    transformCountableRuleElement(input: In.CountableRuleElement): Out.CountableRuleElement {
        if (input instanceof In.RuleReference) {
            return this.transformRuleReference(input);
        } else if (input instanceof In.StringElement) {
            return this.transformStringElement(input);
        } else if (input instanceof In.CharSet) {
            return this.transformCharSet(input);
        } else if (input instanceof In.AnyElement) {
            return this.transformAnyElement(input);
        } else {
            return this.transformRuleBody(input);
        }
    }

    transformRuleElement(input: In.RuleElement): Out.RuleElement {
        if (input instanceof In.CountedRuleElement) {
            return this.transformCountedRuleElement(input);
        } else {
            return this.transformNegativeLookahead(input);
        }
    }

    transformRule(input: In.Rule): Out.Rule {
        throw new Error('Automatic transformation not possible');
    }

    transformRuleAnnotation(input: In.RuleAnnotation): Out.RuleAnnotation {
        switch (input.value) {
            case In.RuleAnnotationEnum.NoSkip:
                return Out.RuleAnnotation.NoSkip;
            case In.RuleAnnotationEnum.Atomic:
                return Out.RuleAnnotation.Atomic;
        }
    }

    transformPrattRule(input: In.PrattRule): Out.PrattRule {
        return new Out.PrattRule({
            name: input.name,
            operators: input.operators.map((o) => this.transformPrattOperator(o)),
            primary: this.transformPrattPrimary(input.primary),
            versionAnnotations: input.versionAnnotations.map((x) => this.transformVersionAnnotation(x))
        });
    }

    transformPrattOperator(input: In.PrattOperator): Out.PrattOperator {
        return new Out.PrattOperator({
            type: this.transformPrattOperatorType(input.type),
            name: input.name,
            body: this.transformRuleBody(input.body),
            versionAnnotations: input.versionAnnotations.map((x) => this.transformVersionAnnotation(x))
        });
    }

    transformPrattOperatorType(input: In.PrattOperatorType): Out.PrattOperatorType {
        switch (input.value) {
            case In.PrattOperatorTypeEnum.Prefix:
                return Out.PrattOperatorType.Prefix;
            case In.PrattOperatorTypeEnum.Postfix:
                return Out.PrattOperatorType.Postfix;
            case In.PrattOperatorTypeEnum.Left:
                return Out.PrattOperatorType.Left;
            case In.PrattOperatorTypeEnum.Right:
                return Out.PrattOperatorType.Right;
        }
    }

    transformPrattPrimary(input: In.PrattPrimary): Out.PrattPrimary {
        return new Out.PrattPrimary({ name: input.name, body: this.transformRuleBody(input.body) });
    }

    transformSequenceRule(input: In.SequenceRule): Out.SequenceRule {
        return new Out.SequenceRule({ elements: input.elements.map((e) => this.transformRuleElement(e)) });
    }

    transformChoiceRule(input: In.ChoiceRule): Out.ChoiceRule {
        return new Out.ChoiceRule({ choices: input.choices.map((a) => this.transformSequenceRule(a)) });
    }

    transformEnumRule(input: In.EnumRule): Out.EnumRule {
        return new Out.EnumRule({ members: input.members });
    }

    transformSeparatedByRule(input: In.SeparatedByRule): Out.SeparatedByRule {
        return new Out.SeparatedByRule({
            element: this.transformRuleElement(input.element),
            separator: input.separator,
            minCount: input.minCount
        });
    }

    transformCountedRuleElement(input: In.CountedRuleElement): Out.CountedRuleElement {
        return new Out.CountedRuleElement({
            countableRuleElement: this.transformCountableRuleElement(input.countableRuleElement),
            count: input.count && this.transformCount(input.count),
            versionAnnotations: input.versionAnnotations.map((x) => this.transformVersionAnnotation(x))
        });
    }

    transformCharSet(input: In.CharSet): Out.CharSet {
        return new Out.CharSet({ negated: input.negated, startChars: input.startChars, endChars: input.endChars });
    }

    transformNegativeLookahead(input: In.NegativeLookahead): Out.NegativeLookahead {
        return new Out.NegativeLookahead({ content: this.transformNegativeLookaheadContent(input.content) });
    }

    transformNegativeLookaheadContent(input: In.CharSet | In.StringElement): Out.CharSet | Out.StringElement {
        if (input instanceof In.CharSet) {
            return this.transformCharSet(input);
        } else {
            return this.transformStringElement(input);
        }
    }

    transformRuleReference(input: In.RuleReference): Out.RuleReference {
        return new Out.RuleReference({ names: input.names });
    }

    transformStringElement(input: In.StringElement): Out.StringElement {
        return new Out.StringElement({ value: input.value });
    }

    transformAnyElement(input: In.AnyElement): Out.AnyElement {
        return new Out.AnyElement({});
    }

    transformVersionAnnotation(input: In.VersionAnnotation): Out.VersionAnnotation {
        return new Out.VersionAnnotation({
            type: this.transformVersionAnnotationType(input.type),
            version: input.version
        });
    }

    transformVersionAnnotationType(input: In.VersionAnnotationType): Out.VersionAnnotationType {
        switch (input.value) {
            case In.VersionAnnotationTypeEnum.Enabled:
                return Out.VersionAnnotationType.Enabled;
            case In.VersionAnnotationTypeEnum.Disabled:
                return Out.VersionAnnotationType.Disabled;
        }
    }

    transformCount(input: In.Count): Out.Count {
        switch (input.value) {
            case In.CountEnum.OneOrMore:
                return Out.Count.OneOrMore;
            case In.CountEnum.ZeroOrMore:
                return Out.Count.ZeroOrMore;
            case In.CountEnum.Optional:
                return Out.Count.Optional;
        }
    }
}
