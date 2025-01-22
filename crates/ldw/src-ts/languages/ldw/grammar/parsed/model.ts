// Generated on 2024-10-15T13:11:27.645Z by Bach.local at /Users/antony/Development/Linkuistics/language-design-workbench

export enum Discriminator {
    ChoiceRule = 'ChoiceRule',
    SequenceRule = 'SequenceRule',
    CountedRuleElement = 'CountedRuleElement',
    RuleReference = 'RuleReference',
    StringElement = 'StringElement',
    CharSet = 'CharSet',
    AnyElement = 'AnyElement',
    NegativeLookahead = 'NegativeLookahead',
    LineComment = 'LineComment',
    BlockComment = 'BlockComment',
    Whitespace = 'Whitespace'
}

export class Grammar {
    public names: Name[];
    public rules: Rule[];
    public prattRules: PrattRule[];

    constructor(init: { names: Name[]; rules: Rule[]; prattRules: PrattRule[] }) {
        this.names = init.names;
        this.rules = init.rules;
        this.prattRules = init.prattRules;
    }
}

export class Rule {
    public name: Name;
    public annotation: RuleAnnotation | undefined;
    public versionAnnotations: VersionAnnotation[];
    public body: RuleBody;

    constructor(init: {
        name: Name;
        annotation?: RuleAnnotation | undefined;
        versionAnnotations: VersionAnnotation[];
        body: RuleBody;
    }) {
        this.name = init.name;
        this.annotation = init.annotation;
        this.versionAnnotations = init.versionAnnotations;
        this.body = init.body;
    }
}

export enum RuleAnnotationEnum {
    NoSkip = 'NoSkip',
    Atomic = 'Atomic'
}
export class RuleAnnotation {
    static NoSkip: RuleAnnotation = new RuleAnnotation(RuleAnnotationEnum.NoSkip);
    static Atomic: RuleAnnotation = new RuleAnnotation(RuleAnnotationEnum.Atomic);

    private constructor(public readonly value: RuleAnnotationEnum) {}
}

export class PrattRule {
    public name: Name;
    public versionAnnotations: VersionAnnotation[];
    public operators: PrattOperator[];
    public primary: PrattPrimary;

    constructor(init: {
        name: Name;
        versionAnnotations: VersionAnnotation[];
        operators: PrattOperator[];
        primary: PrattPrimary;
    }) {
        this.name = init.name;
        this.versionAnnotations = init.versionAnnotations;
        this.operators = init.operators;
        this.primary = init.primary;
    }
}

export class PrattOperator {
    public type: PrattOperatorType;
    public name: Name;
    public versionAnnotations: VersionAnnotation[];
    public body: RuleBody;

    constructor(init: {
        type: PrattOperatorType;
        name: Name;
        versionAnnotations: VersionAnnotation[];
        body: RuleBody;
    }) {
        this.type = init.type;
        this.name = init.name;
        this.versionAnnotations = init.versionAnnotations;
        this.body = init.body;
    }
}

export class PrattPrimary {
    public name: Name;
    public body: RuleBody;

    constructor(init: { name: Name; body: RuleBody }) {
        this.name = init.name;
        this.body = init.body;
    }
}

export enum PrattOperatorTypeEnum {
    Prefix = 'Prefix',
    Postfix = 'Postfix',
    Left = 'Left',
    Right = 'Right'
}
export class PrattOperatorType {
    static Prefix: PrattOperatorType = new PrattOperatorType(PrattOperatorTypeEnum.Prefix);
    static Postfix: PrattOperatorType = new PrattOperatorType(PrattOperatorTypeEnum.Postfix);
    static Left: PrattOperatorType = new PrattOperatorType(PrattOperatorTypeEnum.Left);
    static Right: PrattOperatorType = new PrattOperatorType(PrattOperatorTypeEnum.Right);

    private constructor(public readonly value: PrattOperatorTypeEnum) {}
}

export class VersionAnnotation {
    public type: VersionAnnotationType;
    public version: VersionNumber;

    constructor(init: { type: VersionAnnotationType; version: VersionNumber }) {
        this.type = init.type;
        this.version = init.version;
    }
}

export enum VersionAnnotationTypeEnum {
    Enabled = 'Enabled',
    Disabled = 'Disabled'
}
export class VersionAnnotationType {
    static Enabled: VersionAnnotationType = new VersionAnnotationType(VersionAnnotationTypeEnum.Enabled);
    static Disabled: VersionAnnotationType = new VersionAnnotationType(VersionAnnotationTypeEnum.Disabled);

    private constructor(public readonly value: VersionAnnotationTypeEnum) {}
}

export type VersionNumber = VersionSegment[];

export type VersionSegment = string;

export type RuleBody = ChoiceRule | SequenceRule;
export function isRuleBody(
    value: RuleReference | StringElement | CharSet | AnyElement | ChoiceRule | SequenceRule
): value is RuleBody {
    switch (value.discriminator) {
        case Discriminator.ChoiceRule:
        case Discriminator.SequenceRule:
            return true;
        default:
            return false;
    }
}

export class ChoiceRule {
    readonly discriminator = Discriminator.ChoiceRule;

    public choices: SequenceRule[];

