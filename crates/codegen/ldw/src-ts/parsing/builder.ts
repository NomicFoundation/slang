import assert from 'assert';

export type Stack = { label: string | undefined; value: any }[];

export class Builder {
    stack: Stack = [];

    mark(): number {
        return this.stack.length;
    }

    restore(depth: number): void {
        this.stack.splice(depth);
    }

    push(label: string | undefined, value: any): void {
        this.stack.push({ label, value });
    }

    relabel(label: string | undefined): void {
        this.stack[this.stack.length - 1].label = label;
    }

    finalise(): any {
        assert(this.stack.length === 1);
        return this.stack.pop()!.value;
    }

    create(depth: number, label: string | undefined, factory: (stack: Stack) => any): void {
        const stackSegment = this.stack.splice(this.stack.length - depth);
        this.push(label, factory(stackSegment));
    }
}
