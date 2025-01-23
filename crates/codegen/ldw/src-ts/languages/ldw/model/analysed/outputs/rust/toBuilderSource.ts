import { Registry } from '../../../../../../nanopass/registry';
import * as Model from '../../model';

export class AnalysedModelToRustBuilderSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        return '';
    }
}
