/**
 * @file stringInputStream.ts
 * @description Implements the InputStream interface for string-based input processing.
 */

import { InputStream, LineAndColumn } from './inputStream';

/**
 * Implements the {@link InputStream} interface, providing methods for reading and manipulating string input.
 * This class is designed to work with string-based input sources, allowing for efficient parsing and manipulation
 * of string content.
 *
 * @implements {InputStream}
 */
export class StringInputStream implements InputStream {
    private position: number;

    /**
     * Creates a new StringInputStream instance.
     * @param input - The input string to be parsed.
     */
    constructor(private input: string) {
        this.position = 0;
    }

    /**
     * Returns the current position in the input string.
     * @returns The current position as a number.
     */
    getPosition(): number {
        return this.position;
    }

    mark(): number {
        return this.position;
    }

    /**
     * Restores the stream to a previously saved position.
     * This method is crucial for implementing backtracking in the parser.
     * @param position - The position to restore to.
     */
    restore(position: number): void {
        this.position = position;
    }

    /**
     * Converts a position to line and column information.
     * This method is useful for generating human-readable error messages.
     *
     * @param position - The position to convert.
     * @returns An object containing line and column information.
     *
     * @remarks
     * TODO: Optimize this method by caching values and using incremental calculation.
     */
    positionToLineAndColumn(position: number): LineAndColumn {
        let line = 1;
        let column = 1;
        let p = 0;
        while (p < position) {
            if (this.input[p] === '\n') {
                line++;
                column = 0;
            } else {
                column++;
            }
            p++;
        }
        return { line, column };
    }

    /**
     * Checks if the end of the input has been reached.
     * @returns True if at the end of the input, false otherwise.
     */
    isEOF(): boolean {
        return this.position >= this.input.length;
    }

    skip(count: number = 1): boolean {
        if (count < 0) return false;
        const newPosition = this.position + count;
        if (newPosition > this.input.length) return false;
        this.position = newPosition;
        return true;
    }

    skipString(str: string): boolean {
        if (this.input.startsWith(str, this.position)) {
            this.position += str.length;
            return true;
        }
        return false;
    }

    skipRegex(regex: RegExp): boolean {
        const remainingInput = this.input.slice(this.position);
        const match = remainingInput.match(regex);
        if (match && match.index === 0) {
            const consumedString = match[0];
            this.position += consumedString.length;
            return true;
        }
        return false;
    }

    skipWhile(predicate: (char: string) => boolean): boolean {
        if (this.isEOF()) return false;

        let foundAtLeastOne = false;
        while (true) {
            // TODO: inline the peeking logic
            const char = this.peek();
            if (char === undefined || !predicate(char)) {
                return foundAtLeastOne;
            }
            this.position++;
            foundAtLeastOne = true;
        }
    }

    /**
     * Attempts to peek at upcoming characters in the input without consuming them.
     * This method is essential for lookahead operations in the parser.
     *
     * @param lookAhead - The number of characters to look ahead (default is 0).
     * @returns The character at the specified look-ahead position, or undefined if beyond the input length.
     */
    peek(lookAhead: number = 0): string | undefined {
        const peekIndex = this.position + lookAhead;
        if (peekIndex >= this.input.length) {
            return undefined;
        }
        return this.input[peekIndex];
    }

    makeString(from: number, to: number): string {
        return this.input.slice(from, to);
    }
}
