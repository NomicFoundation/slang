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
    ProductMember,
    ProductType,
    SequenceType,
    SetType,
    SumType,
    Type,
    VoidType
} from './model';

export function baseType(type: Type): Type {
    if (type instanceof SequenceType) {
        return type.elementType;
    } else if (type instanceof OptionType) {
        return baseType(type.type);
    }
    return type;
}

export function findFieldName(field: ProductMember): string {
    if (field.name) {
        return field.name;
    }

    // Generate a name based on the field's type
    const typeName = getTypeName(field.type);
    return `${typeName.toLowerCase()}Field`;
}

function getTypeName(type: Type): string {
    if (typeof type === 'string') {
        return type;
    } else if ('constructor' in type) {
        return type.constructor.name;
    } else {
        return 'UnknownType';
    }
}

export function typesAreEqual(type1: Type, type2: Type, debug: boolean = false): boolean {
    const checkAndLog = (condition: boolean, message: string): boolean => {
        if (debug && !condition) {
            console.log(message);
        }
        return condition;
    };

    // Helper function to get the type name
    const getTypeDiscriminator = (type: Type): string => {
        if (typeof type === 'string') return 'PrimitiveType';
        if (type instanceof VoidType) return 'VoidType';
        if (type instanceof EnumType) return 'EnumType';
        if (type instanceof SumType) return 'SumType';
        if (type instanceof ProductType) return 'ProductType';
        if (type instanceof MapType) return 'MapType';
        if (type instanceof SetType) return 'SetType';
        if (type instanceof SequenceType) return 'SequenceType';
        if (type instanceof OptionType) return 'OptionType';
        if (type instanceof NamedTypeReference) return 'NamedTypeReference';
        return 'Unknown';
    };

    const typeDiscriminator1 = getTypeDiscriminator(type1);
    const typeDiscriminator2 = getTypeDiscriminator(type2);

    if (
        !checkAndLog(
            typeDiscriminator1 === typeDiscriminator2,
            `Type mismatch: ${typeDiscriminator1} vs ${typeDiscriminator2}`
        )
    ) {
        return false;
    }

    switch (typeDiscriminator1) {
        case 'VoidType':
            return true;
        case 'PrimitiveType':
            return checkAndLog(type1 === type2, `Primitive type mismatch: ${type1} vs ${type2}`);
        case 'EnumType':
            return checkAndLog(
                JSON.stringify((type1 as EnumType).members) === JSON.stringify((type2 as EnumType).members),
                `Enum members mismatch: ${JSON.stringify((type1 as EnumType).members)} vs ${JSON.stringify((type2 as EnumType).members)}`
            );
        case 'SumType':
            return (type1 as SumType).members.every((m, i) =>
                checkAndLog(
                    typesAreEqual(m, (type2 as SumType).members[i], debug),
                    `Sum type member mismatch at index ${i}`
                )
            );
        case 'ProductType':
            return (type1 as ProductType).members.every((m, i) =>
                checkAndLog(
                    m.name === (type2 as ProductType).members[i].name &&
                        typesAreEqual(m.type, (type2 as ProductType).members[i].type, debug),
                    `Product type member mismatch at index ${i}: ${m.name} vs ${(type2 as ProductType).members[i].name}`
                )
            );
        case 'MapType':
            const mapType1 = type1 as MapType;
            const mapType2 = type2 as MapType;
            const keyEqual = typesAreEqual(mapType1.keyType, mapType2.keyType, debug);
            const valueEqual = typesAreEqual(mapType1.valueType, mapType2.valueType, debug);
            return checkAndLog(keyEqual, `Map key type mismatch`) && checkAndLog(valueEqual, `Map value type mismatch`);
        case 'SetType':
            return checkAndLog(
                typesAreEqual((type1 as SetType).keyType, (type2 as SetType).keyType, debug),
                `Set key type mismatch`
            );
        case 'SequenceType':
            return checkAndLog(
                typesAreEqual((type1 as SequenceType).elementType, (type2 as SequenceType).elementType, debug),
                `Sequence element type mismatch`
            );
        case 'OptionType':
            return checkAndLog(
                typesAreEqual((type1 as OptionType).type, (type2 as OptionType).type, debug),
                `Option type mismatch`
            );
        case 'NamedTypeReference':
            return checkAndLog(
                JSON.stringify((type1 as NamedTypeReference).fqn) === JSON.stringify((type2 as NamedTypeReference).fqn),
                `Named type reference mismatch: ${JSON.stringify((type1 as NamedTypeReference).fqn)} vs ${JSON.stringify((type2 as NamedTypeReference).fqn)}`
            );
        default:
            checkAndLog(false, `Unhandled type comparison: ${typeDiscriminator1}`);
            checkAndLog(false, `${JSON.stringify(type1, null, 2)} vs ${JSON.stringify(type2, null, 2)}`);
            return false;
    }
}

