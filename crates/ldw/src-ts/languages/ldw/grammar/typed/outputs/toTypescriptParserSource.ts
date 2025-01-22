import assert from 'assert';
import { camelCase, pascalCase } from 'literal-case';
import { singular } from 'pluralize';
import { IndentingOutputStream } from '../../../../../output/indentingOutputStream';
import * as LdwModelParsed from '../../../model/parsed/model';
import { typeAsTypescript } from '../../../model/parsed/typescript';
import * as Model from '../model';
import { Visitor } from '../visitor';

export class TypedGrammarToTypescriptParserSource {
    constructor(public roots: string[]) {}

    transform(grammar: Model.Grammar): string {
        // Move this to a validation pass
        // Store it in the extended model or move it to utils?

        const rootRules: Model.Rule[] = [];
        for (const root of this.roots) {
            const rule = grammar.rules.find((rule) => rule.name.toLocaleLowerCase() === root.toLowerCase());
            if (!rule) throw new Error(`Root rule ${root} not found`);
            rootRules.push(rule);
        }
        const reachableRules = transitiveClosureOfRules(rootRules, grammar);

        const identifierRule = grammar.rules.find((rule) => rule.name.toLowerCase() === 'identifier');
        if (!identifierRule) throw new Error(`Identifier rule not found`);
        const identifierReachableRules = transitiveClosureOfRules([identifierRule], grammar);
        identifierReachableRules.delete(identifierRule);
        identifierReachableRules.forEach((rule) => {
            if (rule.annotation !== Model.RuleAnnotation.Atomic)
                throw new Error(`Identifier rule isn't atomic: ${rule.name}`);
        });

        const triviaRule = grammar.rules.find((rule) => rule.name.toLowerCase() === 'trivia');
        if (!triviaRule) throw new Error(`Trivia rule not found`);
        if (reachableRules.has(triviaRule)) {
            throw new Error(`Trivia rule should not be reachable from the root rules`);
        }
        const triviaChoices = collectSimpleNamedChoices(triviaRule, grammar);
        const triviaReachableRules = transitiveClosureOfRules([triviaRule], grammar);
        triviaReachableRules.delete(triviaRule);
        triviaReachableRules.forEach((rule) => {
            if (rule.annotation !== Model.RuleAnnotation.Atomic)
                throw new Error(`Trivia rule isn't atomic: ${rule.name}`);
            if (reachableRules.has(rule)) {
                throw new Error(`Trivia rule ${rule.name} should not be reachable from the root rules`);
            }
        });

        let output = new IndentingOutputStream();

        output.writeLine("import * as Model from './src-ts/languages/ldw/grammar/parsed/model';");
        output.writeLine("import * as HappyPathParser from './src-ts/parsing/happyPathParser';");
        output.writeLine("import * as Builder from './src-ts/parsing/builder';");
        output.writeLine("import assert from 'assert';");
        output.writeLine();

        output.writeLine(`export class Parser extends HappyPathParser.ParserBase {`);
        output.writeLine();

        reachableRules.forEach((rule) => {
            generatePublicParseMethod(rule, output);
        });
        reachableRules.forEach((rule) => {
            if (rule.annotation !== Model.RuleAnnotation.Atomic) generateNonAtomicParseMethod(rule, output);
        });
        reachableRules.forEach((rule) => {
            if (rule.annotation === Model.RuleAnnotation.Atomic) generateAtomicParseMethod(rule, output);
        });
        generateIdentifierHandlingMethods(identifierRule, output);
        generateTriviaHandlingMethods(triviaRule, triviaChoices, triviaReachableRules, output);

        output.writeLine('}');
        output.writeLine();

        reachableRules.forEach((rule) => {
            if (rule.annotation !== Model.RuleAnnotation.Atomic) {
                generateFieldEnum(rule, output);
            }
        });
        output.writeLine();

        output.writeLine('class Build {');
        reachableRules.forEach((rule) => {
            if (rule.annotation !== Model.RuleAnnotation.Atomic) {
                generateBuildFunction(rule, output);
            }
        });
        output.writeLine('}');
        output.writeLine();

        return output.toString().trim();
    }
}

function generateIdentifierHandlingMethods(identifierRule: Model.Rule, output: IndentingOutputStream): void {
    output.writeLine('consumeIdentifierForKeyword(): string | undefined {');
    output.writeLine(`const start = this.getPosition();`);
    output.writeLine(
        `if (this.#_lex${pascalCase(identifierRule.name)}()) return this.makeString(start, this.input.getPosition());`
    );
    output.writeLine('return undefined');
    output.writeLine('}');
    output.writeLine();
}

