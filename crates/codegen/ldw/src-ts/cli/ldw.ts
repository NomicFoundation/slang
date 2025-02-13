import { exec } from 'child_process';
import { Command, program } from 'commander';
import * as fs from 'fs';
import * as path from 'path';
import { promisify } from 'util';
import { ExtendedGrammarFromParsedGrammar } from '../languages/ldw/grammar/extended/creation/fromParsedGrammar';
import { ParsedGrammarFromSource } from '../languages/ldw/grammar/parsed/creation/fromSource';
import { TypedGrammarFromExtendedGrammar } from '../languages/ldw/grammar/typed/creation/fromExtendedGrammar';
import { TypedGrammarToRustParserSource } from '../languages/ldw/grammar/typed/outputs/toRustParserSource';
import { TypedGrammarToTypescriptParserSource } from '../languages/ldw/grammar/typed/outputs/toTypescriptParserSource';
import { AnalysedModelFromResolvedModel } from '../languages/ldw/model/analysed/creation/fromResolvedModel';
import { AnalysedModelToRustBuilderSource } from '../languages/ldw/model/analysed/outputs/rust/toBuilderSource';
import { AnalysedModelToRustHandlerSource } from '../languages/ldw/model/analysed/outputs/rust/toHandlerSource';
import { AnalysedModelToRustInPlaceTransformerSource } from '../languages/ldw/model/analysed/outputs/rust/toInPlaceTransformerSource';
import { AnalysedModelToRustModelSource } from '../languages/ldw/model/analysed/outputs/rust/toModelSource';
import { AnalysedModelToRustTransformerSource } from '../languages/ldw/model/analysed/outputs/rust/toTransformerSource';
import { AnalysedModelToRustVisitorSource } from '../languages/ldw/model/analysed/outputs/rust/toVisitorSource';
import { AnalysedModelToTypescriptBuilderSource } from '../languages/ldw/model/analysed/outputs/typescript/toBuilderSource';
import { AnalysedModelToTypescriptModelSource } from '../languages/ldw/model/analysed/outputs/typescript/toModelSource';
import { AnalysedModelToTypescriptTransformerSource } from '../languages/ldw/model/analysed/outputs/typescript/toTransformerSource';
import { AnalysedModelToTypescriptVisitorSource } from '../languages/ldw/model/analysed/outputs/typescript/toVisitorSource';
import { ParsedModelFromSource } from '../languages/ldw/model/parsed/creation/fromSource';
import { ParsedModelFromTypedGrammar } from '../languages/ldw/model/parsed/creation/fromTypedGrammar';
import { Model as ParsedModel } from '../languages/ldw/model/parsed/model';
import { ParsedModelToSource } from '../languages/ldw/model/parsed/outputs/toSource';
import { ResolvedModelFromParsedModel } from '../languages/ldw/model/resolved/creation/fromParsedModel';
import { RemoveAnonymousTypes } from '../languages/ldw/model/resolved/transforms/removeAnonymousTypes';
import { composePasses, optionalPass } from '../nanopass/combinators';
import { Registry } from '../nanopass/registry';
import { ParseError } from '../parsing/parseError';

program.version('1.0.0').description('Language Design Workbench CLI');

class ArgumentValidationError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'ArgumentValidationError';
    }
}

function handleError(error: unknown, command: Command): never {
    if (error instanceof ArgumentValidationError) {
        process.stderr.write(`Error: ${error.message}\n`);
        command.help({ error: true });
    } else if (error instanceof ParseError) {
        process.stderr.write(`Parse Error: ${error.toString()}\n`);
    } else {
        // process.stderr.write(`${error}\n`);
        throw error;
    }
    process.exit(1);
}

function validateDirectory(dir: string, create: boolean = false): void {
    const absolutePath = path.resolve(dir);
    if (fs.existsSync(absolutePath)) {
        if (!fs.statSync(absolutePath).isDirectory()) {
            throw new ArgumentValidationError(`Path exists but is not a directory: ${dir}`);
        }
    } else if (create) {
        fs.mkdirSync(absolutePath, { recursive: true });
    } else {
        throw new ArgumentValidationError(`Directory does not exist: ${dir}`);
    }
}

async function writeOutput(content: string, outDir: string, fqn: string, filename: string) {
    const dir = path.resolve(outDir, ...fqn.split('::'));
    validateDirectory(dir, true);
    const outputPath = path.resolve(dir, filename);

    const newPath = path.join(path.dirname(outputPath), `new.${path.basename(outputPath)}`);
    const header = `// Generated on ${new Date().toISOString()}\n`;
    fs.writeFileSync(newPath, header + content);
    await formatOutput(newPath);
    // Re-read the new content after it has been formatted.
    const newContent = fs.readFileSync(newPath, 'utf-8');
    const newContentWithoutHeader = newContent.startsWith('// Generated on')
        ? newContent.slice(newContent.indexOf('\n') + 1).trim()
        : newContent.trim();

    const oldPath = path.join(path.dirname(outputPath), `${path.basename(outputPath)}`);
    if (fs.existsSync(oldPath)) {
        const oldContent = fs.readFileSync(oldPath, 'utf-8');
        const oldContentWithoutHeader = oldContent.startsWith('// Generated on')
            ? oldContent.slice(oldContent.indexOf('\n') + 1).trim()
            : oldContent.trim();

        if (newContentWithoutHeader === oldContentWithoutHeader) {
            fs.unlinkSync(newPath);
            console.log(`        Unchanged ${fqn} ${filename}`);
            return;
        }

        fs.unlinkSync(oldPath);
    }

    console.log(`        Update ${fqn} ${filename}`);
    fs.renameSync(newPath, oldPath);
}

const execPromise = promisify(exec);

