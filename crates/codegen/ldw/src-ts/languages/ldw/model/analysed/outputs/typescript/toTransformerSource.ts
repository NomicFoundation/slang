import { Registry } from '../../../../../../nanopass/registry';
import * as Model from '../../model';

export class AnalysedModelToTypescriptTransformerSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        return '';
    }
}
