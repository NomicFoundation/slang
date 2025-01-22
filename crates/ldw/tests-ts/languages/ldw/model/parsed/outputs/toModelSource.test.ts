import { ParseError } from '../../../../../src-ts/parsing/parseError';
import { ParsedModelFromSource } from '../../../../../src-ts/languages/ldw/model/parsed/creation/fromSource';
import { Model } from '../../../../../src-ts/languages/ldw/model/parsed/model';
import { ParsedModelToSource } from '../../../../../src-ts/languages/ldw/model/parsed/outputs/toSource';
import { modelsAreEqual } from '../../../../../src-ts/languages/ldw/model/parsed/util';
import { IncrementalModelGenerator } from '../../../../../src-ts/languages/ldw/model/parsed/creation/incrementalGenerator';

describe('ToLDWMSource', () => {
    it('should correctly transform and parse back incrementally generated models', () => {
        const iterations = 20;
        for (let i = 0; i < iterations; i++) {
            const generator = new IncrementalModelGenerator(100);
            for (const { model, change } of generator) {
                testModelTransformation(model, change);
            }
        }
    });
});

function testModelTransformation(model: Model, changeDescription: string) {
    const toLDWMSource = new ParsedModelToSource();
    const fromLDWMSource = new ParsedModelFromSource();

    let ldwmSource: string;
    let parsedModel: Model;

    try {
        ldwmSource = toLDWMSource.transform(model);
    } catch (error) {
        console.error(`Error transforming model to LDWM source after change: ${changeDescription}`);
        console.error(error);
        throw error;
    }

    try {
        parsedModel = fromLDWMSource.transform(ldwmSource);
    } catch (error) {
        console.error(`Parsing error after change: ${changeDescription}`);
        console.error('Generated LDWM Source:', ldwmSource);
        if (error instanceof ParseError) {
            console.error('Error:', error.toString());
        } else {
            console.error('Error:', error);
        }
        throw error;
    }

    const modelsEqual = modelsAreEqual(model, parsedModel, true);

    if (!modelsEqual) {
        console.error(`Models are not equal after change: ${changeDescription}`);
        console.error('Original Model:', JSON.stringify(model, null, 2));
        console.error('Generated LDWM Source:', ldwmSource);
        console.error('Parsed Model:', JSON.stringify(parsedModel, null, 2));
    }

    expect(modelsEqual).toBe(true);
}
