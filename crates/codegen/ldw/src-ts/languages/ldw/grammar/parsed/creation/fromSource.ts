import { InputStream } from '../../../../../parsing/inputStream';
import { Parser, ParseResult } from '../../../../../parsing/parser';
import { StringInputStream } from '../../../../../parsing/stringInputStream';
import * as Model from '../model';
import { Grammar } from '../model';

export class ParsedGrammarFromSource {
    transform(input: string): Grammar {
        const inputStream = new StringInputStream(input);
        const parser = new GrammarParser(inputStream, false);
        const grammar = parser.successOrThrow(parser.parseGrammar());
        parser.successOrThrow(parser.mustBeEOF());
        return grammar;
    }
}

type TriviaKind = 'LineComment' | 'BlockComment' | 'Whitespace';

class GrammarParser extends Parser {
    constructor(input: InputStream, debug: boolean = false) {
        super(input, debug);
    }

    parseGrammar(): ParseResult<Model.Grammar> {
        return this.withContext('grammar', () => {
            let name: Model.Name[] = [];
            let rules: Model.Rule[] = [];
            let prattRules: Model.PrattRule[] = [];

            const grammarKeyword = this.mustConsumeKeyword('grammar');
            if (!grammarKeyword.success) return grammarKeyword;

            const nameResult = this.parseName();
            if (!nameResult.success) return nameResult;
            name.push(nameResult.value);

            this.zeroOrMore(() => {
                const doubleColon = this.mustConsumeString('::');
                if (!doubleColon.success) return doubleColon;
                const nameResult = this.parseName();
                if (!nameResult.success) return nameResult;
                name.push(nameResult.value);
                return nameResult;
            });

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            const elements = this.zeroOrMore(() =>
                this.firstAlternative(
                    'grammar rule',
                    () => {
                        const prattRuleResult = this.parsePrattRule();
                        if (!prattRuleResult.success) return prattRuleResult;
                        prattRules.push(prattRuleResult.value);
                        return this.success(undefined);
                    },
                    () => {
                        const ruleResult = this.parseRule();
                        if (!ruleResult.success) return ruleResult;
                        rules.push(ruleResult.value);
                        return this.success(undefined);
                    }
                )
            );

            if (!elements.success) return elements;

            const closeBrace = this.mustConsumeString('}');
            if (!closeBrace.success) return closeBrace;

            return this.success(new Model.Grammar({ names: name, rules, prattRules }));
        });
    }

    private parseRule(): ParseResult<Model.Rule> {
        return this.withContext('rule', () => {
            let name: Model.Name;
            let annotation: Model.RuleAnnotation | undefined;
            let versionAnnotations: Model.VersionAnnotation[];
            let body: Model.RuleBody;

            const nameResult = this.parseName();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const annotationResult = this.parseOptionalRuleAnnotation();
            if (!annotationResult.success) return annotationResult;
            annotation = annotationResult.value;

            const versionAnnotationsResult = this.parseVersionAnnotations();
            if (!versionAnnotationsResult.success) return versionAnnotationsResult;
            versionAnnotations = versionAnnotationsResult.value;

            const equals = this.mustConsumeString('=');
            if (!equals.success) return equals;

            const bodyResult = this.parseRuleBody();
            if (!bodyResult.success) return bodyResult;
            body = bodyResult.value;

            const semicolon = this.mustConsumeString(';');
            if (!semicolon.success) return semicolon;

            return this.success(new Model.Rule({ name, annotation, versionAnnotations, body }));
        });
    }

    private parseOptionalRuleAnnotation(): ParseResult<Model.RuleAnnotation | undefined> {
        if (this.consumeString('@noskip')) return this.success(Model.RuleAnnotation.NoSkip);
        if (this.consumeString('@atomic')) return this.success(Model.RuleAnnotation.Atomic);
        return this.success(undefined);
    }

    private parsePrattRule(): ParseResult<Model.PrattRule> {
        return this.withContext('pratt_rule', () => {
            let name: Model.Name;
            let versionAnnotations: Model.VersionAnnotation[];
            let operators: Model.PrattOperator[];
            let primary: Model.PrattPrimary;

            const prattKeyword = this.mustConsumeKeyword('pratt');
            if (!prattKeyword.success) return prattKeyword;

            const nameResult = this.parseName();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const versionAnnotationsResult = this.parseVersionAnnotations();
            if (!versionAnnotationsResult.success) return versionAnnotationsResult;
            versionAnnotations = versionAnnotationsResult.value;

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            const operatorsResult = this.oneOrMore(() => this.parsePrattOperator());
            if (!operatorsResult.success) return operatorsResult;
            operators = operatorsResult.value;

            const primaryResult = this.parsePrattPrimary();
            if (!primaryResult.success) return primaryResult;
            primary = primaryResult.value;

            const closeBrace = this.mustConsumeString('}');
            if (!closeBrace.success) return closeBrace;

            return this.success(new Model.PrattRule({ name, versionAnnotations, operators, primary }));
        });
    }

