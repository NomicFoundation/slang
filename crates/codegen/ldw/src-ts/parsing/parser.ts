/**
 * @file parser.ts
 * @description Core parsing infrastructure for a custom parser implementation.
 *
 * This file provides the foundation for building specific parsers with built-in
 * trivia handling and error management. It defines the abstract Parser class,
 * which implements the InputStream interface and adds automatic trivia skipping
 * functionality.
 */

import { Builder, Stack } from './builder';
import { InputStream, LineAndColumn } from './inputStream';

export type ParseFailure = {
    message: string;
    position: number;
    children?: ParseFailure[];
};
export type ParseResult<T> = { success: true; value: T } | { success: false; failure: ParseFailure };

/**
 * An abstract base Parser class that implements InputStream and adds automatic trivia skipping functionality.
 * This class serves as a foundation for building specific parsers with built-in trivia handling.
 *
 * @implements {InputStream}
 */
export abstract class Parser implements InputStream {
    /** Whether to skip trivia or not (true by default) */
    private skipTriviaEnabled: boolean = true;
    private debugEnabled: boolean;

    builder: Builder = new Builder();

    /**
     * Creates a new Parser instance.
     * @param input - The InputStream to wrap.
     * @param debug - Whether to enable debug logging (false by default).
     */
    constructor(
        protected input: InputStream,
        debug: boolean = false
    ) {
        this.debugEnabled = debug;
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
        const mark = this.mark();
        const builderMark = this.builder.mark();
        if (predicate()) return true;
        this.builder.restore(builderMark);
        this.restore(mark);
        return false;
    }

    skipNegativeLookahead(predicate: () => boolean): boolean {
        const mark = this.mark();
        const builderMark = this.builder.mark();
        if (predicate()) {
            this.builder.restore(builderMark);
            this.restore(mark);
            return false;
        }
        return true;
    }

    skipTrivia(predicate: () => boolean): boolean {
        const inputMark = this.mark();
        const builderMark = this.builder.mark();
        this.skipTriviaIfEnabled();
        if (predicate()) return true;
        this.builder.restore(builderMark);
        this.restore(inputMark);
        return false;
    }

    ignoreSkipTriviaDuring<T>(consumer: () => boolean): boolean {
        this.skipTriviaIfEnabled();
        const originalSkipTrivia = this.skipTriviaEnabled;
        this.skipTriviaEnabled = false;
        try {
            return consumer();
        } finally {
            this.skipTriviaEnabled = originalSkipTrivia;
        }
    }

    makeString(from: number, to: number): string {
        return this.input.makeString(from, to);
    }

    buildBoolean(label: string | undefined, parser: () => boolean): boolean {
        if (!parser()) return false;
        this.builder.push(label, true);
        return true;
    }

    buildString(label: string | undefined, parser: () => boolean): boolean {
        const mark = this.mark();
        if (!parser()) return false;
        this.builder.push(label, this.makeString(mark, this.mark()));
        return true;
    }

    buildStringObject<T>(
        label: string | undefined,
        factory: { new (init: { value: string }): T },
        parser: () => boolean
    ): boolean {
        const position = this.getPosition();
        if (!parser()) return false;
        this.builder.push(label, new factory({ value: this.makeString(position, this.getPosition()) }));
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
        const builderMark = this.builder.mark();
        if (parser()) {
            this.builder.create(builderMark, label, buildFunction);
            return true;
        } else {
            return false;
        }
    }

    /**
     * Abstract method to skip trivia. Must be implemented by subclasses.
     * This method defines how trivia is identified and skipped in the input.
     * @returns The kind of trivia skipped, or undefined if no trivia was skipped.
     */
    protected abstract consumeTrivia(): string | undefined;

    /**
     * Abstract method to consume an identifier for a keyword.
     * Must be implemented by subclasses.
     * @returns The consumed identifier, or undefined if no identifier was consumed.
     */
    protected abstract consumeIdentifierForKeyword(): string | undefined;

    /**
     * Logs a debug message if debugging is enabled.
     * @param message - The message to log.
     * @private
     */
    private debugLog(message: string): void {
        if (this.debugEnabled) {
            const position = this.getPosition();
            const { line, column } = this.positionToLineAndColumn(position);
            console.log(
                `[DEBUG] [${line}:${column < 10 ? '0' : ''}${column}] ${this.indent < 10 ? ' ' : ''}${this.indent} ${'  '.repeat(this.indent)}${message}`
            );
        }
    }

