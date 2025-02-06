import {
    Definition,
    Deletion,
    EnumType,
    MapType,
    MemberAddition,
    MemberDeletion,
    MemberModification,
    Model,
    NamedTypeReference,
    OptionType,
    PrimitiveType,
    ProductMember,
    ProductType,
    SequenceType,
    SetType,
    SumType,
    Type,
    VoidType
} from '../model';

function generateRandomString(length: number = 5): string {
    const alphabet = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const validChars = alphabet + '0123456789_';
    let result = alphabet[Math.floor(Math.random() * alphabet.length)];
    for (let i = 1; i < length; i++) {
        result += validChars[Math.floor(Math.random() * validChars.length)];
    }
    return result;
}

function generateComplexType(depth: number = 0): Type {
    if (depth >= 2) {
        return generatePrimitiveType();
    }

    const types: (() => Type)[] = [
        () => new VoidType(),
        generatePrimitiveType,
        () => new EnumType({ members: [generateRandomString(), generateRandomString()] }),
        () => new SumType({ members: [generateComplexType(depth + 1), generateComplexType(depth + 1)] }),
        () =>
            new ProductType({
                members: [
                    new ProductMember({ name: generateRandomString(), type: generateComplexType(depth + 1) }),
                    new ProductMember({ name: generateRandomString(), type: generateComplexType(depth + 1) })
                ]
            }),
        () => new MapType({ keyType: generateComplexType(depth + 1), valueType: generateComplexType(depth + 1) }),
        () => new SetType({ keyType: generateComplexType(depth + 1) }),
        () => new SequenceType({ elementType: generateComplexType(depth + 1) }),
        () => new OptionType({ type: generateComplexType(depth + 1) }),
        () => new NamedTypeReference({ fqn: [generateRandomString()] })
    ];

    return types[Math.floor(Math.random() * types.length)]();
}

function generatePrimitiveType(): PrimitiveType {
    const primitives: PrimitiveType[] = [
        PrimitiveType.Boolean,
        PrimitiveType.Char,
        PrimitiveType.String,
        PrimitiveType.I8,
        PrimitiveType.I16,
        PrimitiveType.I32,
        PrimitiveType.I64,
        PrimitiveType.U8,
        PrimitiveType.U16,
        PrimitiveType.U32,
        PrimitiveType.U64,
        PrimitiveType.F32,
        PrimitiveType.F64
    ];
    return primitives[Math.floor(Math.random() * primitives.length)];
}

class CoverageTracker {
    private coveredTypes: Set<string> = new Set();

    markTypeCovered(type: string): void {
        this.coveredTypes.add(type);
    }

    getUncoveredTypes(): string[] {
        const allTypes = [
            'Definition',
            'Deletion',
            'MemberModification',
            'VoidType',
            'PrimitiveType',
            'EnumType',
            'SumType',
            'ProductType',
            'MapType',
            'SetType',
            'SequenceType',
            'OptionType',
            'ResultType',
            'NamedTypeReference'
        ];
        return allTypes.filter((type) => !this.coveredTypes.has(type));
    }

    isFullyCovered(): boolean {
        return this.getUncoveredTypes().length === 0;
    }
}

export class IncrementalModelGenerator implements IterableIterator<{ model: Model; change: string }> {
    private currentModel: Model;
    private changeHistory: string[];
    private coverageTracker: CoverageTracker;
    private stepCount: number;

    constructor(private maxSteps: number = 50) {
        this.currentModel = new Model({ name: ['root'], values: [] });
        this.changeHistory = ['Initial empty model'];
        this.coverageTracker = new CoverageTracker();
        this.stepCount = 0;
    }

    next(): IteratorResult<{ model: Model; change: string }> {
        if (this.stepCount >= this.maxSteps || this.coverageTracker.isFullyCovered()) {
            return { done: true, value: undefined };
        }

        this.generateNextChange();
        this.stepCount++;

        return {
            done: false,
            value: {
                model: this.getCurrentModel(),
                change: this.getLastChange()
            }
        };
    }

    [Symbol.iterator](): IterableIterator<{ model: Model; change: string }> {
        return this;
    }

    private generateNextChange(): void {
        const uncoveredTypes = this.coverageTracker.getUncoveredTypes();
        if (uncoveredTypes.length > 0) {
            const typeToGenerate = uncoveredTypes[Math.floor(Math.random() * uncoveredTypes.length)];
            switch (typeToGenerate) {
                case 'Definition':
                    this.addDefinition();
                    break;
                case 'Deletion':
                    this.addDeletion();
                    break;
                case 'MemberModification':
                    this.addMemberModification();
                    break;
                default:
                    this.addDefinition(typeToGenerate);
            }
        } else {
            // If all types are covered, randomly choose an action
            const actions = [this.addDefinition, this.addDeletion, this.addMemberModification];
            const action = actions[Math.floor(Math.random() * actions.length)];
            action.call(this);
        }
    }

    private addDefinition(typeKind?: string): void {
        const type = typeKind ? generateComplexType() : generateComplexType();
        const name = generateRandomString();
        const newDefinition = new Definition({ name, type });
        this.currentModel.values.push(newDefinition);
        this.changeHistory.push(`Added definition: ${name} (${this.typeToString(type)})`);
        this.coverageTracker.markTypeCovered('Definition');
        if (typeKind) this.coverageTracker.markTypeCovered(typeKind);
    }

    private addDeletion(): void {
        const name = generateRandomString();
        const newDeletion = new Deletion({ name });
        this.currentModel.values.push(newDeletion);
        this.changeHistory.push(`Added deletion: ${name}`);
        this.coverageTracker.markTypeCovered('Deletion');
    }

    private addMemberModification(): void {
        const name = generateRandomString();
        const memberModification = new MemberModification({ name, values: [] });

        // Add a MemberAddition
        const newMember = new ProductMember({ name: generateRandomString(), type: generateComplexType() });
        memberModification.values.push(new MemberAddition({ value: newMember }));

        // Add a MemberDeletion
        memberModification.values.push(new MemberDeletion({ name: generateRandomString() }));

        this.currentModel.values.push(memberModification);
        this.changeHistory.push(`Added member modification: ${name}`);
        this.coverageTracker.markTypeCovered('MemberModification');
    }

    private getCurrentModel(): Model {
        return this.currentModel;
    }

    private getLastChange(): string {
        return this.changeHistory[this.changeHistory.length - 1];
    }

    private typeToString = (type: Type): string => {
        if (typeof type === 'string') return type;
        if (type instanceof VoidType) return '()';
        if (type instanceof ProductType)
            return `{ ${type.members.map((m) => `${m.name}: ${this.typeToString(m.type)}`).join(' ')} }`;
        if (type instanceof SumType) return `{ ${type.members.map(this.typeToString).join(' | ')} }`;
        if (type instanceof SequenceType) return `seq<${this.typeToString(type.elementType)}>`;
        if (type instanceof MapType)
            return `map<${this.typeToString(type.keyType)} ${this.typeToString(type.valueType)}>`;
        if (type instanceof SetType) return `set<${this.typeToString(type.keyType)}>`;
        if (type instanceof OptionType) return `option<${this.typeToString(type.type)}>`;
        if (type instanceof EnumType) return `{ ${type.members.map((m) => `"${m}"`).join(' | ')} }`;
        if (type instanceof NamedTypeReference) return type.fqn.join('::');
        return 'unknown';
    };
}
