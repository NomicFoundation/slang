import * as Model from './model';

export class InPlaceTransformer {
    transformModel(input: Model.Model): Model.Model {
        throw new Error('Automatic transformation not possible');
    }

    transformDefinition(input: Model.Definition): Model.Definition {
        return new Model.Definition({ name: input.name, type: this.transformTypeToType(input.type) });
    }

    // Each `...ToType` form is per-context in which the type is found

    transformTypeToType(input: Model.Type): Model.Type {
        if (input instanceof Model.VoidType) {
            return this.transformVoidTypeToType(input);
        } else if (input instanceof Model.PrimitiveType) {
            return this.transformPrimitiveTypeToType(input);
        } else if (input instanceof Model.EnumType) {
            return this.transformEnumTypeToType(input);
        } else if (input instanceof Model.NamedTypeReference) {
            return this.transformNamedTypeReferenceToType(input);
        } else if (input instanceof Model.SumType) {
            return this.transformSumTypeToType(input);
        } else if (input instanceof Model.ProductType) {
            return this.transformProductTypeToType(input);
        } else if (input instanceof Model.MapType) {
            return this.transformMapTypeToType(input);
        } else if (input instanceof Model.SetType) {
            return this.transformSetTypeToType(input);
        } else if (input instanceof Model.SequenceType) {
            return this.transformSequenceTypeToType(input);
        } else {
            return this.transformOptionTypeToType(input);
        }
    }

    // When this is generated, it should follow the type hierarchy and include the following:
    // transformdTypeWithStructureToType(input: Model.StructType): Model.Type
    // transformGenericTypeToType(input: Model.GenericType): Model.Type

    transformVoidTypeToType(input: Model.VoidType): Model.Type {
        return input;
    }

    transformPrimitiveTypeToType(input: Model.PrimitiveType): Model.Type {
        return input;
    }

    transformEnumTypeToType(input: Model.EnumType): Model.Type {
        return input;
    }

    transformNamedTypeReferenceToType(input: Model.NamedTypeReference): Model.Type {
        return input;
    }

    transformSumTypeToType(input: Model.SumType): Model.Type {
        return new Model.SumType({ members: input.members.map((m) => this.transformTypeToType(m)) });
    }

    transformProductTypeToType(input: Model.ProductType): Model.Type {
        return new Model.ProductType({ members: input.members.map((m) => this.transformProductMember(m)) });
    }

    transformProductMember(input: Model.ProductMember): Model.ProductMember {
        return new Model.ProductMember({ name: input.name, type: this.transformTypeToType(input.type) });
    }

    transformMapTypeToType(input: Model.MapType): Model.Type {
        return new Model.MapType({
            keyType: this.transformTypeToType(input.keyType),
            valueType: this.transformTypeToType(input.valueType)
        });
    }

    transformSetTypeToType(input: Model.SetType): Model.Type {
        return new Model.SetType({ keyType: this.transformTypeToType(input.keyType) });
    }

    transformSequenceTypeToType(input: Model.SequenceType): Model.Type {
        return new Model.SequenceType({ elementType: this.transformTypeToType(input.elementType) });
    }

    transformOptionTypeToType(input: Model.OptionType): Model.Type {
        return new Model.OptionType({ type: this.transformTypeToType(input.type) });
    }
}
