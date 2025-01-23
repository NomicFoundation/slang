/**
 * @file inputStream.ts
 * @description Defines core interfaces for input stream handling in the parser.
 */

/**
 * Represents line and column information for a position in the input.
 * This interface is used to provide more human-readable position information.
 */
export interface LineAndColumn {
    /** The line number in the input (1-based) */
    line: number;
    /** The column number in the input (1-based) */
    column: number;
}

/**
 * Defines the interface for input stream classes, providing methods for reading and manipulating input.
 * This interface serves as a contract for various input stream implementations, allowing for flexibility
 * in how input is sourced and processed.
 */
export interface InputStream {
    /**
     * Returns the current position in the input.
     * @returns The current position as a number.
     */
    mark(): number;

    /**
     * Restores the stream to a previously saved position.
     * This method is crucial for implementing backtracking in the parser.
     * @param position - The position to restore to.
     */
    restore(position: number): void;

    /**
     * Returns the current position in the input.
     * @returns The current position as a number.
     */
    getPosition(): number;

    /**
     * Converts a position to line and column information.
     * This method is useful for generating human-readable error messages.
     * @param position - The position to convert.
     * @returns An object containing line and column information.
     */
    positionToLineAndColumn(position: number): LineAndColumn;

    /**
     * Checks if the end of the input has been reached.
     * @returns True if at the end of the input, false otherwise.
     */
    isEOF(): boolean;

    /**
     * Attempts to peek at upcoming characters in the input without consuming them.
     * This method is essential for lookahead operations in the parser.
     * @param lookAhead - The number of characters to look ahead (default is 0).
     * @returns The character at the specified look-ahead position, or undefined if beyond the input length.
     */
    peek(lookAhead?: number): string | undefined;

    skip(count?: number): boolean;

    skipString(str: string): boolean;

    skipRegex(regex: RegExp): boolean;

    skipWhile(predicate: (char: string) => boolean): boolean;

    makeString(from: number, to: number): string;
}
