import * as In from '../../resolved/model';
import * as Out from '../model';
import { Transformer } from '../transformer';
import { Visitor } from '../visitor';

export class AnalysedModelFromResolvedModel extends Transformer {
    transform(input: In.Model): Out.Model {
        let result = this.transformModel(input);
        new AnalyseSumTypes(result.definitions).visitModel(result);
        return result;
    }
}

class AnalyseSumTypes extends Visitor {
    constructor(public definitions: Map<string, Out.Definition>) {
        super();
    }

    modelDepth = 0;
    visitModel(node: Out.Model): void {
        if (this.modelDepth == 0) {
            this.modelDepth++;
            super.visitModel(node);
            this.modelDepth--;
        }
    }

    visitDefinition(node: Out.Definition): void {
        if (node.type.discriminator === Out.Discriminator.SumType) {
            const nonterminalDefinitions = new Set<string>();
            const terminalDefinitions = new Set<string>();
            this.collectDefinitionsReachableFromSumType(node.type, nonterminalDefinitions, terminalDefinitions);
            node.discriminationMembers = terminalDefinitions;
        }

        super.visitDefinition(node);
    }

    visitSumType(sumType: Out.SumType): void {
        const nonterminalDefinitions = new Set<string>();
        const terminalDefinitions = new Set<string>();
        this.collectDefinitionsReachableFromSumType(sumType, nonterminalDefinitions, terminalDefinitions);
        const allDefinitions = new Set<string>();
        nonterminalDefinitions.forEach((n) => allDefinitions.add(n));
        terminalDefinitions.forEach((n) => allDefinitions.add(n));
        for (const name of allDefinitions) {
            const definition = this.definitions.get(name)!;
            if (!definition.discriminationPeers) definition.discriminationPeers = new Set();
            terminalDefinitions.forEach((n) => definition.discriminationPeers!.add(n));
        }
    }

    collectDefinitionsReachableFromSumType(
        sumType: Out.SumType,
        nonterminalDefinitions: Set<string>,
        terminalDefinitions: Set<string>
    ): void {
        sumType.members.forEach((member) => {
            if (member.discriminator !== Out.Discriminator.NamedTypeReference) {
                throw new Error('Not yet implemented: sum type discriminators on non-named types');
            }

            if (member.fqn.length !== 1) {
                throw new Error('Not yet implemented: sum type discriminators on foreign types');
            }
            const name = member.fqn[0];
            const definition = this.definitions.get(name)!;
            if (!definition) {
                throw new Error(`Analysed type ${name} not found`);
            }

            switch (definition.type.discriminator) {
                case Out.Discriminator.SumType:
                    if (!nonterminalDefinitions.has(name)) {
                        nonterminalDefinitions.add(name);
                        this.collectDefinitionsReachableFromSumType(
                            definition.type,
                            nonterminalDefinitions,
                            terminalDefinitions
                        );
                    }
                    break;
                case Out.Discriminator.ProductType:
                case Out.Discriminator.EnumType:
                    if (!terminalDefinitions.has(name)) {
                        terminalDefinitions.add(name);
                    }
                    break;
                default:
                    throw new Error('Not yet implemented: sum type discriminators on product/enum leaf types');
            }
        });
    }
}