    /**
     * Skips trivia if enabled and records the skipped trivia.
     * This method is called automatically before most input operations.
     * @private
     */
    private skipTriviaIfEnabled(): void {
        if (!this.skipTriviaEnabled) return;

        const originalDebugEnabled = this.debugEnabled;
        this.debugEnabled = false;
        while (true) {
            // This enables consumeTrivia to use its local InputStream
            this.skipTriviaEnabled = false;
            try {
                const kind = this.consumeTrivia();
                if (kind === undefined) break;
            } finally {
                this.skipTriviaEnabled = true;
            }
        }
        this.debugEnabled = originalDebugEnabled;
    }

    /**
     * Temporarily disables trivia skipping, executes a consumer function, then restores trivia skipping.
     * Useful for parsing sections where trivia should be treated as significant.
     *
     * @param consumer - The consumer function to execute without trivia skipping.
     * @returns The result of the executed consumer function.
     */
    ignoreTriviaDuring<T>(consumer: () => ParseResult<T>): ParseResult<T> {
        this.skipTriviaIfEnabled();
        const originalSkipTrivia = this.skipTriviaEnabled;
        this.skipTriviaEnabled = false;
        try {
            return consumer();
        } finally {
            this.skipTriviaEnabled = originalSkipTrivia;
        }
    }

    // InputStream interface methods

    /**
     * @inheritdoc
     */
    getPosition(): number {
        return this.input.getPosition();
    }

    mark(): number {
        return this.input.mark();
    }

    /**
     * @inheritdoc
     * @throws Error if attempting to restore to a future position.
     */
    restore(mark: number): void {
        if (mark > this.input.mark()) throw new Error('Cannot restore to a future position');
        this.input.restore(mark);
        this.debugLog(`Restored position to ${mark}`);
    }

    /**
     * @inheritdoc
     */
    positionToLineAndColumn(position: number): LineAndColumn {
        return this.input.positionToLineAndColumn(position);
    }

    /**
     * @inheritdoc
     */
    isEOF(): boolean {
        this.skipTriviaIfEnabled();
        const eof = this.input.isEOF();
        this.debugLog(`Checked EOF: ${eof}`);
        return eof;
    }

    /**
     * @inheritdoc
     */
    peek(): string | undefined {
        this.skipTriviaIfEnabled();
        const peeked = this.input.peek();
        this.debugLog(`Peeked: ${peeked === undefined ? 'EOF' : `"${peeked}"`}`);
        return peeked;
    }

    mustBeEOF(): ParseResult<void> {
        if (!this.isEOF()) {
            const remainingContent = this.input.peek();
            return this.failure(`Unexpected content parsing: "${remainingContent}${this.input.peek(1) ? '...' : ''}"`);
        }
        return this.success(undefined);
    }

    /**
     * Peeks at the next character in the input stream and ensures it's not EOF.
     *
     * @param expected - A description of the expected character or token, used in the error message.
     * @returns A ParseResult containing the next character in the input stream or a failure.
     */
    mustPeek(expected: string): ParseResult<string> {
        const c = this.peek();
        if (!c) {
            return this.failure(`Expected ${expected}, but found EOF`);
        }
        return this.success(c);
    }

    /**
     * Checks if the parser is at EOF or if the next character matches the given string.
     *
     * @param str - A single-character string to check against the next input character.
     * @returns true if at EOF or if the next character matches str, false otherwise.
     * @throws {Error} if str is not a single-character string.
     */
    isEOFOrPeekChar(str: string): boolean {
        if (str.length > 1) throw new Error('notEOFAndCannotPeekChar only accepts single-character strings');
        this.skipTriviaIfEnabled();
        const result = this.isEOF() || this.input.peek() === str;
        this.debugLog(`Checked EOF or peek char "${str}": ${result}`);
        return result;
    }

    /**
     * Checks if the parser is at EOF or if the next character is one of the characters in the given string.
     *
     * @param str - A string containing all characters to check against the next input character.
     * @returns true if at EOF or if the next character is in the provided string, false otherwise.
     */
    isEOFOrPeekOneOf(str: string): boolean {
        this.skipTriviaIfEnabled();
        const result = this.isEOF() || str.includes(this.input.peek()!);
        this.debugLog(`Checked EOF or peek one of "${str}": ${result}`);
        return result;
    }

