interface Nanopass<I, O> {
    transform(input: I): O;
}

type NanopassTuple = Nanopass<any, any>[];

type Head<T extends NanopassTuple> = T extends [infer H, ...any[]] ? H : never;
type Tail<T extends NanopassTuple> = T extends [any, ...infer R] ? R : never;

type ValidateNanopassChain<T extends NanopassTuple> = T extends []
    ? T
    : T extends [Nanopass<any, any>]
      ? T
      : Head<T> extends Nanopass<any, infer O1>
        ? Head<Tail<T>> extends Nanopass<infer I2, any>
            ? O1 extends I2
                ? [Head<T>, ...ValidateNanopassChain<Tail<T>>]
                : never
            : never
        : never;

type ChainInput<T extends NanopassTuple> = T extends [Nanopass<infer I, any>, ...any[]] ? I : never;

type ChainOutput<T extends NanopassTuple> = T extends [...any[], Nanopass<any, infer O>] ? O : never;

export function composePasses<T extends NanopassTuple>(
    ...passes: ValidateNanopassChain<T>
): Nanopass<ChainInput<T>, ChainOutput<T>> {
    return {
        transform: (input: ChainInput<T>): ChainOutput<T> => {
            return passes.reduce((value, transformer) => transformer.transform(value), input) as ChainOutput<T>;
        }
    };
}

export function dumpPassAsJSON<T>(): Nanopass<T, T> {
    return {
        transform: (input: T): T => {
            console.log(JSON.stringify(input, null, 2));
            return input;
        }
    };
}

export function repeatPass<T>(count: number, nanopass: Nanopass<T, T>): Nanopass<T, T> {
    return {
        transform: (input: T): T => {
            let output = input;
            for (let i = 0; i < count; i++) {
                output = nanopass.transform(output);
            }
            return output;
        }
    };
}

export function iterateUntilFixedPoint<T>(
    iterationLimit: number,
    logSuccesss: boolean,
    nanopass: Nanopass<T, T>
): Nanopass<T, T> {
    return {
        transform: (input: T): T => {
            const initialOutput = nanopass.transform(input);
            let fixedPoint = nanopass.transform(initialOutput);
            let numberOfIterations = 1;
            let fixedPointReached = JSON.stringify(initialOutput) !== JSON.stringify(fixedPoint);
            while (fixedPointReached && numberOfIterations < iterationLimit) {
                fixedPoint = nanopass.transform(fixedPoint);
                fixedPointReached = JSON.stringify(initialOutput) !== JSON.stringify(fixedPoint);
                numberOfIterations++;
            }
            if (!fixedPointReached || logSuccesss) {
                console.log(
                    '\n--------------------------------------------------------------------------------\n',
                    '    ',
                    fixedPointReached
                        ? `Fixed point NOT reached after ${numberOfIterations} iterations`
                        : `Fixed point reached after ${numberOfIterations} iterations`,
                    '\n--------------------------------------------------------------------------------\n'
                );
            }

            return fixedPoint;
        }
    };
}

export function optionalPass<T>(enable: boolean, nanopass: Nanopass<T, T>): Nanopass<T, T> {
    return {
        transform: (input: T): T => {
            if (enable) {
                return nanopass.transform(input);
            } else {
                return input;
            }
        }
    };
}

export function alternatePasses<I, O>(
    condition: boolean,
    truePass: Nanopass<I, O>,
    falsePass: Nanopass<I, O>
): Nanopass<I, O> {
    return {
        transform: (input: I): O => {
            if (condition) {
                return truePass.transform(input);
            } else {
                return falsePass.transform(input);
            }
        }
    };
}
