// Generated on 2024-10-15T17:16:26.387Z
import * as Model from './model';

export class Visitor {
    visitGrammar(node: Model.Grammar): void {
        node.names.forEach((x) => {});
        node.rules.forEach((x) => {
            this.visitRule(x);
        });
        node.prattRules.forEach((x) => {
            this.visitPrattRule(x);
        });
        node.definitions.forEach((x) => {});
    }

    visitRule(node: Model.Rule): void {
        if (node.annotation != undefined) {
            this.visitRuleAnnotation(node.annotation);
        }
        node.versionAnnotations.forEach((x) => {
            this.visitVersionAnnotation(x);
        });
        this.visitRuleBody(node.body);
    }

    visitRuleAnnotation(node: Model.RuleAnnotation): void {}

    visitPrattRule(node: Model.PrattRule): void {
        node.versionAnnotations.forEach((x) => {
            this.visitVersionAnnotation(x);
        });
        node.operators.forEach((x) => {
            this.visitPrattOperator(x);
        });
        this.visitPrattPrimary(node.primary);
    }

    visitPrattOperator(node: Model.PrattOperator): void {
        this.visitPrattOperatorType(node.type);
        node.versionAnnotations.forEach((x) => {
            this.visitVersionAnnotation(x);
        });
        this.visitRuleBody(node.body);
    }

    visitPrattPrimary(node: Model.PrattPrimary): void {
        this.visitRuleBody(node.body);
    }

    visitPrattOperatorType(node: Model.PrattOperatorType): void {}

    visitVersionAnnotation(node: Model.VersionAnnotation): void {
        this.visitVersionAnnotationType(node.type);
        this.visitVersionNumber(node.version);
    }

    visitVersionAnnotationType(node: Model.VersionAnnotationType): void {}

    visitVersionNumber(node: Model.VersionNumber): void {
        node.forEach((x) => {});
    }

    visitRuleBody(node: Model.RuleBody): void {
        switch (node.discriminator) {
            case Model.Discriminator.ChoiceRule:
                this.visitChoiceRule(node);
                break;
            case Model.Discriminator.SequenceRule:
                this.visitSequenceRule(node);
                break;
            case Model.Discriminator.EnumRule:
                this.visitEnumRule(node);
                break;
            case Model.Discriminator.SeparatedByRule:
                this.visitSeparatedByRule(node);
                break;
        }
    }

    visitChoiceRule(node: Model.ChoiceRule): void {
        node.choices.forEach((x) => {
            this.visitSequenceRule(x);
        });
    }

    visitSequenceRule(node: Model.SequenceRule): void {
        node.elements.forEach((x) => {
            this.visitRuleElement(x);
        });
    }

    visitRuleElement(node: Model.RuleElement): void {
        switch (node.discriminator) {
            case Model.Discriminator.CountedRuleElement:
                this.visitCountedRuleElement(node);
                break;
            case Model.Discriminator.NegativeLookahead:
                this.visitNegativeLookahead(node);
                break;
        }
    }

    visitCountedRuleElement(node: Model.CountedRuleElement): void {
        this.visitCountableRuleElement(node.countableRuleElement);
        if (node.count != undefined) {
            this.visitCount(node.count);
        }
        node.versionAnnotations.forEach((x) => {
            this.visitVersionAnnotation(x);
        });
    }

    visitCountableRuleElement(node: Model.CountableRuleElement): void {
        switch (node.discriminator) {
            case Model.Discriminator.RuleReference:
                this.visitRuleReference(node);
                break;
            case Model.Discriminator.StringElement:
                this.visitStringElement(node);
                break;
            case Model.Discriminator.CharSet:
                this.visitCharSet(node);
                break;
            case Model.Discriminator.AnyElement:
                this.visitAnyElement(node);
                break;
            case Model.Discriminator.ChoiceRule:
            case Model.Discriminator.SequenceRule:
            case Model.Discriminator.EnumRule:
            case Model.Discriminator.SeparatedByRule:
                this.visitRuleBody(node);
                break;
        }
    }

    visitCount(node: Model.Count): void {}

    visitRuleReference(node: Model.RuleReference): void {
        node.names.forEach((x) => {});
        if (node.field != undefined) {
            this.visitField(node.field);
        }
    }

    visitStringElement(node: Model.StringElement): void {
        if (node.field != undefined) {
            this.visitField(node.field);
        }
    }

    visitCharSet(node: Model.CharSet): void {
        node.startChars.forEach((x) => {});
        node.endChars.forEach((x) => {
            if (x != undefined) {
            }
        });
        if (node.field != undefined) {
            this.visitField(node.field);
        }
    }

    visitAnyElement(node: Model.AnyElement): void {
        if (node.field != undefined) {
            this.visitField(node.field);
        }
    }

    visitNegativeLookahead(node: Model.NegativeLookahead): void {
        switch (node.content.discriminator) {
            case Model.Discriminator.CharSet:
                this.visitCharSet(node.content);
                break;
            case Model.Discriminator.StringElement:
                this.visitStringElement(node.content);
                break;
        }
    }

    visitTrivia(node: Model.Trivia): void {
        switch (node.discriminator) {
            case Model.Discriminator.LineComment:
                this.visitLineComment(node);
                break;
            case Model.Discriminator.BlockComment:
                this.visitBlockComment(node);
                break;
            case Model.Discriminator.Whitespace:
                this.visitWhitespace(node);
                break;
        }
    }

    visitLineComment(node: Model.LineComment): void {}

    visitBlockComment(node: Model.BlockComment): void {}

    visitWhitespace(node: Model.Whitespace): void {}

    visitEnumRule(node: Model.EnumRule): void {
        node.members.forEach((x) => {});
        if (node.field != undefined) {
            this.visitField(node.field);
        }
    }

    visitSeparatedByRule(node: Model.SeparatedByRule): void {
        this.visitRuleElement(node.element);
    }

    visitField(node: Model.Field): void {
        if (node.name != undefined) {
        }
    }
}
