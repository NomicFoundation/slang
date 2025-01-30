export class IndentingOutputStream {
    private content: string = '';
    private indentLevel: number = 0;
    private indentString: string = '    ';
    pendingIndent: any;

    constructor(indentString: string = '    ') {
        this.indentString = indentString;
    }

    write(text: string): void {
        if (this.pendingIndent) {
            this.content += this.indentString.repeat(this.indentLevel);
            this.pendingIndent = false;
        }
        this.content += text;
    }

    writeLine(text: string = ''): void {
        if (this.pendingIndent) {
            this.content += this.indentString.repeat(this.indentLevel);
            this.pendingIndent = false;
        }
        this.content += text + '\n';
        this.pendingIndent = true;
    }

    indent(): void {
        this.indentLevel++;
    }

    dedent(): void {
        if (this.indentLevel > 0) {
            this.indentLevel--;
        }
    }

    indentDuring(fn: () => void): void {
        this.indent();
        fn();
        this.dedent();
    }

    join<T>(items: T[], separator: string, itemCallback: (item: T, index: number) => void): void {
        items.forEach((item, index) => {
            itemCallback(item, index);
            if (index < items.length - 1) {
                this.write(separator);
            }
        });
    }

    joinLinesSeparating<T>(items: T[], separator: string, itemCallback: (item: T, index: number) => void): void {
        items.forEach((item, index) => {
            itemCallback(item, index);
            if (index < items.length - 1) {
                this.writeLine(separator);
            } else {
                this.writeLine();
            }
        });
    }

    joinLinesPrefixing<T>(items: T[], prefix: string, itemCallback: (item: T, index: number) => void): void {
        items.forEach((item, index) => {
            if (index > 0) {
                this.write(prefix);
            } else {
                this.write(' '.repeat(prefix.length));
            }
            itemCallback(item, index);
            this.writeLine();
        });
    }

    toString(): string {
        return this.content;
    }
}
