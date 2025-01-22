// Generated on 2024-10-15T17:24:00.356Z
export enum Discriminator {
    VoidType = 'VoidType',
    PrimitiveType = 'PrimitiveType',
    EnumType = 'EnumType',
    SumType = 'SumType',
    ProductType = 'ProductType',
    MapType = 'MapType',
    SetType = 'SetType',
    SequenceType = 'SequenceType',
    OptionType = 'OptionType',
    NamedTypeReference = 'NamedTypeReference',
    Whitespace = 'Whitespace',
    LineComment = 'LineComment',
    BlockComment = 'BlockComment'
}

export class Model {
    public name: Fqn;
    public parent: Model | undefined;
    public definitions: Map<string, Definition>;

    constructor(init: { name: Fqn; parent?: Model | undefined; definitions: Map<string, Definition> }) {
        this.name = init.name;
        this.parent = init.parent;
        this.definitions = init.definitions;
    }
}

export type Fqn = Id[];

export class Definition {
    public name: Id;
    public type: Type;
    public discriminationPeers: Set<string> | undefined;
    public discriminationMembers: Set<string> | undefined;

    constructor(init: {
        name: Id;
        type: Type;
        discriminationPeers?: Set<string> | undefined;
        discriminationMembers?: Set<string> | undefined;
    }) {
        this.name = init.name;
        this.type = init.type;
        this.discriminationPeers = init.discriminationPeers;
        this.discriminationMembers = init.discriminationMembers;
    }
}

export type Type = VoidType | PrimitiveType | EnumType | TypeWithStructure | NamedTypeReference;

export class VoidType {
    readonly discriminator = Discriminator.VoidType;
}
export function isVoidType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is VoidType {
    return value.discriminator === Discriminator.VoidType;
}

export enum PrimitiveTypeEnum {
    Boolean = 'Boolean',
    Char = 'Char',
    String = 'String',
    I8 = 'I8',
    I16 = 'I16',
    I32 = 'I32',
    I64 = 'I64',
    U8 = 'U8',
    U16 = 'U16',
    U32 = 'U32',
    U64 = 'U64',
    F32 = 'F32',
    F64 = 'F64'
}
export class PrimitiveType {
    readonly discriminator = Discriminator.PrimitiveType;

    static Boolean: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.Boolean);
    static Char: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.Char);
    static String: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.String);
    static I8: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.I8);
    static I16: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.I16);
    static I32: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.I32);
    static I64: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.I64);
    static U8: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.U8);
    static U16: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.U16);
    static U32: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.U32);
    static U64: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.U64);
    static F32: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.F32);
    static F64: PrimitiveType = new PrimitiveType(PrimitiveTypeEnum.F64);

    private constructor(public readonly value: PrimitiveTypeEnum) {}
}
export function isPrimitiveType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is PrimitiveType {
    return value.discriminator === Discriminator.PrimitiveType;
}

export class EnumType {
    readonly discriminator = Discriminator.EnumType;

    public members: StringElement[];

    constructor(init: { members: StringElement[] }) {
        this.members = init.members;
    }
}
export function isEnumType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is EnumType {
    return value.discriminator === Discriminator.EnumType;
}

export type StringElement = Id;

export type TypeWithStructure = SumType | ProductType | GenericType;
export function isTypeWithStructure(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is TypeWithStructure {
    switch (value.discriminator) {
        case Discriminator.SumType:
        case Discriminator.ProductType:
        case Discriminator.MapType:
        case Discriminator.SetType:
        case Discriminator.SequenceType:
        case Discriminator.OptionType:
            return true;
        default:
            return false;
    }
}

export class SumType {
    readonly discriminator = Discriminator.SumType;

    public members: Type[];

    constructor(init: { members: Type[] }) {
        this.members = init.members;
    }
}
export function isSumType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is SumType {
    return value.discriminator === Discriminator.SumType;
}

export class ProductType {
    readonly discriminator = Discriminator.ProductType;

    public members: ProductMember[];

    constructor(init: { members: ProductMember[] }) {
        this.members = init.members;
    }
}
export function isProductType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is ProductType {
    return value.discriminator === Discriminator.ProductType;
}

export class ProductMember {
    public name: Id;
    public type: Type;

    constructor(init: { name: Id; type: Type }) {
        this.name = init.name;
        this.type = init.type;
    }
}

export type GenericType = MapType | SetType | SequenceType | OptionType;
export function isGenericType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is GenericType {
    switch (value.discriminator) {
        case Discriminator.MapType:
        case Discriminator.SetType:
        case Discriminator.SequenceType:
        case Discriminator.OptionType:
            return true;
        default:
            return false;
    }
}

export class MapType {
    readonly discriminator = Discriminator.MapType;

    public keyType: Type;
    public valueType: Type;

    constructor(init: { keyType: Type; valueType: Type }) {
        this.keyType = init.keyType;
        this.valueType = init.valueType;
    }
}
export function isMapType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is MapType {
    return value.discriminator === Discriminator.MapType;
}

export class SetType {
    readonly discriminator = Discriminator.SetType;

    public keyType: Type;

    constructor(init: { keyType: Type }) {
        this.keyType = init.keyType;
    }
}
export function isSetType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is SetType {
    return value.discriminator === Discriminator.SetType;
}

export class SequenceType {
    readonly discriminator = Discriminator.SequenceType;

    public elementType: Type;

    constructor(init: { elementType: Type }) {
        this.elementType = init.elementType;
    }
}
export function isSequenceType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is SequenceType {
    return value.discriminator === Discriminator.SequenceType;
}

export class OptionType {
    readonly discriminator = Discriminator.OptionType;

    public type: Type;

    constructor(init: { type: Type }) {
        this.type = init.type;
    }
}
export function isOptionType(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is OptionType {
    return value.discriminator === Discriminator.OptionType;
}

export class NamedTypeReference {
    readonly discriminator = Discriminator.NamedTypeReference;

    public fqn: Fqn;

    constructor(init: { fqn: Fqn }) {
        this.fqn = init.fqn;
    }
}
export function isNamedTypeReference(
    value:
        | VoidType
        | PrimitiveType
        | EnumType
        | SumType
        | ProductType
        | MapType
        | SetType
        | SequenceType
        | OptionType
        | NamedTypeReference
): value is NamedTypeReference {
    return value.discriminator === Discriminator.NamedTypeReference;
}

export type Id = Identifier;

export type Identifier = string;

export type InitialIdentifierChar = string;

export type IdentifierChar = string;

export type Trivia = LineComment | BlockComment | Whitespace;

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
