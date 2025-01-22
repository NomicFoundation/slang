import { Registry } from '../../src-ts/nanopass/registry';
import * as path from 'path';
import * as fs from 'fs';

describe('Registry', () => {
    let registry: Registry;
    const mockRegistryPath = path.join(__dirname, 'mock-registry.json');

    beforeEach(() => {
        // Create a mock registry file
        const mockRegistry = {
            'module::a': 'src-ts/moduleA',
            'module::b': 'src-ts/nested/moduleB',
            'module::c::d': 'src-ts/deeply/nested/moduleCD'
        };
        fs.writeFileSync(mockRegistryPath, JSON.stringify(mockRegistry));

        registry = new Registry(mockRegistryPath);
    });

    afterEach(() => {
        // Clean up the mock registry file
        fs.unlinkSync(mockRegistryPath);
    });

    test('relativePathToModule returns correct relative path with double-colon notation', () => {
        const relativePath = registry.relativePathToModule('module::a', 'module::b');
        expect(relativePath).toBe('./nested');
    });

    test('relativePathToModule returns correct relative path for deeply nested modules', () => {
        const relativePath = registry.relativePathToModule('module::a', 'module::c::d');
        expect(relativePath).toBe('./deeply/nested');
    });

    test('relativePathToModule throws error for non-existent module', () => {
        expect(() => {
            registry.relativePathToModule('module::a', 'module::e');
        }).toThrow('Module path not found in registry: module::e');
    });

    test('relativePathToModule throws error for invalid FQN with dot notation', () => {
        expect(() => {
            registry.relativePathToModule('module.a', 'module.b');
        }).toThrow("Invalid FQN: module.a. FQNs must use '::'");
    });

    test('resolvePath throws error for invalid FQN', () => {
        expect(() => {
            registry.resolvePath('invalid-fqn', 'file.txt');
        }).toThrow("Invalid FQN: invalid-fqn. FQNs must use '::'");
    });

    test('readInput throws error for invalid FQN', () => {
        expect(() => {
            registry.readInput('invalid.fqn', 'file.txt');
        }).toThrow("Invalid FQN: invalid.fqn. FQNs must use '::'");
    });

    test('writeOutput throws error for invalid FQN', async () => {
        await expect(async () => {
            await registry.writeOutput('invalid.fqn', 'content', 'file.txt');
        }).rejects.toThrow("Invalid FQN: invalid.fqn. FQNs must use '::'");
    });
});