function generateTriviaHandlingMethods(
    triviaRule: Model.Rule,
    triviaChoices: Set<Model.Rule>,
    triviaReachableRules: Set<Model.Rule>,
    output: IndentingOutputStream
): void {
    output.writeLine('consumeTrivia(): string | undefined {');
    for (const rule of triviaChoices) {
        output.writeLine(`if (this.#_lex${pascalCase(rule.name)}()) return "${pascalCase(rule.name)}";`);
    }
    output.writeLine('return undefined');
    output.writeLine('}');
    output.writeLine();

    for (const rule of triviaReachableRules) {
        output.writeLine(`#_lex${pascalCase(rule.name)}(): boolean {`);
        output.indentDuring(() => {
            output.write('return ');
            new AtomicBodyGenerator(output).visitRule(rule);
        });
        output.writeLine('}');
        output.writeLine();
    }
}

function generatePublicParseMethod(rule: Model.Rule, output: IndentingOutputStream): void {
    const ruleName = pascalCase(rule.name);

    output.writeLine(`parse${ruleName}(label: string | undefined): boolean {`);
    output.write(`return `);
    output.writeLine(`this.skipTrivia(() => `);
    output.writeLine(`this.#_parse${ruleName}(label)`);
    output.writeLine(')');
    output.writeLine('}');
    output.writeLine();
}

function generateAtomicParseMethod(rule: Model.Rule, output: IndentingOutputStream): void {
    assert(rule.annotation == Model.RuleAnnotation.Atomic);

    const name = pascalCase(rule.name);

    output.writeLine(`#_parse${name}(label: string | undefined): boolean {`);
    output.write('return ');
    output.indentDuring(() => {
        if (LdwModelParsed.isPrimitiveType(rule.type)) {
            output.writeLine(`this.buildString(label, () => this.#_lex${name}())`);
        } else {
            output.writeLine(`this.buildStringObject(label, Model.${name}, () => this.#_lex${name}())`);
        }
    });
    output.writeLine('}');
    output.writeLine();

    output.writeLine(`#_lex${name}(): boolean {`);
    output.indentDuring(() => {
        output.write('return ');
        new AtomicBodyGenerator(output).visitRule(rule);
    });
    output.writeLine('}');
    output.writeLine();
}

function generateNonAtomicParseMethod(rule: Model.Rule, output: IndentingOutputStream): void {
    assert(rule.annotation !== Model.RuleAnnotation.Atomic);

    const ruleName = pascalCase(rule.name);
    const buildFnName = camelCase(rule.name);

    const isNoSkip = rule.annotation === Model.RuleAnnotation.NoSkip;
    output.writeLine(`#_parse${ruleName}(label: string | undefined): boolean {`);
    output.write(`return `);

    function noSkipTrivia(content: () => void): void {
        if (isNoSkip) {
            output.writeLine(`this.ignoreSkipTriviaDuring(() => `);
            content();
            output.writeLine(')');
        } else {
            content();
        }
    }

    noSkipTrivia(() => {
        switch (rule.type.discriminator) {
            case LdwModelParsed.Discriminator.NamedTypeReference:
            case LdwModelParsed.Discriminator.SumType:
                new NonAtomicBodyGenerator(output, true, rule, ruleName, isNoSkip).visitRule(rule);
                break;
            case LdwModelParsed.Discriminator.SequenceType:
            case LdwModelParsed.Discriminator.ProductType:
                output.write(`this.buildObject(label, Build.${buildFnName}, () => `);
                new NonAtomicBodyGenerator(output, false, rule, ruleName, isNoSkip).visitRule(rule);
                output.writeLine(`)`);
                break;
            case LdwModelParsed.Discriminator.EnumType:
                output.writeLine(`this.buildEnum(label,`);
                output.join(rule.type.members, ',', (member) => {
                    output.writeLine(`[ '${member}', Model.${ruleName}.${pascalCase(member)} ]`);
                });
                output.writeLine(`)`);
                break;
            default:
                throw new Error(`Not yet implemented: rule type discriminator ${rule.type.discriminator}`);
        }
    });

    output.writeLine('}');
    output.writeLine();
}

