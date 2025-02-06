import { InputStream } from '../../../../../parsing/inputStream';
import { Parser, ParseResult } from '../../../../../parsing/parser';
import { StringInputStream } from '../../../../../parsing/stringInputStream';
import * as Model from '../model';

export class ParsedModelFromSource {
    transform(input: string): Model.Model {
        const inputStream = new StringInputStream(input);
        const parser = new ModelParser(inputStream);
        let model = parser.successOrThrow(parser.parseModel());
        parser.successOrThrow(parser.mustBeEOF());
        return model;
    }
}

type TriviaKind = 'LineComment' | 'BlockComment' | 'Whitespace';
type Id = string;

export class ModelParser extends Parser {
    constructor(input: InputStream) {
        super(input);
    }

    parseModel(): ParseResult<Model.Model> {
        return this.withContext('model', () => {
            let name: Id[] = [];
            let parentName: Id[] | undefined;
            const values: (Model.Definition | Model.Deletion | Model.MemberModification)[] = [];

            const modelKeyword = this.mustConsumeKeyword('model');
            if (!modelKeyword.success) return modelKeyword;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name.push(nameResult.value);

            this.zeroOrMore(() => {
                const doubleColon = this.mustConsumeString('::');
                if (!doubleColon.success) return doubleColon;
                const nameResult = this.parseId();
                if (!nameResult.success) return nameResult;
                name.push(nameResult.value);
                return nameResult;
            });

            if (this.consumeKeyword('modifies')) {
                parentName = [];
                const parentNameResult = this.parseId();
                if (!parentNameResult.success) return parentNameResult;
                parentName.push(parentNameResult.value);
                this.zeroOrMore(() => {
                    const doubleColon = this.mustConsumeString('::');
                    if (!doubleColon.success) return doubleColon;
                    const nameResult = this.parseId();
                    if (!nameResult.success) return nameResult;
                    parentName!.push(nameResult.value);
                    return nameResult;
                });
            }

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            while (!this.consumeString('}')) {
                const element = this.firstAlternative(
                    'model element',
                    () => this.parseDeletion(),
                    () => this.parseMemberModification(),
                    () => this.parseDefinition()
                );
                if (!element.success) return element;
                values.push(element.value);

                const semicolon = this.mustConsumeString(';');
                if (!semicolon.success) return semicolon;
            }

            return this.success(new Model.Model({ name, parentName, values }));
        });
    }

    parseDefinition(): ParseResult<Model.Definition> {
        return this.withContext('definition', () => {
            let name: Id;
            let type: Model.Type;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const equals = this.mustConsumeString('=');
            if (!equals.success) return equals;

            const typeResult = this.parseType();
            if (!typeResult.success) return typeResult;
            type = typeResult.value;

            return this.success(new Model.Definition({ name, type }));
        });
    }

    parseDeletion(): ParseResult<Model.Deletion> {
        return this.withContext('deletion', () => {
            let name: Id;

            const deleteKeyword = this.mustConsumeKeyword('delete');
            if (!deleteKeyword.success) return deleteKeyword;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            return this.success(new Model.Deletion({ name }));
        });
    }

    parseMemberModification(): ParseResult<Model.MemberModification> {
        return this.withContext('member modification', () => {
            let name: Id;
            const values: (Model.MemberDeletion | Model.MemberAddition)[] = [];

            const modifyKeyword = this.mustConsumeKeyword('modify');
            if (!modifyKeyword.success) return modifyKeyword;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            this.oneOrMore(() => {
                const element = this.firstAlternative(
                    'member modification element',
                    () => this.parseMemberDeletion(),
                    () => this.parseMemberAddition()
                );
                if (element.success) values.push(element.value);
                return element;
            });

            return this.success(new Model.MemberModification({ name, values }));
        });
    }

    parseMemberDeletion(): ParseResult<Model.MemberDeletion> {
        return this.withContext('member deletion', () => {
            let name: Id;

            const minusEquals = this.mustConsumeString('-=');
            if (!minusEquals.success) return minusEquals;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            return this.success(new Model.MemberDeletion({ name }));
        });
    }

    parseMemberAddition(): ParseResult<Model.MemberAddition> {
        return this.withContext('member addition', () => {
            let value: Model.Type | Model.ProductMember;

            const plusEquals = this.mustConsumeString('+=');
            if (!plusEquals.success) return plusEquals;

            const valueResult = this.firstAlternative(
                'member addition element',
                () => this.parseProductMember(),
                () => this.parseType()
            );
            if (!valueResult.success) return valueResult;
            value = valueResult.value;

            return this.success(new Model.MemberAddition({ value }));
        });
    }

