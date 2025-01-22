import { Registry } from '../../../../../../nanopass/registry';
import { Model } from '../../model';

export class AnalysedModelToRustTransformerSource {
    constructor(public registry: Registry) {}

    transform(model: Model): string {
        return '';
    }
}
