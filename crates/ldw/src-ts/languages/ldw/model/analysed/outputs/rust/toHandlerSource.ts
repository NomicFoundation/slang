import { Registry } from '../../../../../../nanopass/registry';
import * as Model from '../../model';
import { RustGenericHandlerGenerator } from './genericHandlerGenerator';

export class AnalysedModelToRustHandlerSource {
    constructor(public registry: Registry) {}

    transform(model: Model.Model): string {
        return new RustHandlerGenerator(model, this.registry).generate();
    }
}

class RustHandlerGenerator extends RustGenericHandlerGenerator {
    traitName(): string {
        return 'Handler';
    }

    visitProductType(): void {}
    visitGenericType(node: Model.GenericType): void {}
}
