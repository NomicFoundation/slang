import { camelCase, pascalCase } from 'literal-case';
import * as Model from './model';

export function typeAsTypescript(type: Model.Type, prefix: string = ''): string {
    switch (type.discriminator) {
        case Model.Discriminator.VoidType:
            return 'void';
        case Model.Discriminator.PrimitiveType:
            switch (type.value) {
                case Model.PrimitiveTypeEnum.Boolean:
                    return 'boolean';
                case Model.PrimitiveTypeEnum.Char:
                case Model.PrimitiveTypeEnum.String:
                    return 'string';
                case Model.PrimitiveTypeEnum.I8:
                case Model.PrimitiveTypeEnum.I16:
                case Model.PrimitiveTypeEnum.I32:
                case Model.PrimitiveTypeEnum.I64:
                case Model.PrimitiveTypeEnum.U8:
                case Model.PrimitiveTypeEnum.U16:
                case Model.PrimitiveTypeEnum.U32:
                case Model.PrimitiveTypeEnum.U64:
                case Model.PrimitiveTypeEnum.F32:
                case Model.PrimitiveTypeEnum.F64:
                    return 'number';
            }
        case Model.Discriminator.EnumType:
            return type.members.join(' | ');
        case Model.Discriminator.SumType:
            return type.members.map((m) => typeAsTypescript(m, prefix)).join(' | ');
        case Model.Discriminator.ProductType:
            return `{ ${type.members.map((member) => `${member.name}: ${typeAsTypescript(member.type, prefix)}`).join(', ')} }`;
        case Model.Discriminator.MapType:
            return `Map<${typeAsTypescript(type.keyType, prefix)}, ${typeAsTypescript(type.valueType, prefix)}>`;
        case Model.Discriminator.SetType:
            return `Set<${typeAsTypescript(type.keyType, prefix)}>`;
        case Model.Discriminator.SequenceType:
            if (Model.isOptionType(type.elementType) || Model.isSumType(type.elementType)) {
                return `(${typeAsTypescript(type.elementType, prefix)})[]`;
            } else {
                return `${typeAsTypescript(type.elementType, prefix)}[]`;
            }
        case Model.Discriminator.OptionType:
            return `(${typeAsTypescript(type.type, prefix)} | undefined)`;
        case Model.Discriminator.NamedTypeReference:
            return `${prefix}${pascalCase(type.fqn[type.fqn.length - 1])}`;
    }
}

export function defaultInitializer(type: Model.Type): string | undefined {
    switch (type.discriminator) {
        case Model.Discriminator.VoidType:
            return;
        case Model.Discriminator.PrimitiveType:
            switch (type.value) {
                case Model.PrimitiveTypeEnum.Boolean:
                    return 'false';
                case Model.PrimitiveTypeEnum.Char:
                case Model.PrimitiveTypeEnum.String:
                    return "''";
                case Model.PrimitiveTypeEnum.I8:
                case Model.PrimitiveTypeEnum.I16:
                case Model.PrimitiveTypeEnum.I32:
                case Model.PrimitiveTypeEnum.I64:
                case Model.PrimitiveTypeEnum.U8:
                case Model.PrimitiveTypeEnum.U16:
                case Model.PrimitiveTypeEnum.U32:
                case Model.PrimitiveTypeEnum.U64:
                case Model.PrimitiveTypeEnum.F32:
                case Model.PrimitiveTypeEnum.F64:
                    return '0';
            }
        case Model.Discriminator.EnumType:
            return;
        case Model.Discriminator.SumType:
            return;
        case Model.Discriminator.ProductType:
            return `{ ${type.members.map((member) => `${camelCase(member.name)}: ${defaultInitializer(member.type)}`).join(', ')} }`;
        case Model.Discriminator.MapType:
            return `new Map()`;
        case Model.Discriminator.SetType:
            return `new Set()`;
        case Model.Discriminator.SequenceType:
            return `[]`;
        case Model.Discriminator.OptionType:
            return `undefined`;
        case Model.Discriminator.NamedTypeReference:
            return;
    }
}
