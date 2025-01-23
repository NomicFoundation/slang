// Generated on 2024-10-15T17:19:37.174Z
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
        if (node.label != undefined) {
        }
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
                this.visitRuleBody(node);
                break;
        }
    }

    visitCount(node: Model.Count): void {}

    visitRuleReference(node: Model.RuleReference): void {
        node.names.forEach((x) => {});
    }

    visitStringElement(node: Model.StringElement): void {}

    visitCharSet(node: Model.CharSet): void {
        node.startChars.forEach((x) => {});
        node.endChars.forEach((x) => {
            if (x != undefined) {
            }
        });
    }

    visitAnyElement(node: Model.AnyElement): void {}

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
}