    /**
     * @inheritdoc
     */
    consume(count: number = 1): string | undefined {
        this.skipTriviaIfEnabled();
        const position = this.input.getPosition();
        if (this.input.skip(count)) {
            const result = this.input.makeString(position, this.input.getPosition());
            this.debugLog(`Consumed ${count} character(s): "${result}"`);
            return result;
        } else {
            this.debugLog(`Failed to consume ${count} character(s)`);
            return undefined;
        }
    }

    /**
     * @inheritdoc
     */
    consumeString(str: string): string | undefined {
        this.skipTriviaIfEnabled();
        if (this.input.skipString(str)) {
            this.debugLog(`Failed to consume string: "${str}"`);
            return str;
        } else {
            this.debugLog(`Failed to consume string: "${str}"`);
            return undefined;
        }
    }

    /**
     * Attempts to consume a specific string and returns a ParseResult.
     *
     * @param str - The string to consume.
     * @returns A ParseResult containing the consumed string or a failure.
     */
    protected mustConsumeString(str: string): ParseResult<string> {
        const consumed = this.consumeString(str);
        if (consumed === undefined) {
            return this.failure(`Expected "${str}", but found "${this.peek() ?? 'EOF'}"`);
        }
        return this.success(consumed);
    }

    protected consumeKeyword(keyword: string): string | undefined {
        const inputMark = this.input.mark();
        const consumed = this.consumeIdentifierForKeyword();
        if (consumed !== keyword) {
            this.input.restore(inputMark);
            this.debugLog(`Failed to consume keyword: "${keyword}"`);
            return undefined;
        }
        this.debugLog(`Consumed keyword: "${keyword}"`);
        return keyword;
    }

    /**
     * Attempts to consume a keyword and returns a ParseResult.
     *
     * @param keyword - The keyword to consume.
     * @returns A ParseResult containing the consumed keyword or a failure.
     */
    protected mustConsumeKeyword(keyword: string): ParseResult<string> {
        const consumed = this.consumeKeyword(keyword);
        if (consumed === undefined) {
            return this.failure(`Expected keyword "${keyword}", but found "${this.peek() ?? 'EOF'}"`);
        }
        return this.success(consumed);
    }

    /**
     * @inheritdoc
     */
    consumeWhile(predicate: (char: string) => boolean): string | undefined {
        this.skipTriviaIfEnabled();
        const position = this.input.getPosition();
        if (this.input.skipWhile(predicate)) {
            const result = this.input.makeString(position, this.input.getPosition());
            this.debugLog(`Consumed while predicate: "${result}"`);
            return result;
        } else {
            this.debugLog(`Failed to consume while predicate`);
            return undefined;
        }
    }

    mustConsumeWhile(predicate: (char: string) => boolean, expected: string): ParseResult<string> {
        const consumed = this.consumeWhile(predicate);
        if (consumed === undefined) {
            return this.failure(`Expected ${expected}, but found "${this.peek() ?? 'EOF'}"`);
        }
        return this.success(consumed);
    }

    /**
     * @inheritdoc
     */
    consumeRegex(regex: RegExp): string | undefined {
        this.skipTriviaIfEnabled();
        const position = this.input.getPosition();
        if (this.input.skipRegex(regex)) {
            const result = this.input.makeString(position, this.input.getPosition());
            this.debugLog(`Consumed regex ${regex}: "${result}"`);
            return result;
        } else {
            this.debugLog(`Failed to consume regex: ${regex}`);
            return undefined;
        }
    }

    mustConsumeRegex(regex: RegExp, expected: string): ParseResult<string> {
        const consumed = this.consumeRegex(regex);
        if (consumed === undefined) {
            return this.failure(`Expected ${expected}, but found "${this.peek() ?? 'EOF'}"`);
        }
        return this.success(consumed);
    }

    /**
     * Attempts to parse using multiple alternative parsers.
     *
     * @param alternatives - An array of parser functions to try.
     * @returns A ParseResult containing the result of the first successful parser or a failure.
     */
    protected firstAlternative<T extends any[]>(
        expected: string,
        ...alternatives: { [K in keyof T]: () => ParseResult<T[K]> }
    ): ParseResult<T[number]> {
        const mark = this.mark();
        const errors: ParseFailure[] = [];
        for (const alternative of alternatives) {
            const result = alternative();
            if (result.success) {
                this.debugLog(`First alternative succeeded: ${expected}`);
                return result;
            }
            errors.push(result.failure);
            this.restore(mark);
        }
        const found = this.peek() ?? 'EOF';
        this.debugLog(`First alternative failed: ${expected}`);
        return {
            success: false,
            failure: {
                message: `Expected "${expected}", but found "${found}"`,
                position: mark,
                children: errors
            }
        };
    }