    parseType(): ParseResult<Model.Type> {
        return this.withContext('type', () => {
            return this.firstAlternative(
                'type',
                () => this.parseVoidType(),
                () => this.parsePrimitiveType(),
                () => this.parseEnumType(),
                () => this.parseTypeWithStructure(),
                () => this.parseNamedTypeReference()
            );
        });
    }

    parseVoidType(): ParseResult<Model.VoidType> {
        const voidType = this.mustConsumeString('()');
        if (!voidType.success) return voidType;
        return this.success(new Model.VoidType());
    }

    parsePrimitiveType(): ParseResult<Model.PrimitiveType> {
        if (this.consumeKeyword('string')) return this.success(Model.PrimitiveType.String);
        if (this.consumeKeyword('boolean')) return this.success(Model.PrimitiveType.Boolean);
        if (this.consumeKeyword('char')) return this.success(Model.PrimitiveType.Char);
        if (this.consumeKeyword('i8')) return this.success(Model.PrimitiveType.I8);
        if (this.consumeKeyword('i16')) return this.success(Model.PrimitiveType.I16);
        if (this.consumeKeyword('i32')) return this.success(Model.PrimitiveType.I32);
        if (this.consumeKeyword('i64')) return this.success(Model.PrimitiveType.I64);
        if (this.consumeKeyword('u8')) return this.success(Model.PrimitiveType.U8);
        if (this.consumeKeyword('u16')) return this.success(Model.PrimitiveType.U16);
        if (this.consumeKeyword('u32')) return this.success(Model.PrimitiveType.U32);
        if (this.consumeKeyword('u64')) return this.success(Model.PrimitiveType.U64);
        if (this.consumeKeyword('f32')) return this.success(Model.PrimitiveType.F32);
        if (this.consumeKeyword('f64')) return this.success(Model.PrimitiveType.F64);
        return this.failure('Missing primitive type');
    }

    parseEnumType(): ParseResult<Model.EnumType> {
        return this.withContext('enum type', () => {
            const members: string[] = [];

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            const firstMemberResult = this.parseString();
            if (!firstMemberResult.success) return firstMemberResult;
            members.push(firstMemberResult.value);

            while (this.consumeString('|')) {
                const memberResult = this.parseString();
                if (!memberResult.success) return memberResult;
                members.push(memberResult.value);
            }

            const closeBrace = this.mustConsumeString('}');
            if (!closeBrace.success) return closeBrace;

            return this.success(new Model.EnumType({ members }));
        });
    }

    parseString(): ParseResult<string> {
        let value: Id;

        const openQuote = this.mustConsumeString('"');
        if (!openQuote.success) return openQuote;

        const valueResult = this.parseId();
        if (!valueResult.success) return valueResult;
        value = valueResult.value;

        const closeQuote = this.mustConsumeString('"');
        if (!closeQuote.success) return closeQuote;

        return this.success(value);
    }

    parseTypeWithStructure(): ParseResult<Model.TypeWithStructure> {
        return this.withContext('type with structure', () => {
            return this.firstAlternative(
                'type with structure',
                () => this.parseSumType(),
                () => this.parseProductType(),
                () => this.parseGenericType()
            );
        });
    }

    parseSumType(): ParseResult<Model.SumType> {
        return this.withContext('sum type', () => {
            const members: Model.Type[] = [];

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            const firstMemberResult = this.parseType();
            if (!firstMemberResult.success) return firstMemberResult;
            members.push(firstMemberResult.value);

            while (this.consumeString('|')) {
                const memberResult = this.parseType();
                if (!memberResult.success) return memberResult;
                members.push(memberResult.value);
            }

            const closeBrace = this.mustConsumeString('}');
            if (!closeBrace.success) return closeBrace;

            return this.success(new Model.SumType({ members }));
        });
    }

    parseProductType(): ParseResult<Model.ProductType> {
        return this.withContext('product type', () => {
            const members: Model.ProductMember[] = [];

            const openBrace = this.mustConsumeString('{');
            if (!openBrace.success) return openBrace;

            const firstMemberResult = this.optional(() => this.parseProductMember());
            if (!firstMemberResult.success) return firstMemberResult;
            if (firstMemberResult.value) members.push(firstMemberResult.value);

            while (this.consumeString(',')) {
                const memberResult = this.parseProductMember();
                if (!memberResult.success) return memberResult;
                members.push(memberResult.value);
            }

            const closeBrace = this.mustConsumeString('}');
            if (!closeBrace.success) return closeBrace;

            return this.success(new Model.ProductType({ members }));
        });
    }

    parseProductMember(): ParseResult<Model.ProductMember> {
        return this.withContext('product member', () => {
            let name: Id;
            let type: Model.Type;

            const nameResult = this.parseId();
            if (!nameResult.success) return nameResult;
            name = nameResult.value;

            const colon = this.mustConsumeString(':');
            if (!colon.success) return colon;

            const typeResult = this.parseType();
            if (!typeResult.success) return typeResult;
            type = typeResult.value;

            return this.success(new Model.ProductMember({ name, type }));
        });
    }