    constructor(init: { choices: SequenceRule[] }) {
        this.choices = init.choices;
    }
}
export function isChoiceRule(
    value: ChoiceRule | SequenceRule | RuleReference | StringElement | CharSet | AnyElement
): value is ChoiceRule {
    return value.discriminator === Discriminator.ChoiceRule;
}

export class SequenceRule {
    readonly discriminator = Discriminator.SequenceRule;

    public elements: RuleElement[];

    constructor(init: { elements: RuleElement[] }) {
        this.elements = init.elements;
    }
}
export function isSequenceRule(
    value: ChoiceRule | SequenceRule | RuleReference | StringElement | CharSet | AnyElement
): value is SequenceRule {
    return value.discriminator === Discriminator.SequenceRule;
}

export type RuleElement = CountedRuleElement | NegativeLookahead;

export class CountedRuleElement {
    readonly discriminator = Discriminator.CountedRuleElement;

    public label: Label | undefined;
    public countableRuleElement: CountableRuleElement;
    public count: Count | undefined;
    public versionAnnotations: VersionAnnotation[];

    constructor(init: {
        label?: Label | undefined;
        countableRuleElement: CountableRuleElement;
        count?: Count | undefined;
        versionAnnotations: VersionAnnotation[];
    }) {
        this.label = init.label;
        this.countableRuleElement = init.countableRuleElement;
        this.count = init.count;
        this.versionAnnotations = init.versionAnnotations;
    }
}
export function isCountedRuleElement(value: CountedRuleElement | NegativeLookahead): value is CountedRuleElement {
    return value.discriminator === Discriminator.CountedRuleElement;
}

export type CountableRuleElement = RuleReference | StringElement | CharSet | AnyElement | RuleBody;

export enum CountEnum {
    OneOrMore = 'OneOrMore',
    ZeroOrMore = 'ZeroOrMore',
    Optional = 'Optional'
}
export class Count {
    static OneOrMore: Count = new Count(CountEnum.OneOrMore);
    static ZeroOrMore: Count = new Count(CountEnum.ZeroOrMore);
    static Optional: Count = new Count(CountEnum.Optional);

    private constructor(public readonly value: CountEnum) {}
}

export type Label = Name;

export type Name = Identifier;

export class RuleReference {
    readonly discriminator = Discriminator.RuleReference;

    public names: Name[];

    constructor(init: { names: Name[] }) {
        this.names = init.names;
    }
}
export function isRuleReference(
    value: RuleReference | StringElement | CharSet | AnyElement | ChoiceRule | SequenceRule
): value is RuleReference {
    return value.discriminator === Discriminator.RuleReference;
}

export class StringElement {
    readonly discriminator = Discriminator.StringElement;

    public value: string;

    constructor(init: { value: string }) {
        this.value = init.value;
    }
}
export function isStringElement(
    value: RuleReference | StringElement | CharSet | AnyElement | ChoiceRule | SequenceRule
): value is StringElement {
    return value.discriminator === Discriminator.StringElement;
}

export class CharSet {
    readonly discriminator = Discriminator.CharSet;

    public negated: boolean;
    public startChars: CharSetChar[];
    public endChars: (CharSetChar | undefined)[];

    constructor(init: { negated: boolean; startChars: CharSetChar[]; endChars: (CharSetChar | undefined)[] }) {
        this.negated = init.negated;
        this.startChars = init.startChars;
        this.endChars = init.endChars;
    }
}
export function isCharSet(
    value: RuleReference | StringElement | CharSet | AnyElement | ChoiceRule | SequenceRule
): value is CharSet {
    return value.discriminator === Discriminator.CharSet;
}

export type CharSetChar = string;

export class AnyElement {
    readonly discriminator = Discriminator.AnyElement;
}
export function isAnyElement(
    value: RuleReference | StringElement | CharSet | AnyElement | ChoiceRule | SequenceRule
): value is AnyElement {
    return value.discriminator === Discriminator.AnyElement;
}

export class NegativeLookahead {
    readonly discriminator = Discriminator.NegativeLookahead;

    public content: CharSet | StringElement;

    constructor(init: { content: CharSet | StringElement }) {
        this.content = init.content;
    }
}
export function isNegativeLookahead(value: CountedRuleElement | NegativeLookahead): value is NegativeLookahead {
    return value.discriminator === Discriminator.NegativeLookahead;
}

export type Identifier = string;

export type Trivia = LineComment | BlockComment | Whitespace;

export class LineComment {
    readonly discriminator = Discriminator.LineComment;

    public value: string;

    constructor(init: { value: string }) {
        this.value = init.value;
    }
}
export function isLineComment(value: LineComment | BlockComment | Whitespace): value is LineComment {
    return value.discriminator === Discriminator.LineComment;
}

export class BlockComment {
    readonly discriminator = Discriminator.BlockComment;

    public value: string;

    constructor(init: { value: string }) {
        this.value = init.value;
    }
}
export function isBlockComment(value: LineComment | BlockComment | Whitespace): value is BlockComment {
    return value.discriminator === Discriminator.BlockComment;
}

export class Whitespace {
    readonly discriminator = Discriminator.Whitespace;

    public value: string;

    constructor(init: { value: string }) {
        this.value = init.value;
    }
}
export function isWhitespace(value: LineComment | BlockComment | Whitespace): value is Whitespace {
    return value.discriminator === Discriminator.Whitespace;
}