    /**
     * Helper method to attempt a parsing operation, returning a success result with undefined value if it fails.
     * Useful for implementing optional grammar rules.
     *
     * @param parser - A function that performs a parsing operation.
     * @returns A ParseResult containing the result of the parsing operation, or a success result with undefined value if it fails.
     */
    protected optional<T>(parser: () => ParseResult<T>): ParseResult<T | undefined> {
        const mark = this.mark();
        const result = parser();
        if (result.success) {
            this.debugLog(`Maybe succeeded`);
            return result;
        } else {
            this.restore(mark);
            this.debugLog(`Maybe failed`);
            return this.success(undefined);
        }
    }

    /**
     * Parses zero or more occurrences of a pattern.
     *
     * @param parser - A function that parses a single occurrence of the pattern.
     * @returns A ParseResult containing an array of parsed elements.
     */
    protected zeroOrMore<T>(parser: () => ParseResult<T>): ParseResult<T[]> {
        const elements: T[] = [];
        while (true) {
            const mark = this.mark();
            const result = parser();
            if (result.success) {
                elements.push(result.value);
            } else {
                this.restore(mark);
                this.debugLog(`Zero or more ended with ${elements.length} elements`);
                return this.success(elements);
            }
        }
    }

    /**
     * Parses one or more occurrences of a pattern.
     *
     * @param parser - A function that parses a single occurrence of the pattern.
     * @returns A ParseResult containing an array of parsed elements or a failure.
     */
    protected oneOrMore<T>(parser: () => ParseResult<T>): ParseResult<T[]> {
        const elements: T[] = [];
        while (true) {
            const mark = this.mark();
            const result = parser();
            if (result.success) {
                elements.push(result.value);
            } else {
                this.restore(mark);
                if (elements.length > 0) {
                    this.debugLog(`One or more ended with ${elements.length} elements`);
                    return this.success(elements);
                }
                return result;
            }
        }
    }

    /**
     * Helper method to assert that a value is defined.
     *
     * @param value - The value to check.
     * @param expected - The expected value description.
     * @returns A ParseResult containing the value if it is defined, or a failure.
     */
    protected must<T>(value: T | undefined, expected: string): ParseResult<T> {
        if (value === undefined) {
            const found = this.peek() ?? 'EOF';
            this.debugLog(`Must failed: Expected "${expected}", but found "${found}"`);
            return this.failure(`Expected "${expected}", but found "${found}"`);
        }
        this.debugLog(`Must succeeded: "${expected}"`);
        return this.success(value);
    }

    /**
     * Wraps a parsing operation with context information for better error reporting.
     *
     * @param context - A string describing the context of the parsing operation.
     * @param parser - A function that performs a parsing operation.
     * @returns A ParseResult containing the result of the parsing operation or a failure with added context information.
     */
    private indent = 0;
    protected withContext<T>(context: string, parser: () => ParseResult<T>): ParseResult<T> {
        const pos = this.getPosition();
        this.debugLog(`+ ${context} "${this.peek()}" ${this.mark()}`);
        this.indent++;
        const result = parser();
        this.indent--;
        if (result.success) {
            this.debugLog(`- ${context}`);
            return result;
        } else {
            this.debugLog(`x ${context}`);
            return {
                success: false,
                failure: {
                    message: `«${context}»`,
                    position: pos,
                    children: [result.failure]
                }
            };
        }
    }

    /**
     * Creates a success ParseResult with the given value.
     *
     * @param value - The value to wrap in a success result.
     * @returns A ParseResult indicating success with the given value.
     */
    protected success<T>(value: T): ParseResult<T> {
        return { success: true, value };
    }

    protected failure<T>(message: string): ParseResult<T> {
        return {
            success: false,
            failure: {
                message,
                position: this.mark()
            }
        };
    }

    successOrThrow<T>(result: ParseResult<T>): T {
        if (result.success) {
            return result.value;
        } else {
            throw new Error(result.failure.message);
        }
    }
}
