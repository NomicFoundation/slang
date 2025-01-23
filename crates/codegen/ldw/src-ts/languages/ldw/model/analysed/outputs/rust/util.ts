import { pascalCase } from 'literal-case';
import * as Model from '../../model';

export function typeAsRust(model: Model.Model, type: Model.Type, prefix: string = ''): string {
    switch (type.discriminator) {
        case Model.Discriminator.VoidType:
            return '()';
        case Model.Discriminator.PrimitiveType:
            switch (type.value) {
                case Model.PrimitiveTypeEnum.Boolean:
                    return 'boolean';
                case Model.PrimitiveTypeEnum.Char:
                    return 'char';
                case Model.PrimitiveTypeEnum.String:
                    return 'String';
                case Model.PrimitiveTypeEnum.I8:
                    return 'i8';
                case Model.PrimitiveTypeEnum.I16:
                    return 'i16';
                case Model.PrimitiveTypeEnum.I32:
                    return 'i32';
                case Model.PrimitiveTypeEnum.I64:
                    return 'i64';
                case Model.PrimitiveTypeEnum.U8:
                    return 'u8';
                case Model.PrimitiveTypeEnum.U16:
                    return 'u16';
                case Model.PrimitiveTypeEnum.U32:
                    return 'u32';
                case Model.PrimitiveTypeEnum.U64:
                    return 'u64';
                case Model.PrimitiveTypeEnum.F32:
                    return 'f32';
                case Model.PrimitiveTypeEnum.F64:
                    return 'f64';
            }
        case Model.Discriminator.EnumType:
            throw new Error('Not implemented');
        case Model.Discriminator.SumType:
            throw new Error('Not implemented');
        case Model.Discriminator.ProductType:
            throw new Error('Not implemented');
        case Model.Discriminator.MapType:
            return `std::collections::HashMap<${typeAsRust(model, type.keyType, prefix)}, ${typeAsRust(model, type.valueType, prefix)}>`;
        case Model.Discriminator.SetType:
            return `std::collections::HashSet<${typeAsRust(model, type.keyType, prefix)}>`;
        case Model.Discriminator.SequenceType:
            return `Vec<${typeAsRust(model, type.elementType, prefix)}>`;
        case Model.Discriminator.OptionType:
            return `Option<${typeAsRust(model, type.type, prefix)}>`;
        case Model.Discriminator.NamedTypeReference:
            if (type.fqn.length === 1) {
                if (typeRequiresBox(model, type)) {
                    return `${prefix}${pascalCase(type.fqn[0])}Ref`;
                } else {
                    return `${prefix}${pascalCase(type.fqn[0])}`;
                }
            } else {
                const namespace = type.fqn.slice(0, -1).join('::');
                return `${namespace}::${pascalCase(type.fqn[type.fqn.length - 1])}`;
            }
    }
}

export function typeAsRustDescription(type: Model.Type): string {
    switch (type.discriminator) {
        case Model.Discriminator.VoidType:
            return 'Void';
        case Model.Discriminator.PrimitiveType:
            switch (type.value) {
                case Model.PrimitiveTypeEnum.Boolean:
                    return 'Boolean';
                case Model.PrimitiveTypeEnum.Char:
                    return 'Char';
                case Model.PrimitiveTypeEnum.String:
                    return 'String';
                case Model.PrimitiveTypeEnum.I8:
                    return 'I8';
                case Model.PrimitiveTypeEnum.I16:
                    return 'I16';
                case Model.PrimitiveTypeEnum.I32:
                    return 'I32';
                case Model.PrimitiveTypeEnum.I64:
                    return 'I64';
                case Model.PrimitiveTypeEnum.U8:
                    return 'U8';
                case Model.PrimitiveTypeEnum.U16:
                    return 'U16';
                case Model.PrimitiveTypeEnum.U32:
                    return 'U32';
                case Model.PrimitiveTypeEnum.U64:
                    return 'U64';
                case Model.PrimitiveTypeEnum.F32:
                    return 'F32';
                case Model.PrimitiveTypeEnum.F64:
                    return 'F64';
            }
        case Model.Discriminator.EnumType:
        case Model.Discriminator.SumType:
        case Model.Discriminator.ProductType:
            throw new Error('Not yet implemented');
        case Model.Discriminator.MapType:
            return `MapOf${typeAsRustDescription(type.keyType)}To${typeAsRustDescription(type.valueType)}`;
        case Model.Discriminator.SetType:
            return `SetOf${typeAsRustDescription(type.keyType)}`;
        case Model.Discriminator.SequenceType:
            return `VecOf${typeAsRustDescription(type.elementType)}`;
        case Model.Discriminator.OptionType:
            return `Optional${typeAsRustDescription(type.type)}`;
        case Model.Discriminator.NamedTypeReference:
            return pascalCase(type.fqn.join('_'));
    }
}

const RUST_KEYWORDS = new Set([
    'abstract',
    'alignof',
    'as',
    'become',
    'box',
    'break',
    'const',
    'continue',
    'crate',
    'do',
    'else',
    'enum',
    'extern',
    'false',
    'final',
    'fn',
    'for',
    'if',
    'impl',
    'in',
    'let',
    'loop',
    'macro',
    'match',
    'mod',
    'move',
    'mut',
    'offsetof',
    'override',
    'priv',
    'proc',
    'pub',
    'pure',
    'ref',
    'return',
    'Self',
    'self',
    'sizeof',
    'static',
    'struct',
    'super',
    'trait',
    'true',
    'type',
    'typeof',
    'unsafe',
    'unsized',
    'use',
    'virtual',
    'where',
    'while',
    'yield'
]);

export function rustName(name: string): string {
    if (RUST_KEYWORDS.has(name)) {
        return `r#${name}`;
    }
    return name;
}

export function typeRequiresBox(model: Model.Model, type: Model.Type): boolean {
    return (
        Model.isNamedTypeReference(type) &&
        (type.fqn.length > 1 || Model.isProductType(model.definitions.get(type.fqn[type.fqn.length - 1])!.type))
    );
}
