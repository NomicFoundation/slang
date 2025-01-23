import { IndentingOutputStream } from '../../../../../output/indentingOutputStream';
import * as Model from '../model';
import { Visitor } from '../visitor';

export class ParsedModelToSource {
    transform(model: Model.Model): string {
        const generator = new Generator();
        generator.visitModel(model);
        return generator.output.toString().trim();
    }
}

class Generator extends Visitor {
    output = new IndentingOutputStream();

    visitModel(model: Model.Model) {
        this.output.write(`model ${model.name.join('::')}`);
        if (model.parent) {
            this.output.write(` modifies ${model.parent?.name.join('::')}`);
        }
        this.output.writeLine(' {');
        this.output.writeLine();

        this.output.indentDuring(() => {
            super.visitModel(model);
        });

        this.output.writeLine('}');

        const result = this.output.toString();
        return result;
    }

    visitDefinition(definition: Model.Definition) {
        this.output.write(`${definition.name} = `);
        this.visitType(definition.type);
        this.output.writeLine(';');
        this.output.writeLine();
    }

    visitVoidType(voidType: Model.VoidType) {
        this.output.write('()');
    }

    visitPrimitiveType(primitiveType: Model.PrimitiveType) {
        switch (primitiveType) {
            case Model.PrimitiveType.Boolean:
                this.output.write('boolean');
                break;
            case Model.PrimitiveType.Char:
                this.output.write('char');
                break;
            case Model.PrimitiveType.String:
                this.output.write('string');
                break;
            case Model.PrimitiveType.I8:
                this.output.write('i8');
                break;
            case Model.PrimitiveType.I16:
                this.output.write('i16');
                break;
            case Model.PrimitiveType.I32:
                this.output.write('i32');
                break;
            case Model.PrimitiveType.I64:
                this.output.write('i64');
                break;
            case Model.PrimitiveType.U8:
                this.output.write('u8');
                break;
            case Model.PrimitiveType.U16:
                this.output.write('u16');
                break;
            case Model.PrimitiveType.U32:
                this.output.write('u32');
                break;
            case Model.PrimitiveType.U64:
                this.output.write('u64');
                break;
            case Model.PrimitiveType.F32:
                this.output.write('f32');
                break;
            case Model.PrimitiveType.F64:
                this.output.write('f64');
                break;
        }
    }

    visitEnumType(enumType: Model.EnumType) {
        if (enumType.members.length > 1) {
            this.output.writeLine('{');
            this.output.indentDuring(() => {
                this.output.joinLinesPrefixing(enumType.members, '| ', (member) => this.output.write(`"${member}"`));
            });
            this.output.write('}');
        } else {
            this.output.write('{ ');
            this.output.join(enumType.members, ' | ', (member) => this.output.write(`"${member}"`));
            this.output.write(' }');
        }
    }

    visitSumType(sumType: Model.SumType) {
        if (sumType.members.length > 1) {
            this.output.writeLine('{');
            this.output.indentDuring(() => {
                this.output.joinLinesPrefixing(sumType.members, '| ', (member) => this.visitType(member));
            });
            this.output.write('}');
        } else {
            this.output.write('{ ');
            this.output.join(sumType.members, ' | ', (member) => this.visitType(member));
            this.output.write(' }');
        }
    }

    visitProductType(productType: Model.ProductType) {
        if (productType.members.length > 1) {
            this.output.writeLine('{');
            this.output.indentDuring(() => {
                this.output.joinLinesSeparating(productType.members, ',', (member) => {
                    this.output.write(`${member.name}: `);
                    this.visitType(member.type);
                });
            });
            this.output.write('}');
        } else {
            this.output.write('{ ');
            this.output.join(productType.members, ', ', (member) => {
                this.output.write(`${member.name}: `);
                this.visitType(member.type);
            });
            this.output.write(' }');
        }
    }

    visitMapType(mapType: Model.MapType) {
        this.output.write('map<');
        this.visitType(mapType.keyType);
        this.output.write(', ');
        this.visitType(mapType.valueType);
        this.output.write('>');
    }

    visitSetType(setType: Model.SetType) {
        this.output.write('set<');
        this.visitType(setType.keyType);
        this.output.write('>');
    }

    visitSequenceType(sequenceType: Model.SequenceType) {
        this.output.write('seq<');
        this.visitType(sequenceType.elementType);
        this.output.write('>');
    }

    visitOptionType(optionType: Model.OptionType) {
        this.output.write('option<');
        this.visitType(optionType.type);
        this.output.write('>');
    }

    visitNamedTypeReference(namedTypeReference: Model.NamedTypeReference) {
        this.output.write(namedTypeReference.fqn.join('::'));
    }
}
