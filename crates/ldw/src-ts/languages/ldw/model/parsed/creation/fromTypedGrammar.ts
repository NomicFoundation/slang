import { Definition, Model } from '../model';
import { Grammar } from '../../../grammar/typed/model';

export class ParsedModelFromTypedGrammar {
    transform(input: Grammar): Model {
        return new Model({
            name: input.names,
            parentName: undefined,
            values: [
                ...input.rules.map((r) => new Definition({ name: r.name, type: r.type })),
                // ...input.prattRules.map(r => new Definition(r.name)),
                ...input.definitions
            ]
        });
    }
}
