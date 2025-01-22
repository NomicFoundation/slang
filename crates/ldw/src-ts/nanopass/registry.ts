import * as fs from 'fs';
import * as path from 'path';

export class Registry {
    constructor(private registryDir: string) {}

    private validateFqn(fqn: string): void {
        if (!/^[a-zA-Z_][a-zA-Z0-9_]*(::([a-zA-Z_][a-zA-Z0-9_]*))*$/.test(fqn)) {
            throw new Error(`Invalid FQN: ${fqn}. FQNs must use '::' as separator.`);
        }
    }

    public resolvePath(fqn: string, filename: string): string {
        this.validateFqn(fqn);
        const filePath = path.resolve(this.registryDir, ...fqn.split('::'), filename);
        if (!fs.existsSync(filePath)) {
            throw new Error(`File not found: ${filePath}`);
        }
        return filePath;
    }

    public readInput(fqn: string, filename: string): string {
        const filePath = this.resolvePath(fqn, filename);
        return fs.readFileSync(filePath, 'utf-8');
    }

    // public relativePathToModule(from: string, to: string): string {
    //     this.validateFqn(from);
    //     this.validateFqn(to);
    //     const relativePath = path.relative(from, to);
    //     return relativePath.startsWith('.') ? relativePath : './' + relativePath;
    // }
}