function generateFieldEnum(rule: Model.Rule, output: IndentingOutputStream): void {
    assert(rule.annotation !== Model.RuleAnnotation.Atomic);

    if (LdwModelParsed.isProductType(rule.type)) {
        const ruleName = pascalCase(rule.name);
        output.writeLine(`enum ${ruleName}Field {`);
        rule.type.members.forEach((member) => {
            if (LdwModelParsed.isSequenceType(member.type)) {
                output.writeLine(
                    `${pascalCase(singular(member.name))} = "${ruleName}.${singular(camelCase(member.name))}",`
                );
            }
            output.writeLine(`${pascalCase(member.name)} = "${ruleName}.${camelCase(member.name)}",`);
        });
        output.writeLine(`}`);
        output.writeLine();
    }
}

function generateBuildFunction(rule: Model.Rule, output: IndentingOutputStream): void {
    assert(rule.annotation !== Model.RuleAnnotation.Atomic);

    const ruleName = pascalCase(rule.name);
    const fnName = camelCase(rule.name);

    switch (rule.type.discriminator) {
        case LdwModelParsed.Discriminator.SequenceType:
            output.writeLine(
                `static ${fnName}(stack: Builder.Stack): Model.${ruleName} { return stack.map(x => x.value) }`
            );
            output.writeLine();
            break;
        case LdwModelParsed.Discriminator.ProductType:
            output.writeLine(`static ${fnName}(stack: Builder.Stack): Model.${ruleName} {`);
            if (rule.type.members.length === 0) {
                output.writeLine(`return new Model.${ruleName}()`);
            } else {
                rule.type.members.forEach((member) => {
                    if (LdwModelParsed.isSequenceType(member.type)) {
                        output.writeLine(
                            `let ${camelCase(member.name)}: ${typeAsTypescript(member.type, 'Model.')} = [];`
                        );
                    } else {
                        let newType: LdwModelParsed.Type = member.type;
                        if (!LdwModelParsed.isOptionType(newType))
                            newType = new LdwModelParsed.OptionType({ type: newType });
                        output.writeLine(
                            `let ${camelCase(member.name)}: ${typeAsTypescript(newType, 'Model.')} = undefined;`
                        );
                    }
                });
                output.writeLine(`for (const x of stack) { switch (x.label) {`);
                rule.type.members.forEach((member) => {
                    output.writeLine(`case ${ruleName}Field.${pascalCase(member.name)}:`);
                    output.writeLine(
                        `${camelCase(member.name)} = x.value as ${typeAsTypescript(member.type, 'Model.')};`
                    );
                    output.writeLine('break');
                    if (LdwModelParsed.isSequenceType(member.type)) {
                        output.writeLine(`case ${ruleName}Field.${singular(pascalCase(member.name))}:`);
                        const elementType = member.type.elementType;
                        output.writeLine(
                            `${camelCase(member.name)}.push(x.value as ${typeAsTypescript(elementType, 'Model.')});`
                        );
                        output.writeLine('break');
                    }
                });
                output.writeLine(`default: break`);
                output.writeLine(`}}`);
                rule.type.members.forEach((member) => {
                    if (!LdwModelParsed.isSequenceType(member.type) && !LdwModelParsed.isOptionType(member.type))
                        output.writeLine(`assert(${camelCase(member.name)} !== undefined)`);
                });
                output.writeLine(`return new Model.${ruleName}({`);
                rule.type.members.forEach((member) => {
                    output.writeLine(`${camelCase(member.name)}: ${camelCase(member.name)},`);
                });
                output.writeLine(`})`);
            }
            output.writeLine(`}`);
            output.writeLine();
            break;
        default:
            break;
    }
}