export function modelsAreEqual(model1: Model, model2: Model, debug: boolean = false): boolean {
    const checkAndLog = (condition: boolean, message: string): boolean => {
        if (debug && !condition) {
            console.log(message);
        }
        return condition;
    };

    const getValueDiscriminator = (value: Definition | Deletion | MemberModification): string => {
        if (value instanceof Definition) return 'Definition';
        if (value instanceof Deletion) return 'Deletion';
        if (value instanceof MemberModification) return 'MemberModification';
        return 'Unknown';
    };

    if (!checkAndLog(model1.name === model2.name, `Model name mismatch: ${model1.name} vs ${model2.name}`)) {
        return false;
    }

    if (
        !checkAndLog(
            model1.parentName === model2.parentName,
            `Model parent name mismatch: ${model1.parentName} vs ${model2.parentName}`
        )
    ) {
        return false;
    }

    if (
        !checkAndLog(
            model1.values.length === model2.values.length,
            `Model values length mismatch: ${model1.values.length} vs ${model2.values.length}`
        )
    ) {
        return false;
    }

    return model1.values.every((value, index) => {
        const value2 = model2.values[index];
        const valueDiscriminator1 = getValueDiscriminator(value);
        const valueDiscriminator2 = getValueDiscriminator(value2);

        if (
            !checkAndLog(
                valueDiscriminator1 === valueDiscriminator2,
                `Value type mismatch at index ${index}: ${valueDiscriminator1} vs ${valueDiscriminator2}`
            )
        ) {
            return false;
        }

        switch (valueDiscriminator1) {
            case 'Definition':
                const def1 = value as Definition;
                const def2 = value2 as Definition;
                return (
                    checkAndLog(
                        def1.name === def2.name,
                        `Definition name mismatch at index ${index}: ${def1.name} vs ${def2.name}`
                    ) &&
                    checkAndLog(
                        typesAreEqual(def1.type, def2.type, debug),
                        `Definition type mismatch at index ${index}`
                    )
                );
            case 'Deletion':
                const del1 = value as Deletion;
                const del2 = value2 as Deletion;
                return checkAndLog(
                    del1.name === del2.name,
                    `Deletion name mismatch at index ${index}: ${del1.name} vs ${del2.name}`
                );
            case 'MemberModification':
                const mod1 = value as MemberModification;
                const mod2 = value2 as MemberModification;
                if (
                    !checkAndLog(
                        mod1.name === mod2.name,
                        `MemberModification name mismatch at index ${index}: ${mod1.name} vs ${mod2.name}`
                    )
                ) {
                    return false;
                }
                if (
                    !checkAndLog(
                        mod1.values.length === mod2.values.length,
                        `MemberModification values length mismatch at index ${index}: ${mod1.values.length} vs ${mod2.values.length}`
                    )
                ) {
                    return false;
                }
                return mod1.values.every((v, i) => {
                    const v2 = mod2.values[i];
                    if (v instanceof MemberAddition && v2 instanceof MemberAddition) {
                        if (v.value instanceof ProductMember && v2.value instanceof ProductMember) {
                            return (
                                checkAndLog(
                                    v.value.name === v2.value.name,
                                    `MemberAddition value name mismatch at index ${index}, subindex ${i}: ${v.value.name} vs ${v2.value.name}`
                                ) &&
                                checkAndLog(
                                    typesAreEqual(v.value.type, v2.value.type, debug),
                                    `MemberAddition value type mismatch at index ${index}, subindex ${i}`
                                )
                            );
                        } else {
                            const vt1 = v.value as Type;
                            const vt2 = v.value as Type;
                            return checkAndLog(
                                typesAreEqual(vt1, vt2, debug),
                                `MemberAddition value mismatch at index ${index}, subindex ${i}`
                            );
                        }
                    } else if (v instanceof MemberDeletion && v2 instanceof MemberDeletion) {
                        return checkAndLog(
                            v.name === v2.name,
                            `MemberDeletion name mismatch at index ${index}, subindex ${i}: ${v.name} vs ${v2.name}`
                        );
                    } else {
                        return checkAndLog(
                            false,
                            `Unexpected member modification type at index ${index}, subindex ${i}`
                        );
                    }
                });
            default:
                return checkAndLog(false, `Unhandled value type at index ${index}: ${valueDiscriminator1}`);
        }
    });
}