    private parsePrattOperator(): ParseResult<Model.PrattOperator> {
        return this.withContext('pratt_operator', () => {
            let type: Model.PrattOperatorType;
            let name: Model.Name;
            let versionAnnotations: Model.VersionAnnotation[];
            let body: Model.RuleBody;

            const typeResult = this.parsePrattOperatorType();
            if (!typeResult.success) return typeResult;
            type = typeResult.value;

            const nameResult = this.parseName();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const versionAnnotationsResult = this.parseVersionAnnotations();
            if (!versionAnnotationsResult.success) return versionAnnotationsResult;
            versionAnnotations = versionAnnotationsResult.value;

            const equals = this.mustConsumeString('=');
            if (!equals.success) return equals;

            const bodyResult = this.parseRuleBody();
            if (!bodyResult.success) return bodyResult;
            body = bodyResult.value;

            const semicolon = this.mustConsumeString(';');
            if (!semicolon.success) return semicolon;

            return this.success(new Model.PrattOperator({ type, name, versionAnnotations, body }));
        });
    }

    private parsePrattPrimary(): ParseResult<Model.PrattPrimary> {
        return this.withContext('pratt_primary', () => {
            let name: Model.Name;
            let body: Model.RuleBody;

            const primaryKeyword = this.mustConsumeKeyword('primary');
            if (!primaryKeyword.success) return primaryKeyword;

            const nameResult = this.parseName();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const equals = this.mustConsumeString('=');
            if (!equals.success) return equals;

            const bodyResult = this.parseRuleBody();
            if (!bodyResult.success) return bodyResult;
            body = bodyResult.value;

            const semicolon = this.mustConsumeString(';');
            if (!semicolon.success) return semicolon;

            return this.success(new Model.PrattPrimary({ name, body }));
        });
    }

    private parsePrattOperatorType(): ParseResult<Model.PrattOperatorType> {
        if (this.consumeString('prefix')) return this.success(Model.PrattOperatorType.Prefix);
        if (this.consumeString('postfix')) return this.success(Model.PrattOperatorType.Postfix);
        if (this.consumeString('left')) return this.success(Model.PrattOperatorType.Left);
        if (this.consumeString('right')) return this.success(Model.PrattOperatorType.Right);
        return this.failure('Invalid pratt operator type');
    }

    private parseVersionAnnotations(): ParseResult<Model.VersionAnnotation[]> {
        return this.zeroOrMore(() => this.parseVersionAnnotation());
    }

    private parseVersionAnnotation(): ParseResult<Model.VersionAnnotation> {
        return this.withContext('version_annotation', () => {
            let type: Model.VersionAnnotationType;
            let version: Model.VersionNumber;

            const typeResult = this.parseVersionAnnotationType();
            if (!typeResult.success) return typeResult;
            type = typeResult.value;

            const versionResult = this.ignoreTriviaDuring(() => {
                const openParen = this.mustConsumeString('(');
                if (!openParen.success) return openParen;

                const v = this.parseVersionNumber();
                if (!v.success) return v;

                const closeParen = this.mustConsumeString(')');
                if (!closeParen.success) return closeParen;

                return v;
            });
            if (!versionResult.success) return versionResult;
            version = versionResult.value;

            return this.success(new Model.VersionAnnotation({ type, version }));
        });
    }

    private parseVersionAnnotationType(): ParseResult<Model.VersionAnnotationType> {
        if (this.consumeString('@enabled')) return this.success(Model.VersionAnnotationType.Enabled);
        if (this.consumeString('@disabled')) return this.success(Model.VersionAnnotationType.Disabled);
        return this.failure('Missing version annotation type');
    }

    private parseVersionNumber(): ParseResult<Model.VersionNumber> {
        const firstSegmentResult = this.parseVersionSegment();
        if (!firstSegmentResult.success) return firstSegmentResult;

        const otherSegmentsResult = this.zeroOrMore(() => {
            const dot = this.consumeString('.');
            if (dot === undefined) return this.failure<Model.VersionSegment>('Expected "."');
            return this.parseVersionSegment();
        });

        if (!otherSegmentsResult.success) return otherSegmentsResult;

        const segments = [firstSegmentResult.value, ...otherSegmentsResult.value];
        return this.success(segments);
    }

