import * as Model from '../model';

export class TypedGrammarToRustParserSource {
    constructor(public roots: string[]) {}

    transform(grammar: Model.Grammar): string {
        return '';
    }
}