    parseGenericType(): ParseResult<Model.GenericType> {
        return this.withContext('generic type', () => {
            return this.firstAlternative(
                'generic type',
                () => this.parseMapType(),
                () => this.parseSetType(),
                () => this.parseSequenceType(),
                () => this.parseOptionType()
            );
        });
    }

    parseMapType(): ParseResult<Model.MapType> {
        return this.withContext('map type', () => {
            let keyType: Model.Type;
            let valueType: Model.Type;

            const openAngle = this.mustConsumeString('map<');
            if (!openAngle.success) return openAngle;

            const keyTypeResult = this.parseType();
            if (!keyTypeResult.success) return keyTypeResult;
            keyType = keyTypeResult.value;

            const comma = this.mustConsumeString(',');
            if (!comma.success) return comma;

            const valueTypeResult = this.parseType();
            if (!valueTypeResult.success) return valueTypeResult;
            valueType = valueTypeResult.value;

            const closeAngle = this.mustConsumeString('>');
            if (!closeAngle.success) return closeAngle;

            return this.success(new Model.MapType({ keyType, valueType }));
        });
    }

    parseSetType(): ParseResult<Model.SetType> {
        return this.withContext('set type', () => {
            let keyType: Model.Type;

            const openAngle = this.mustConsumeString('set<');
            if (!openAngle.success) return openAngle;

            const keyTypeResult = this.parseType();
            if (!keyTypeResult.success) return keyTypeResult;
            keyType = keyTypeResult.value;

            const closeAngle = this.mustConsumeString('>');
            if (!closeAngle.success) return closeAngle;

            return this.success(new Model.SetType({ keyType }));
        });
    }

    parseSequenceType(): ParseResult<Model.SequenceType> {
        return this.withContext('sequence type', () => {
            let elementType: Model.Type;

            const openAngle = this.mustConsumeString('seq<');
            if (!openAngle.success) return openAngle;

            const elementTypeResult = this.parseType();
            if (!elementTypeResult.success) return elementTypeResult;
            elementType = elementTypeResult.value;

            const closeAngle = this.mustConsumeString('>');
            if (!closeAngle.success) return closeAngle;

            return this.success(new Model.SequenceType({ elementType }));
        });
    }

    parseOptionType(): ParseResult<Model.OptionType> {
        return this.withContext('option type', () => {
            let type: Model.Type;

            const openAngle = this.mustConsumeString('option<');
            if (!openAngle.success) return openAngle;

            const typeResult = this.parseType();
            if (!typeResult.success) return typeResult;
            type = typeResult.value;

            const closeAngle = this.mustConsumeString('>');
            if (!closeAngle.success) return closeAngle;

            return this.success(new Model.OptionType({ type }));
        });
    }

    parseNamedTypeReference(): ParseResult<Model.NamedTypeReference> {
        return this.withContext('named type reference', () => {
            const names: Id[] = [];

            const firstNameResult = this.parseId();
            if (!firstNameResult.success) return firstNameResult;
            names.push(firstNameResult.value);

            while (this.consumeString('::')) {
                const nameResult = this.parseId();
                if (!nameResult.success) return nameResult;
                names.push(nameResult.value);
            }

            return this.success(new Model.NamedTypeReference({ fqn: names }));
        });
    }

    parseId(): ParseResult<Id> {
        return this.mustConsumeRegex(/^[a-zA-Z_][a-zA-Z0-9_]*/, 'id');
    }

    protected consumeTrivia(): string | undefined {
        return this.parseTrivia();
    }

    protected consumeIdentifierForKeyword(): string | undefined {
        return this.consumeRegex(/^[a-zA-Z_][a-zA-Z0-9_]*/);
    }

    private parseTrivia(): TriviaKind | undefined {
        if (this.parseLineComment()) return 'LineComment';
        if (this.parseBlockComment()) return 'BlockComment';
        if (this.parseWhitespace()) return 'Whitespace';
        return undefined;
    }

    private parseLineComment(): boolean {
        if (this.consumeString('//') === undefined) return false;
        this.consumeWhile((c) => c !== '\n');
        this.consumeString('\n');
        return true;
    }

    private parseBlockComment(): boolean {
        if (this.consumeString('/*') === undefined) return false;
        while (
            this.consumeRegex(/[^*]+/) !== undefined ||
            (this.consumeString('*') !== undefined && this.peek() !== '/')
        ) {}
        if (this.consumeString('/') === undefined) return false;
        return true;
    }

    private parseWhitespace(): boolean {
        return this.consumeRegex(/[\n\t ]+/) !== undefined;
    }
}