function collectSimpleNamedChoices(rule: Model.Rule, grammar: Model.Grammar): Set<Model.Rule> {
    const namedChoices = new Set<Model.Rule>();
    new (class extends Visitor {
        visitRuleBody(node: Model.RuleBody): void {
            switch (node.discriminator) {
                case Model.Discriminator.SequenceRule:
                    this.visitSequenceRule(node);
                    break;
                case Model.Discriminator.ChoiceRule:
                    this.visitChoiceRule(node);
                    break;
                default:
                    throw new Error(`Expected simple named choices in: ${rule.name}`);
            }
        }
        visitSequenceRule(node: Model.SequenceRule): void {
            if (node.elements.length !== 1) {
                throw new Error(`Expected simple named choices in: ${rule.name}`);
            }
            switch (node.elements[0].discriminator) {
                case Model.Discriminator.CountedRuleElement:
                    this.visitCountedRuleElement(node.elements[0]);
                    break;
                default:
                    throw new Error(`Expected simple named choices in: ${rule.name}`);
            }
        }
        visitCountedRuleElement(node: Model.CountedRuleElement): void {
            if (node.count) {
                throw new Error(`Expected simple named choices in: ${rule.name}`);
            }
            switch (node.countableRuleElement.discriminator) {
                case Model.Discriminator.SequenceRule:
                    this.visitSequenceRule(node.countableRuleElement);
                    break;
                case Model.Discriminator.ChoiceRule:
                    this.visitChoiceRule(node.countableRuleElement);
                    break;
                case Model.Discriminator.RuleReference:
                    const names = node.countableRuleElement.names;
                    if (names.length !== 1) {
                        throw new Error(`Expected simple named choices in: ${rule.name}`);
                    }
                    const referencedRule = grammar.rules.find((r) => r.name === names[0]);
                    if (!referencedRule) {
                        throw new Error(`Referenced rule ${names[0]} not found`);
                    }
                    namedChoices.add(referencedRule);
                    break;
                default:
                    throw new Error(`Expected simple named choices in: ${rule.name}`);
            }
        }
    })().visitRule(rule);

    return namedChoices;
}

function transitiveClosureOfRules(rules: Model.Rule[], grammar: Model.Grammar): Set<Model.Rule> {
    const closure = new Set<Model.Rule>();
    const queue: Model.Rule[] = [...rules];
    const seen = new Set<string>();

    while (queue.length > 0) {
        const currentRule = queue.shift()!;
        if (seen.has(currentRule.name)) continue;
        seen.add(currentRule.name);
        closure.add(currentRule);

        const referencedRules = new Set<string>();
        new (class extends Visitor {
            visitRuleReference(node: Model.RuleReference): void {
                if (node.names.length === 1) referencedRules.add(node.names[node.names.length - 1]);
            }
        })().visitRule(currentRule);

        for (const referencedRuleName of referencedRules) {
            const referencedRule = grammar.rules.find((r) => r.name === referencedRuleName);
            if (!referencedRule) {
                throw new Error(`Referenced rule ${referencedRuleName} not found`);
            }
            if (!seen.has(referencedRule.name)) {
                queue.push(referencedRule);
            }
        }
    }

    return closure;
}

class NonAtomicBodyGenerator extends Visitor {
    constructor(
        public output: IndentingOutputStream,
        public useLabelParameter: boolean,
        public rule: Model.Rule,
        public ruleName: string,
        public isNoSkip: boolean
    ) {
        super();
    }

    skipTrivia(content: () => void): void {
        if (this.isNoSkip) {
            content();
        } else {
            this.output.writeLine(`this.skipTrivia(() => `);
            content();
            this.output.writeLine(')');
        }
    }

    visitSequenceRule(node: Model.SequenceRule): void {
        if (node.elements.length === 1) {
            this.visitRuleElement(node.elements[0]);
        } else {
            this.output.writeLine(`this.skipSeq(() => `);
            this.output.join(node.elements, ' && ', (element) => this.visitRuleElement(element));
            this.output.writeLine(`)`);
        }
    }

    visitChoiceRule(node: Model.ChoiceRule): void {
        // Opportunity to join CharSet children
        this.output.writeLine(`(`);
        this.output.join(node.choices, ' || ', (choice) => this.visitSequenceRule(choice));
        this.output.writeLine(`)`);
    }

    visitEnumRule(node: Model.EnumRule): void {
        throw new Error('Not implemented');
    }

    visitSeparatedByRule(node: Model.SeparatedByRule): void {
        throw new Error('Not implemented');
    }

