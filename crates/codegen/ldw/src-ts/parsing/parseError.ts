/**
 * @file parseError.ts
 * @description Defines a custom error class for handling parsing errors with detailed position information.
 */

import { InputStream } from './inputStream';

/**
 * Custom error class for parsing errors.
 * Provides detailed information about the error location in the input and supports a tree structure for nested errors.
 *
 * @extends Error
 */
export class ParseError extends Error {
    /**
     * Optional array of child ParseError instances, allowing for a tree structure of errors.
     */
    public children?: ParseError[];

    /**
     * Creates a new ParseError instance.
     *
     * @param message - The error message.
     * @param position - The position in the input where the error occurred.
     * @param input - The InputStream instance associated with this error.
     */
    constructor(
        message: string,
        public position: number,
        public input: InputStream
    ) {
        super(message);
        this.name = 'ParseError';
    }

    /**
     * Converts the ParseError tree to a string representation.
     * This method provides a human-readable representation of the entire error tree.
     *
     * @returns A string representation of the entire error tree.
     */
    toString(): string {
        // Start the recursion with an empty prefix and mark it as the last (and only) root node
        return this.toIndentedString('', true);
    }

    /**
     * Recursively builds the indented string representation of the error tree.
     * This private method is used internally by {@link ParseError.toString}.
     *
     * @param prefix - The current indentation prefix for this node.
     * @param isLast - Whether this node is the last child of its parent.
     * @returns The indented string representation of this node and its children.
     */
    private toIndentedString(prefix: string, isLast: boolean): string {
        // Convert the error position to line and column
        const { line, column } = this.input.positionToLineAndColumn(this.position);

        // Determine the prefix for this node
        // If it's the root (empty prefix), don't add any prefix
        // Otherwise, use '└─' for the last child, '├─' for others
        const thisPrefix = prefix ? (isLast ? '└─' : '├─') : '';

        // Construct the message for this node
        const thisMessage = `${prefix}${thisPrefix}${this.message} at ${line}:${column}`;

        // If this node has no children, return just this message
        if (!this.children || this.children.length === 0) {
            return thisMessage;
        }

        // Prepare the prefix for child nodes
        // Add '  ' (two spaces) for the last child's branch (no more siblings)
        // Add '│ ' for other branches (there are more siblings)
        const childPrefix = prefix + (isLast ? '  ' : '│ ');

        // Construct the full representation by combining:
        // 1. This node's message
        // 2. The indented representation of each child
        return [
            thisMessage,
            ...this.children.map((child, index, array) =>
                // Recursively call toIndentedString for each child
                // Pass the new child prefix and whether this child is the last sibling
                child.toIndentedString(childPrefix, index === array.length - 1)
            )
        ].join('\n');
    }
}
