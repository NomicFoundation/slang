// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as generated from "../napi-bindings/generated";

export const Severity = generated.diagnostic.Severity;
export type Severity = generated.diagnostic.Severity;

export const Diagnostic = generated.diagnostic.Diagnostic;
export type Diagnostic = generated.diagnostic.Diagnostic;

// NOTE(#987): napi-rs does not allow us to either export traits as interfaces
// or interfaces with methods in general, so we define the interface ourselves.
export interface DiagnosticInterface {
  textRange(): generated.cst.TextRange;
  message(): string;
  severity(): Severity;
}

type AssertImplements<T, _U extends T> = void;
declare const assertDiagnosticInterface: AssertImplements<DiagnosticInterface, Diagnostic>;
