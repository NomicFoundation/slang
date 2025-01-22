import { Registry } from '../../../../../nanopass/registry';
import * as Model from '../model';
import { Visitor } from '../visitor';

export function simplifiedType(type: Model.Type, model: Model.Model): Model.Type {
    switch (type.discriminator) {
        case Model.Discriminator.SumType:
            if (type.members.length === 1) {
                return simplifiedType(type.members[0], model);
            }
            break;
        case Model.Discriminator.ProductType:
            if (type.members.length === 1) {
                return simplifiedType(type.members[0].type, model);
            } else {
                return new Model.ProductType({
                    members: type.members.map(
                        (m) => new Model.ProductMember({ name: m.name, type: simplifiedType(m.type, model) })
                    )
                });
            }
            break;
        case Model.Discriminator.NamedTypeReference:
            if (type.fqn.length === 1) {
                const definition = model.definitions.get(type.fqn[0]);
                if (definition && !Model.isSumType(definition.type)) {
                    return simplifiedType(definition.type, model);
                }
            }
            break;
        case Model.Discriminator.OptionType:
            return new Model.OptionType({ type: simplifiedType(type.type, model) });
        case Model.Discriminator.MapType:
            return new Model.MapType({
                keyType: simplifiedType(type.keyType, model),
                valueType: simplifiedType(type.valueType, model)
            });
        case Model.Discriminator.SetType:
            return new Model.SetType({ keyType: simplifiedType(type.keyType, model) });
        case Model.Discriminator.SequenceType:
            return new Model.SequenceType({ elementType: simplifiedType(type.elementType, model) });
        default:
            break;
    }
    return type;
}

export function simplifiedAtomicTypes(type: Model.Type, model: Model.Model): Model.Type[] {
    switch (type.discriminator) {
        case Model.Discriminator.VoidType:
        case Model.Discriminator.PrimitiveType:
        case Model.Discriminator.EnumType:
            return [type];
        case Model.Discriminator.SumType:
            return type.members.map((m) => simplifiedAtomicTypes(m, model)).flat();
        case Model.Discriminator.ProductType:
            return [type];
        case Model.Discriminator.MapType:
            return [type.keyType, type.valueType].map((m) => simplifiedAtomicTypes(m, model)).flat();
        case Model.Discriminator.SetType:
            return simplifiedAtomicTypes(type.keyType, model).map((t) => new Model.SetType({ keyType: t }));
        case Model.Discriminator.SequenceType:
            return simplifiedAtomicTypes(type.elementType, model).map(
                (t) => new Model.SequenceType({ elementType: t })
            );
        case Model.Discriminator.OptionType:
            return simplifiedAtomicTypes(type.type, model)
                .map((t) => [new Model.OptionType({ type: t }), t])
                .flat();
        case Model.Discriminator.NamedTypeReference:
            // if (type.fqn.length === 1) {
            //     const definition = model.definitions.get(type.fqn[0]);
            //     if (definition && !Model.isSumType(definition.type) && !Model.isProductType(definition.type)) {
            //         return simplifiedAtomicTypes(definition.type, model);
            //     }
            // }
            return [type];
    }
}

export function forEachForeignNamespace(
    model: Model.Model,
    registry: Registry,
    callback: (namespace: string[]) => void
): void {
    const namespaceStrings = new Set<string>();
    new (class extends Visitor {
        visitNamedTypeReference(node: Model.NamedTypeReference): void {
            if (node.fqn.length > 1) {
                const namespace = node.fqn.slice(0, -1);
                const namespaceString = namespace.join('::');
                if (!namespaceStrings.has(namespaceString)) {
                    namespaceStrings.add(namespaceString);
                    callback(namespace);
                }
            }
        }
    })().visitModel(model);
}

export function forEachForeignEntity(
    model: Model.Model,
    callback: (namespace: string[], entity: string) => void
): void {
    const foreignFQNs = new Set<string>();
    new (class extends Visitor {
        visitNamedTypeReference(node: Model.NamedTypeReference): void {
            if (node.fqn.length > 1) {
                const foreignFQN = node.fqn.join('::');
                if (!foreignFQNs.has(foreignFQN)) {
                    foreignFQNs.add(foreignFQN);
                    const namespace = node.fqn.slice(0, -1);
                    const entity = node.fqn[node.fqn.length - 1];
                    callback(namespace, entity);
                }
            }
        }
    })().visitModel(model);
}
