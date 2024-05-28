// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { DiagnosticInterface } from "../diagnostic";
import * as generated from "../napi-bindings/generated";

export const ParseError = generated.parse_error.ParseError;
export type ParseError = generated.parse_error.ParseError;

type AssertImplements<T, _U extends T> = void;
declare const assertDiagnosticInterface: AssertImplements<DiagnosticInterface, ParseError>;
