import * as Model from '../model';
import { InPlaceTransformer } from '../inPlaceTransformer';
import { singular } from 'pluralize';

export class RemoveAnonymousTypes {
    transform(input: Model.Model): Model.Model {
        return new AnonymousTypeRemover().transformModel(input);
    }
}

class AnonymousTypeRemover extends InPlaceTransformer {
    definitions: Model.Definition[] = [];
    depth: number = 0;
    baseName: string = '';

    transformModel(input: Model.Model): Model.Model {
        this.definitions.push(...input.definitions.values());
        // This makes sure that newly created types are then recursively visited
        // to remove any anonymous types they contain
        for (let i = 0; i < this.definitions.length; i++) {
            let definition = this.definitions[i];
            this.baseName = definition.name;
            this.definitions[i] = this.transformDefinition(definition);
        }
        input.definitions = new Map(this.definitions.map((d) => [d.name, d]));
        return input;
    }

    transformTypeToType(node: Model.TypeWithStructure): Model.Type {
        this.depth++;
        const result = super.transformTypeToType(node);
        this.depth--;
        return result;
    }

    transformEnumTypeToType(enumType: Model.EnumType): Model.Type {
        if (this.depth > 1) {
            // console.log('Removing internal EnumType', this.baseName, enumType);
            return this.createNewDefinition(enumType);
        } else {
            return super.transformEnumTypeToType(enumType);
        }
    }

    transformSumTypeToType(sumType: Model.SumType): Model.Type {
        if (this.depth > 1) {
            // console.log('Removing internal SumType', this.baseName, sumType);
            return this.createNewDefinition(sumType);
        } else {
            return super.transformSumTypeToType(sumType);
        }
    }

    transformProductTypeToType(productType: Model.ProductType): Model.Type {
        if (this.depth > 1) {
            // console.log('Removing internal ProductType', this.baseName, productType);
            return this.createNewDefinition(productType);
        } else {
            return super.transformProductTypeToType(productType);
        }
    }

    transformProductMember(input: Model.ProductMember): Model.ProductMember {
        const oldBaseName = this.baseName;
        this.baseName = `${this.baseName}_${singular(input.name)}`;
        const result = super.transformProductMember(input);
        this.baseName = oldBaseName;
        return result;
    }

    createNewDefinition(type: Model.Type): Model.Type {
        // TODO: disambiguate repeated names
        // let name = `${this.baseName}_${this.definitions.length}`;
        this.definitions.push(new Model.Definition({ name: this.baseName, type }));
        return new Model.NamedTypeReference({ fqn: [this.baseName] });
    }
}
