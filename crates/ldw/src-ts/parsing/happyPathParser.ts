import { Builder, Stack } from './builder';
import { InputStream, LineAndColumn } from './inputStream';

export abstract class ParserBase implements InputStream {
    private skipTriviaEnabled: boolean = true;

    builder: Builder = new Builder();

    constructor(protected input: InputStream) {}

    skipOptional(predicate: () => boolean): boolean {
        predicate();
        return true;
    }

    skipZeroOrMore(predicate: () => boolean): boolean {
        while (predicate());
        return true;
    }

    skipOneOrMore(predicate: () => boolean): boolean {
        if (!predicate()) return false;
        while (predicate());
        return true;
    }

    skipSeq(predicate: () => boolean): boolean {
        const position = this.mark();
        const depth = this.builder.mark();
        try {
            return predicate();
        } finally {
            this.builder.restore(depth);
            this.restore(position);
        }
    }

    skipNegativeLookahead(predicate: () => boolean): boolean {
        const position = this.mark();
        const depth = this.builder.mark();
        if (predicate()) {
            this.builder.restore(depth);
            this.restore(position);
            return false;
        }
        return true;
    }

    skipTrivia(predicate: () => boolean): boolean {
        const position = this.mark();
        const depth = this.builder.mark();
        if (this.skipTriviaEnabled) {
            while (true) {
                const kind = this.consumeTrivia();
                if (kind === undefined) break;
            }
        }
        try {
            return predicate();
        } finally {
            this.builder.restore(depth);
            this.restore(position);
        }
    }

    ignoreSkipTriviaDuring<T>(predicate: () => boolean): boolean {
        if (this.skipTriviaEnabled) {
            while (true) {
                const kind = this.consumeTrivia();
                if (kind === undefined) break;
            }
            this.skipTriviaEnabled = false;
            try {
                return predicate();
            } finally {
                this.skipTriviaEnabled = true;
            }
        } else {
            return predicate();
        }
    }

    buildBoolean(label: string | undefined, parser: () => boolean): boolean {
        if (!parser()) return false;
        this.builder.push(label, true);
        return true;
    }

    buildString(label: string | undefined, parser: () => boolean): boolean {
        const pos = this.mark();
        if (!parser()) return false;
        this.builder.push(label, this.makeString(pos, this.mark()));
        return true;
    }

    buildStringObject<T>(
        label: string | undefined,
        factory: { new (init: { value: string }): T },
        parser: () => boolean
    ): boolean {
        const pos = this.mark();
        if (!parser()) return false;
        this.builder.push(label, new factory({ value: this.makeString(pos, this.mark()) }));
        return true;
    }

    buildEnum<T>(label: string | undefined, ...values: [string, T][]): boolean {
        for (const [text, value] of values) {
            if (this.skipString(text)) {
                this.builder.push(label, value);
                return true;
            }
        }
        return false;
    }

    buildObject<T>(label: string | undefined, buildFunction: (stack: Stack) => T, parser: () => boolean): boolean {
        const depth = this.builder.mark();
        if (parser()) {
            this.builder.create(depth, label, buildFunction);
            return true;
        } else {
            return false;
        }
    }

    protected abstract consumeTrivia(): string | undefined;

    protected abstract consumeIdentifierForKeyword(): string | undefined;

    mark(): number {
        return this.input.mark();
    }

    restore(position: number): void {
        if (position > this.input.mark()) throw new Error('Cannot restore to a future position');
        this.input.restore(position);
    }

    mark(): number {
        return this.input.mark();
    }

    positionToLineAndColumn(position: number): LineAndColumn {
        return this.input.positionToLineAndColumn(position);
    }

    peek(): string | undefined {
        return this.input.peek();
    }

    isEOF(): boolean {
        return this.input.isEOF();
    }

    skip(count?: number): boolean {
        return this.input.skip(count);
    }

    skipString(str: string): boolean {
        return this.input.skipString(str);
    }

    skipRegex(regex: RegExp): boolean {
        return this.input.skipRegex(regex);
    }

    skipWhile(predicate: (char: string) => boolean): boolean {
        return this.input.skipWhile(predicate);
    }

    makeString(from: number, to: number): string {
        return this.input.makeString(from, to);
    }
}