    private parseVersionSegment(): ParseResult<Model.VersionSegment> {
        return this.mustConsumeRegex(/^[0-9]+/, 'version segment');
    }

    private parseRuleBody(): ParseResult<Model.RuleBody> {
        return this.withContext('rule_body', () => {
            return this.firstAlternative(
                'rule body',
                () => this.parseChoiceRule(),
                () => this.parseSequenceRule()
            );
        });
    }

    private parseChoiceRule(): ParseResult<Model.ChoiceRule> {
        return this.withContext('alternative_rule', () => {
            const firstAlternativeResult = this.parseSequenceRule();
            if (!firstAlternativeResult.success) return firstAlternativeResult;

            const otherAlternativesResult = this.oneOrMore(() => {
                const pipe = this.mustConsumeString('|');
                if (!pipe.success) return pipe;
                return this.parseSequenceRule();
            });

            if (!otherAlternativesResult.success) return otherAlternativesResult;

            const choices = [firstAlternativeResult.value, ...otherAlternativesResult.value];
            return this.success(new Model.ChoiceRule({ choices }));
        });
    }

    private parseSequenceRule(): ParseResult<Model.SequenceRule> {
        return this.withContext('rule_element_sequence', () => {
            const elementsResult = this.oneOrMore(() => this.parseRuleElement());
            if (!elementsResult.success) return elementsResult;
            return this.success(new Model.SequenceRule({ elements: elementsResult.value }));
        });
    }

    private parseRuleElement(): ParseResult<Model.RuleElement> {
        return this.withContext('rule_element', () => {
            return this.firstAlternative(
                'rule element or negative lookahead',
                () => this.parseCountedRuleElement(),
                () => this.parseNegativeLookahead()
            );
        });
    }

    private parseCountedRuleElement(): ParseResult<Model.CountedRuleElement> {
        let label: Model.Label | undefined;
        let countableRuleElement: Model.CountableRuleElement;
        let count: Model.Count | undefined;
        let versionAnnotations: Model.VersionAnnotation[];

        const labelResult = this.parseOptionalLabel();
        if (!labelResult.success) return labelResult;
        label = labelResult.value;

        const countableRuleElementResult = this.parseCountableRuleElement();
        if (!countableRuleElementResult.success) return countableRuleElementResult;
        countableRuleElement = countableRuleElementResult.value;

        const countResult = this.parseOptionalCount();
        if (!countResult.success) return countResult;
        count = countResult.value;

        const versionAnnotationsResult = this.parseVersionAnnotations();
        if (!versionAnnotationsResult.success) return versionAnnotationsResult;
        versionAnnotations = versionAnnotationsResult.value;

        return this.success(new Model.CountedRuleElement({ label, countableRuleElement, count, versionAnnotations }));
    }

    private parseCountableRuleElement(): ParseResult<Model.CountableRuleElement> {
        return this.withContext('countable_rule_element', () => {
            return this.firstAlternative(
                'countable rule element',
                () => this.parseRuleReference(),
                () => {
                    const stringResult = this.parseString();
                    if (stringResult.success) {
                        return this.success(new Model.StringElement({ value: stringResult.value }));
                    }
                    return stringResult;
                },
                () => this.parseCharset(),
                () => this.parseAny(),
                () => {
                    const openParen = this.mustConsumeString('(');
                    if (!openParen.success) return openParen;

                    const body = this.parseRuleBody();
                    if (!body.success) return body;

                    const closeParen = this.mustConsumeString(')');
                    if (!closeParen.success) return closeParen;

                    return body;
                }
            );
        });
    }

    private parseRuleReference(): ParseResult<Model.RuleReference> {
        return this.withContext('rule_reference', () => {
            const firstNameResult = this.parseName();
            if (!firstNameResult.success) return firstNameResult;

            const otherNamesResult = this.zeroOrMore(() => {
                const doubleColon = this.mustConsumeString('::');
                if (!doubleColon.success) return doubleColon;
                return this.parseName();
            });

            if (!otherNamesResult.success) return otherNamesResult;

            const names = [firstNameResult.value, ...otherNamesResult.value];
            return this.success(new Model.RuleReference({ names }));
        });
    }

    private parseOptionalCount(): ParseResult<Model.Count | undefined> {
        if (this.consumeString('+')) return this.success(Model.Count.OneOrMore);
        if (this.consumeString('*')) return this.success(Model.Count.ZeroOrMore);
        if (this.consumeString('?')) return this.success(Model.Count.Optional);
        return this.success(undefined);
    }

    private parseName(): ParseResult<Model.Name> {
        return this.mustConsumeRegex(/^[a-zA-Z_][a-zA-Z0-9_]*/, 'name');
    }

