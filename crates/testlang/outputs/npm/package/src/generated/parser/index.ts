// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as generated from "../napi-bindings/generated";
import { DiagnosticInterface } from "../diagnostic";

export const Parser = generated.parser.Parser;
export type Parser = generated.parser.Parser;

export const ParseError = generated.parser.ParseError;
export type ParseError = generated.parser.ParseError;

type AssertImplements<T, _U extends T> = void;
declare const assertDiagnosticInterface: AssertImplements<DiagnosticInterface, ParseError>;

export const ParseOutput = generated.parser.ParseOutput;
export type ParseOutput = generated.parser.ParseOutput;