    visitCountedRuleElement(node: Model.CountedRuleElement): void {
        // Oportunity to lift CharSet children
        switch (node.count) {
            case Model.Count.OneOrMore:
                this.output.writeLine('this.skipOneOrMore(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            case Model.Count.ZeroOrMore:
                this.output.writeLine('this.skipZeroOrMore(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            case Model.Count.Optional:
                this.output.writeLine('this.skipOptional(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            default:
                super.visitCountedRuleElement(node);
        }
    }

    visitRuleReference(node: Model.RuleReference): void {
        if (node.field) {
            // If we are calling a non-atomic rule, we don't need to skip trivia
            if (this.useLabelParameter) {
                this.output.writeLine(`this.#_parse${pascalCase(node.names[node.names.length - 1])}(label)`);
            } else {
                if (LdwModelParsed.isSequenceType(this.rule.type)) {
                    this.output.writeLine(`this.#_parse${pascalCase(node.names[node.names.length - 1])}(undefined)`);
                } else {
                    this.output.writeLine(
                        `this.#_parse${pascalCase(node.names[node.names.length - 1])}(${this.ruleName}Field.${pascalCase(node.field.name!)})`
                    );
                }
            }
        } else {
            this.skipTrivia(() =>
                this.output.writeLine(`this.#_lex${pascalCase(node.names[node.names.length - 1])}()`)
            );
        }
    }

    visitStringElement(node: Model.StringElement): void {
        let text = node.value.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
        this.skipTrivia(() => {
            if (node.field) {
                this.output.writeLine(
                    `this.buildBoolean('${camelCase(node.field!.name!)}', () => this.skipString('${text}'))`
                );
            } else {
                this.output.writeLine(`this.skipString('${text}')`);
            }
        });
    }

    visitCharSet(node: Model.CharSet): void {
        // TODO: lots of opportunity to optimise for specific regex forms
        const regex = charSetToRegex(node);
        this.skipTrivia(() => {
            if (node.field) {
                this.output.writeLine(
                    `this.buildString('${camelCase(node.field.name!)}', () => this.skipRegex(${regex}))`
                );
            } else {
                this.output.writeLine(`skipRegex(${regex})`);
            }
        });
    }

    visitAnyElement(node: Model.AnyElement): void {
        throw new Error('Not implemented');
    }

    visitNegativeLookahead(node: Model.NegativeLookahead): void {
        throw new Error('Not implemented');
    }
}

class AtomicBodyGenerator extends Visitor {
    constructor(public output: IndentingOutputStream) {
        super();
    }

    visitSequenceRule(node: Model.SequenceRule): void {
        if (node.elements.length === 1) {
            this.visitRuleElement(node.elements[0]);
        } else {
            this.output.writeLine(`this.skipSeq(() => `);
            this.output.join(node.elements, ' && ', (element) => this.visitRuleElement(element));
            this.output.writeLine(`)`);
        }
    }

    visitChoiceRule(node: Model.ChoiceRule): void {
        // Oportunity to join CharSet children
        this.output.writeLine(`(`);
        this.output.join(node.choices, ' || ', (choice) => this.visitSequenceRule(choice));
        this.output.writeLine(`)`);
    }

    visitEnumRule(node: Model.EnumRule): void {
        throw new Error('Not implemented');
    }

    visitSeparatedByRule(node: Model.SeparatedByRule): void {
        throw new Error('Not implemented');
    }

    visitCountedRuleElement(node: Model.CountedRuleElement): void {
        // Oportunity to lift CharSet children
        switch (node.count) {
            case Model.Count.OneOrMore:
                this.output.writeLine('this.skipOneOrMore(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            case Model.Count.ZeroOrMore:
                this.output.writeLine('this.skipZeroOrMore(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            case Model.Count.Optional:
                this.output.writeLine('this.skipOptional(() => ');
                super.visitCountedRuleElement(node);
                this.output.writeLine(')');
                break;
            default:
                super.visitCountedRuleElement(node);
        }
    }

    visitRuleReference(node: Model.RuleReference): void {
        // TODO: must earlier validate that atomic rules only refer to other atomic rules
        this.output.writeLine(`this.#_lex${pascalCase(node.names[node.names.length - 1])}()`);
    }

    visitStringElement(node: Model.StringElement): void {
        let text = node.value.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
        this.output.writeLine(`this.skipString('${text}')`);
    }

    visitCharSet(node: Model.CharSet): void {
        // TODO: lots of opportunity to optimise for specific regex forms
        const regex = charSetToRegex(node);
        this.output.writeLine(`this.skipRegex(${regex})`);
    }

    visitNegativeLookahead(node: Model.NegativeLookahead): void {
        this.output.write('this.skipNegativeLookahead(() =>');
        super.visitNegativeLookahead(node);
        this.output.write(')');
    }

    visitAnyElement(node: Model.AnyElement): void {
        this.output.writeLine('this.skip()');
    }
}

function charSetToRegex(charSet: Model.CharSet): string {
    let regex = charSet.negated ? '/[^' : '/[';
    for (let i = 0; i < charSet.startChars.length; i++) {
        const startChar = charSet.startChars[i];
        const endChar = charSet.endChars[i];
        if (endChar) {
            regex += `${startChar}-${endChar}`;
        } else {
            regex += startChar;
        }
    }
    regex += ']/';
    return regex;
}