    private parseOptionalLabel(): ParseResult<Model.Label | undefined> {
        const startPos = this.mark();
        const nameResult = this.optional(() => this.parseName());
        if (!nameResult.success) return nameResult;

        if (nameResult.value) {
            if (this.consumeString(':')) {
                return this.success(nameResult.value);
            }
        }
        this.restore(startPos);
        return this.success(undefined);
    }

    private parseString(): ParseResult<string> {
        return this.ignoreTriviaDuring(() => {
            let str = '';

            const quoteResult = this.must(this.consumeString("'") || this.consumeString('"'), 'string');
            if (!quoteResult.success) return quoteResult;
            const quote = quoteResult.value;

            while (!this.isEOF() && this.peek() !== quote) {
                const escape = this.consumeString('\\');
                if (escape) str += escape;
                const charResult = this.must(this.consume(), 'continuation of string');
                if (!charResult.success) return charResult;
                str += charResult.value;
            }

            const closeQuoteResult = this.mustConsumeString(quote);
            if (!closeQuoteResult.success) return closeQuoteResult;

            return this.success(str);
        });
    }

    private parseCharset(): ParseResult<Model.CharSet> {
        return this.ignoreTriviaDuring(() => {
            const openBracket = this.mustConsumeString('[');
            if (!openBracket.success) return openBracket;

            const negated = this.consumeString('^') !== undefined;

            const startChars: Model.CharSetChar[] = [];
            const endChars: (Model.CharSetChar | undefined)[] = [];
            while (!this.isEOF() && this.peek() !== ']') {
                const startCharResult = this.parseCharsetChar();
                if (!startCharResult.success) return startCharResult;
                startChars.push(startCharResult.value);

                if (this.consumeString('-') && this.peek() !== ']') {
                    const endCharResult = this.parseCharsetChar();
                    if (!endCharResult.success) return endCharResult;
                    endChars.push(endCharResult.value);
                } else {
                    endChars.push(undefined);
                }
            }

            const closeBracket = this.mustConsumeString(']');
            if (!closeBracket.success) return closeBracket;

            return this.success(new Model.CharSet({ negated, startChars, endChars }));
        });
    }

    private parseCharsetChar(): ParseResult<Model.CharSetChar> {
        if (this.consumeString('\\')) {
            const escapedResult = this.must(this.consume(), 'escaped charset character');
            if (!escapedResult.success) return escapedResult;
            return this.success('\\' + escapedResult.value);
        }
        return this.must(this.consume(), 'charset character');
    }

    private parseAny(): ParseResult<Model.AnyElement> {
        const dot = this.mustConsumeString('.');
        if (!dot.success) return dot;
        return this.success(new Model.AnyElement());
    }

    private parseNegativeLookahead(): ParseResult<Model.NegativeLookahead> {
        return this.ignoreTriviaDuring(() => {
            const exclamation = this.mustConsumeString('!');
            if (!exclamation.success) return exclamation;

            const content = this.firstAlternative(
                'charset or string',
                () => this.parseCharset(),
                () => {
                    const stringResult = this.parseString();
                    if (stringResult.success) {
                        return this.success(new Model.StringElement({ value: stringResult.value }));
                    }
                    return stringResult;
                }
            );
            if (!content.success) return content;

            return this.success(new Model.NegativeLookahead({ content: content.value }));
        });
    }

    private parseTrivia(): ParseResult<TriviaKind | undefined> {
        if (this.parseLineComment()) return this.success('LineComment');
        if (this.parseBlockComment()) return this.success('BlockComment');
        if (this.parseWhitespace()) return this.success('Whitespace');
        return this.success(undefined);
    }

    private parseLineComment(): boolean {
        if (this.consumeString('//') === undefined) return false;
        this.consumeWhile((c) => c !== '\n');
        this.consumeString('\n');
        return true;
    }

    private parseBlockComment(): boolean {
        if (this.consumeString('/*') === undefined) return false;
        while (
            this.consumeRegex(/[^*]+/) !== undefined ||
            (this.consumeString('*') !== undefined && this.peek() !== '/')
        ) {}
        return this.consumeString('/') !== undefined;
    }

    private parseWhitespace(): boolean {
        return this.consumeRegex(/[\n\t ]+/) !== undefined;
    }

    protected consumeTrivia(): string | undefined {
        const result = this.parseTrivia();
        return result.success && result.value ? result.value : undefined;
    }

    protected consumeIdentifierForKeyword(): string | undefined {
        return this.consumeRegex(/^[a-zA-Z_][a-zA-Z0-9_]*/);
    }
}