async function formatOutput(filePath: string) {
    const ext = path.extname(filePath);
    if (ext === '.ts' || ext === '.js' || ext === '.json') {
        await execPromise(`npx prettier --write "${filePath}"`);
    } else if (ext === '.rs') {
        await execPromise(`rustfmt "${filePath}"`);
    }
}

program
    .command('process-grammar')
    .description('Parse .grammar source and generate artefacts')
    .option('--in-dir <dir>', 'Languages directory for resolving fully qualified names')
    .option('--out-dir <dir>', 'Output directory for generated files')
    .option('--name <name>', 'Fully qualified name of the grammar')
    .option('--language <lang>', 'Output language (typescript or rust)', 'typescript')
    .option(
        '--roots <roots>',
        'Comma-separated list of root rule names',
        (value: string, previous: string[]) => {
            const newRoots = value.split(',');
            return previous ? previous.concat(newRoots) : newRoots;
        },
        [] as string[]
    )
    .action(async (options, command) => {
        try {
            if (!options.inDir) {
                throw new ArgumentValidationError('Input directory is required');
            }
            if (!options.outDir) {
                throw new ArgumentValidationError('Output directory is required');
            }
            if (!options.name) {
                throw new ArgumentValidationError('Fully qualified name is required');
            }

            validateDirectory(options.inDir);
            validateDirectory(options.outDir, true);

            const registry = new Registry(options.inDir);
            const isTypescript = options.language === 'typescript';
            const dslSource = registry.readInput(options.name, 'ldw.grammar');

            const typedGrammar = composePasses(
                new ParsedGrammarFromSource(),
                new ExtendedGrammarFromParsedGrammar(),
                new TypedGrammarFromExtendedGrammar()
            ).transform(dslSource);

            const modelSource = composePasses(new ParsedModelFromTypedGrammar(), new ParsedModelToSource()).transform(
                typedGrammar
            );
            writeOutput(modelSource, options.outDir, options.name, 'ldw.model');

            if (isTypescript) {
                const parserSource = new TypedGrammarToTypescriptParserSource(options.roots).transform(typedGrammar);
                writeOutput(parserSource, options.outDir, options.name, 'parser.ts');
            } else {
                const parserSource = new TypedGrammarToRustParserSource(options.roots).transform(typedGrammar);
                writeOutput(parserSource, options.outDir, options.name, 'parser.rs');
            }
        } catch (error) {
            handleError(error, command);
        }
    });

program
    .command('process-model')
    .description('Parse .model source and generate artefacts')
    .option('--in-dir <dir>', 'Languages directory for resolving fully qualified names')
    .option('--out-dir <dir>', 'Output directory for generated files')
    .option('--name <name>', 'Fully qualified name of the model')
    .option('--language <lang>', 'Output language (typescript or rust)', 'typescript')
    .action(async (options, command) => {
        try {
            if (!options.inDir) {
                throw new ArgumentValidationError('Input directory is required');
            }
            if (!options.outDir) {
                throw new ArgumentValidationError('Output directory is required');
            }
            if (!options.name) {
                throw new ArgumentValidationError('Fully qualified name is required');
            }

            validateDirectory(options.inDir);
            validateDirectory(options.outDir, true);

            const registry = new Registry(options.inDir);
            const isTypescript = options.language === 'typescript';
            const isRust = options.language === 'rust';

            const dslSource = registry.readInput(options.name, 'ldw.model');

            const analysedModel = composePasses(
                new ParsedModelFromSource(),
                new ResolvedModelFromParsedModel((fqn: string): ParsedModel => {
                    const modelSource = registry.readInput(fqn, 'ldw.model');
                    return new ParsedModelFromSource().transform(modelSource);
                }),
                optionalPass(isRust, new RemoveAnonymousTypes()),
                new AnalysedModelFromResolvedModel()
            ).transform(dslSource);

            if (isTypescript) {
                const modelSource = new AnalysedModelToTypescriptModelSource(registry).transform(analysedModel);
                writeOutput(modelSource, options.outDir, options.name, 'model.ts');

                const visitorSource = new AnalysedModelToTypescriptVisitorSource(registry).transform(analysedModel);
                writeOutput(visitorSource, options.outDir, options.name, 'visitor.ts');

                const transformerSource = new AnalysedModelToTypescriptTransformerSource(registry).transform(
                    analysedModel
                );
                writeOutput(transformerSource, options.outDir, options.name, 'transformer.ts');

                const builderSource = new AnalysedModelToTypescriptBuilderSource(registry).transform(analysedModel);
                writeOutput(builderSource, options.outDir, options.name, 'builder.ts');
            } else {
                const modelSource = new AnalysedModelToRustModelSource(registry).transform(analysedModel);
                writeOutput(modelSource, options.outDir, options.name, 'model.rs');

                const handlerSource = new AnalysedModelToRustHandlerSource(registry).transform(analysedModel);
                writeOutput(handlerSource, options.outDir, options.name, 'handler.rs');

                const visitorSource = new AnalysedModelToRustVisitorSource(registry).transform(analysedModel);
                writeOutput(visitorSource, options.outDir, options.name, 'visitor.rs');

                const transformerSource = new AnalysedModelToRustTransformerSource(registry).transform(analysedModel);
                writeOutput(transformerSource, options.outDir, options.name, 'transformer.rs');

                const inPlaceTransformerSource = new AnalysedModelToRustInPlaceTransformerSource(registry).transform(
                    analysedModel
                );
                writeOutput(inPlaceTransformerSource, options.outDir, options.name, 'in_place_transformer.rs');

                const builderSource = new AnalysedModelToRustBuilderSource(registry).transform(analysedModel);
                writeOutput(builderSource, options.outDir, options.name, 'builder.rs');
            }
        } catch (error) {
            handleError(error, command);
        }
    });

program.parse(process.argv);
