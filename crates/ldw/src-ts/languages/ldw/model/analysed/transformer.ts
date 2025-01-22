import * as In from '../resolved/model';
import * as Out from './model';

export class Transformer {
    transformModel(input: In.Model): Out.Model {
        const definitions = new Map<string, Out.Definition>();
        input.definitions.forEach((definition) => {
            definitions.set(definition.name, this.transformDefinition(definition));
        });
        return new Out.Model({
            name: input.name,
            parent: input.parent && this.transformModel(input.parent),
            definitions: definitions
        });
    }

    transformDefinition(input: In.Definition): Out.Definition {
        return new Out.Definition({
            name: input.name,
            type: this.transformType(input.type),
            discriminationPeers: undefined,
            discriminationMembers: undefined
        });
    }

    transformType(input: In.Type): Out.Type {
        if (input instanceof In.VoidType) {
            return this.transformVoidType(input);
        } else if (input instanceof In.PrimitiveType) {
            return this.transformPrimitiveType(input);
        } else if (input instanceof In.EnumType) {
            return this.transformEnumType(input);
        } else if (input instanceof In.NamedTypeReference) {
            return this.transformNamedTypeReference(input);
        } else if (input instanceof In.SumType) {
            return this.transformSumType(input);
        } else if (input instanceof In.ProductType) {
            return this.transformProductType(input);
        } else if (input instanceof In.MapType) {
            return this.transformMapType(input);
        } else if (input instanceof In.SetType) {
            return this.transformSetType(input);
        } else if (input instanceof In.SequenceType) {
            return this.transformSequenceType(input);
        } else {
            return this.transformOptionType(input);
        }
    }

    transformVoidType(input: In.VoidType): Out.VoidType {
        return new Out.VoidType();
    }

    transformPrimitiveType(input: In.PrimitiveType): Out.PrimitiveType {
        switch (input) {
            case In.PrimitiveType.Boolean:
                return Out.PrimitiveType.Boolean;
            case In.PrimitiveType.Char:
                return Out.PrimitiveType.Char;
            case In.PrimitiveType.String:
                return Out.PrimitiveType.String;
            case In.PrimitiveType.I8:
                return Out.PrimitiveType.I8;
            case In.PrimitiveType.I16:
                return Out.PrimitiveType.I16;
            case In.PrimitiveType.I32:
                return Out.PrimitiveType.I32;
            case In.PrimitiveType.I64:
                return Out.PrimitiveType.I64;
            case In.PrimitiveType.U8:
                return Out.PrimitiveType.U8;
            case In.PrimitiveType.U16:
                return Out.PrimitiveType.U16;
            case In.PrimitiveType.U32:
                return Out.PrimitiveType.U32;
            case In.PrimitiveType.U64:
                return Out.PrimitiveType.U64;
            case In.PrimitiveType.F32:
                return Out.PrimitiveType.F32;
            case In.PrimitiveType.F64:
                return Out.PrimitiveType.F64;
            default:
                throw new Error('Unexpected primitive type');
        }
    }

    transformEnumType(input: In.EnumType): Out.EnumType {
        return new Out.EnumType({ members: input.members });
    }

    transformNamedTypeReference(input: In.NamedTypeReference): Out.NamedTypeReference {
        return new Out.NamedTypeReference({ fqn: input.fqn });
    }

    transformSumType(input: In.SumType): Out.SumType {
        return new Out.SumType({ members: input.members.map((m) => this.transformType(m)) });
    }

    transformProductType(input: In.ProductType): Out.ProductType {
        return new Out.ProductType({ members: input.members.map((m) => this.transformProductMember(m)) });
    }

    transformProductMember(input: In.ProductMember): Out.ProductMember {
        return new Out.ProductMember({ name: input.name, type: this.transformType(input.type) });
    }

    transformMapType(input: In.MapType): Out.MapType {
        return new Out.MapType({
            keyType: this.transformType(input.keyType),
            valueType: this.transformType(input.valueType)
        });
    }

    transformSetType(input: In.SetType): Out.SetType {
        return new Out.SetType({ keyType: this.transformType(input.keyType) });
    }

    transformSequenceType(input: In.SequenceType): Out.SequenceType {
        return new Out.SequenceType({ elementType: this.transformType(input.elementType) });
    }

    transformOptionType(input: In.OptionType): Out.OptionType {
        return new Out.OptionType({ type: this.transformType(input.type) });
    }
}
