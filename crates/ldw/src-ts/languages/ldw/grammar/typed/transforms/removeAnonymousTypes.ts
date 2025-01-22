import {
    Definition,
    EnumType,
    NamedTypeReference,
    ProductType,
    SumType,
    Type,
    TypeWithStructure
} from '../../../model/parsed/model';
import { Visitor as ModelVisitor } from '../../../model/parsed/visitor';
import { Grammar } from '../model';

export class RemoveAnonymousTypes {
    transform(input: Grammar): Grammar {
        for (const rule of input.rules) {
            const typeRemover = new TypeRemover(rule.name);
            typeRemover.visitType(rule.type);
            input.definitions.push(...typeRemover.definitions);
        }
        return input;
    }
}

class TypeRemover extends ModelVisitor {
    public definitions: Definition[] = [];
    public depth: number = 0;

    constructor(private baseName: string) {
        super();
    }

    visitTypeWithStructure(typeWithStructure: TypeWithStructure): void {
        this.depth++;
        // traverser.next();
        this.depth--;
    }

    visitEnumType(enumType: EnumType): void | Type {
        if (this.depth > 1) {
            let name = `${this.baseName}_${this.definitions.length}`;
            this.definitions.push(new Definition({ name, type: enumType }));
            return new NamedTypeReference({ fqn: [name] });
        }
    }

    visitSumType(sumType: SumType): void | Type {
        if (this.depth > 1) {
            console.log('Removing internal SumType', this.baseName, sumType);
            let name = `${this.baseName}_${this.definitions.length}`;
            this.definitions.push(new Definition({ name, type: sumType }));
            return new NamedTypeReference({ fqn: [name] });
        }
    }

    visitProductType(productType: ProductType): void | Type {
        if (this.depth > 1) {
            let name = `${this.baseName}_${this.definitions.length}`;
            this.definitions.push(new Definition({ name, type: productType }));
            return new NamedTypeReference({ fqn: [name] });
        }
    }
}
