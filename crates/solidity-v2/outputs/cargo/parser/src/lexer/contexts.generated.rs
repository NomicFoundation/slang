// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use logos::{Lexer, Logos};
use slang_solidity_v2_ast::ast::lexemes::{Lexeme, LexemeKind};
use slang_solidity_v2_common::versions::LanguageVersion;

#[derive(Clone, Copy, Debug)]
pub enum ContextKind {
    Pragma,
    Solidity,
    Yul,
}

#[derive(Clone)]
pub struct ContextExtras {
    pub language_version: LanguageVersion,
}

#[derive(Clone)]
pub enum ContextWrapper<'source> {
    Pragma(Lexer<'source, PragmaContext>),
    Solidity(Lexer<'source, SolidityContext>),
    Yul(Lexer<'source, YulContext>),
}

impl<'source> ContextWrapper<'source> {
    pub fn new(kind: ContextKind, source: &'source str, extras: ContextExtras) -> Self {
        match kind {
            ContextKind::Pragma => Self::Pragma(PragmaContext::lexer_with_extras(source, extras)),
            ContextKind::Solidity => {
                Self::Solidity(SolidityContext::lexer_with_extras(source, extras))
            }
            ContextKind::Yul => Self::Yul(YulContext::lexer_with_extras(source, extras)),
        }
    }

    pub fn source(&self) -> &'source str {
        match self {
            Self::Pragma(lexer) => lexer.source(),
            Self::Solidity(lexer) => lexer.source(),
            Self::Yul(lexer) => lexer.source(),
        }
    }

    pub fn bump(&mut self, n: usize) {
        match self {
            Self::Pragma(lexer) => lexer.bump(n),
            Self::Solidity(lexer) => lexer.bump(n),
            Self::Yul(lexer) => lexer.bump(n),
        }
    }

    #[must_use]
    pub fn morph(self, kind: ContextKind) -> Self {
        match self {
            Self::Pragma(lexer) => match kind {
                ContextKind::Pragma => Self::Pragma(lexer),
                ContextKind::Solidity => Self::Solidity(lexer.morph()),
                ContextKind::Yul => Self::Yul(lexer.morph()),
            },
            Self::Solidity(lexer) => match kind {
                ContextKind::Pragma => Self::Pragma(lexer.morph()),
                ContextKind::Solidity => Self::Solidity(lexer),
                ContextKind::Yul => Self::Yul(lexer.morph()),
            },
            Self::Yul(lexer) => match kind {
                ContextKind::Pragma => Self::Pragma(lexer.morph()),
                ContextKind::Solidity => Self::Solidity(lexer.morph()),
                ContextKind::Yul => Self::Yul(lexer),
            },
        }
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        let (kind, range) = match self {
            Self::Pragma(lexer) => match lexer.next() {
                Some(Ok(PragmaContext::Lexeme(kind))) => (kind, lexer.span()),
                Some(Err(())) => (LexemeKind::UNRECOGNIZED, lexer.span()),
                None => return None,
            },
            Self::Solidity(lexer) => match lexer.next() {
                Some(Ok(SolidityContext::Lexeme(kind))) => (kind, lexer.span()),
                Some(Err(())) => (LexemeKind::UNRECOGNIZED, lexer.span()),
                None => return None,
            },
            Self::Yul(lexer) => match lexer.next() {
                Some(Ok(YulContext::Lexeme(kind))) => (kind, lexer.span()),
                Some(Err(())) => (LexemeKind::UNRECOGNIZED, lexer.span()),
                None => return None,
            },
        };

        Some(Lexeme { kind, range })
    }
}

#[derive(Clone, Debug, Logos)]
#[logos(extras = ContextExtras)]
pub enum PragmaContext {
    #[regex(r#"(([0-9]|x|X|\*)+)"#, |_| { LexemeKind::VersionSpecifier }, priority = 2000001)]
    #[regex(r#"'(([0-9]|x|X|\*)+)(\.(([0-9]|x|X|\*)+))*'"#, |_| { LexemeKind::SingleQuotedVersionLiteral }, priority = 2000002)]
    #[regex(r#""(([0-9]|x|X|\*)+)(\.(([0-9]|x|X|\*)+))*""#, |_| { LexemeKind::DoubleQuotedVersionLiteral }, priority = 2000003)]
    #[regex(r#"( |\t)+"#, |_| { LexemeKind::Whitespace }, priority = 1000004, allow_greedy = true)]
    #[regex(r#"\n|(\r\n?)"#, |_| { LexemeKind::EndOfLine }, priority = 1000005, allow_greedy = true)]
    #[regex(r#"//[^\r\n]*"#, |_| { LexemeKind::SingleLineComment }, priority = 1000006, allow_greedy = true)]
    #[regex(r#"/\*[^\*]*\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineComment }, priority = 1000007, allow_greedy = true)]
    #[regex(r#"///[^\r\n]*"#, |_| { LexemeKind::SingleLineNatSpecComment }, priority = 1000008, allow_greedy = true)]
    #[regex(r#"/\*\*([^/\*][^\*]*)?\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineNatSpecComment }, priority = 1000009, allow_greedy = true)]
    Lexeme(LexemeKind),
}

#[derive(Clone, Debug, Logos)]
#[logos(extras = ContextExtras)]
pub enum SolidityContext {
    #[regex(r#"abicoder"#, |_| { LexemeKind::AbicoderKeyword_Unreserved }, priority = 3000001)]
    #[regex(r#"v1"#, |_| { LexemeKind::AbicoderV1Keyword_Unreserved }, priority = 3000002)]
    #[regex(r#"v2"#, |_| { LexemeKind::AbicoderV2Keyword_Unreserved }, priority = 3000003)]
    #[regex(r#"ABIEncoderV2"#, |_| { LexemeKind::ABIEncoderV2Keyword_Unreserved }, priority = 3000004)]
    #[regex(r#"abstract"#, |_| { LexemeKind::AbstractKeyword_Reserved }, priority = 3000005)]
    #[regex(r#"address"#, |_| { LexemeKind::AddressKeyword_Unreserved }, priority = 3000006)]
    #[regex(r#"after"#, |_| { LexemeKind::AfterKeyword_Reserved }, priority = 3000007)]
    #[regex(r#"alias"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::AliasKeyword_Reserved } else { LexemeKind::AliasKeyword_Unreserved } }, priority = 3000008)]
    #[regex(r#"anonymous"#, |_| { LexemeKind::AnonymousKeyword_Reserved }, priority = 3000009)]
    #[regex(r#"apply"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::ApplyKeyword_Reserved } else { LexemeKind::ApplyKeyword_Unreserved } }, priority = 3000010)]
    #[regex(r#"as"#, |_| { LexemeKind::AsKeyword_Reserved }, priority = 3000011)]
    #[regex(r#"assembly"#, |_| { LexemeKind::AssemblyKeyword_Reserved }, priority = 3000012)]
    #[regex(r#"at"#, |_| { LexemeKind::AtKeyword_Unreserved }, priority = 3000013)]
    #[regex(r#"auto"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::AutoKeyword_Reserved } else { LexemeKind::AutoKeyword_Unreserved } }, priority = 3000014)]
    #[regex(r#"bool"#, |_| { LexemeKind::BoolKeyword_Reserved }, priority = 3000015)]
    #[regex(r#"break"#, |_| { LexemeKind::BreakKeyword_Reserved }, priority = 3000016)]
    #[regex(r#"byte"#, |_| { LexemeKind::ByteKeyword_Reserved }, priority = 3000017)]
    #[regex(r#"bytes(1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32)?"#, |_| { LexemeKind::BytesKeyword_Reserved }, priority = 3000018)]
    #[regex(r#"calldata"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::CallDataKeyword_Reserved } else { LexemeKind::CallDataKeyword_Unreserved } }, priority = 3000019)]
    #[regex(r#"case"#, |_| { LexemeKind::CaseKeyword_Reserved }, priority = 3000020)]
    #[regex(r#"catch"#, |_| { LexemeKind::CatchKeyword_Reserved }, priority = 3000021)]
    #[regex(r#"constant"#, |_| { LexemeKind::ConstantKeyword_Reserved }, priority = 3000022)]
    #[regex(r#"constructor"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::ConstructorKeyword_Reserved } else { LexemeKind::ConstructorKeyword_Unreserved } }, priority = 3000023)]
    #[regex(r#"continue"#, |_| { LexemeKind::ContinueKeyword_Reserved }, priority = 3000024)]
    #[regex(r#"contract"#, |_| { LexemeKind::ContractKeyword_Reserved }, priority = 3000025)]
    #[regex(r#"copyof"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::CopyOfKeyword_Reserved } else { LexemeKind::CopyOfKeyword_Unreserved } }, priority = 3000026)]
    #[regex(r#"days"#, |_| { LexemeKind::DaysKeyword_Reserved }, priority = 3000027)]
    #[regex(r#"default"#, |_| { LexemeKind::DefaultKeyword_Reserved }, priority = 3000028)]
    #[regex(r#"define"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::DefineKeyword_Reserved } else { LexemeKind::DefineKeyword_Unreserved } }, priority = 3000029)]
    #[regex(r#"delete"#, |_| { LexemeKind::DeleteKeyword_Reserved }, priority = 3000030)]
    #[regex(r#"do"#, |_| { LexemeKind::DoKeyword_Reserved }, priority = 3000031)]
    #[regex(r#"else"#, |_| { LexemeKind::ElseKeyword_Reserved }, priority = 3000032)]
    #[regex(r#"emit"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::EmitKeyword_Reserved } else { LexemeKind::EmitKeyword_Unreserved } }, priority = 3000033)]
    #[regex(r#"enum"#, |_| { LexemeKind::EnumKeyword_Reserved }, priority = 3000034)]
    #[regex(r#"error"#, |_| { LexemeKind::ErrorKeyword_Unreserved }, priority = 3000035)]
    #[regex(r#"ether"#, |_| { LexemeKind::EtherKeyword_Reserved }, priority = 3000036)]
    #[regex(r#"event"#, |_| { LexemeKind::EventKeyword_Reserved }, priority = 3000037)]
    #[regex(r#"experimental"#, |_| { LexemeKind::ExperimentalKeyword_Unreserved }, priority = 3000038)]
    #[regex(r#"external"#, |_| { LexemeKind::ExternalKeyword_Reserved }, priority = 3000039)]
    #[regex(r#"fallback"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version { LexemeKind::FallbackKeyword_Reserved } else { LexemeKind::FallbackKeyword_Unreserved } }, priority = 3000040)]
    #[regex(r#"false"#, |_| { LexemeKind::FalseKeyword_Reserved }, priority = 3000041)]
    #[regex(r#"final"#, |_| { LexemeKind::FinalKeyword_Reserved }, priority = 3000042)]
    #[regex(r#"finney"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_0 { LexemeKind::FinneyKeyword_Reserved } else { LexemeKind::FinneyKeyword_Unreserved } }, priority = 3000043)]
    #[regex(r#"fixed"#, |_| { LexemeKind::FixedKeyword_Reserved }, priority = 3000044)]
    #[regex(r#"fixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176)x(8|16|24|32|40|48|56|64|72|80)"#, |_| { LexemeKind::FixedKeyword_Reserved }, priority = 3000045)]
    #[regex(r#"fixed(184x8|184x16|184x24|184x32|184x40|184x48|184x56|184x64|184x72|192x8|192x16|192x24|192x32|192x40|192x48|192x56|192x64|200x8|200x16|200x24|200x32|200x40|200x48|200x56|208x8|208x16|208x24|208x32|208x40|208x48|216x8|216x16|216x24|216x32|216x40|224x8|224x16|224x24|224x32|232x8|232x16|232x24|240x8|240x16|248x8)"#, |_| { LexemeKind::FixedKeyword_Reserved }, priority = 3000046)]
    #[regex(r#"fixed(184x80|192x72|192x80|200x64|200x72|200x80|208x56|208x64|208x72|208x80|216x48|216x56|216x64|216x72|216x80|224x40|224x48|224x56|224x64|224x72|224x80|232x32|232x40|232x48|232x56|232x64|232x72|232x80|240x24|240x32|240x40|240x48|240x56|240x64|240x72|240x80|248x16|248x24|248x32|248x40|248x48|248x56|248x64|248x72|248x80|256x8|256x16|256x24|256x32|256x40|256x48|256x56|256x64|256x72|256x80)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version { LexemeKind::FixedKeyword_Reserved } else { LexemeKind::FixedKeyword_Unreserved } }, priority = 3000047)]
    #[regex(r#"fixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)x(0|1|2|3|4|5|6|7|9|10|11|12|13|14|15|17|18|19|20|21|22|23|25|26|27|28|29|30|31|33|34|35|36|37|38|39|41|42|43|44|45|46|47|49|50|51|52|53|54|55|57|58|59|60|61|62|63|65|66|67|68|69|70|71|73|74|75|76|77|78|79)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version { LexemeKind::FixedKeyword_Reserved } else { LexemeKind::FixedKeyword_Unreserved } }, priority = 3000048)]
    #[regex(r#"for"#, |_| { LexemeKind::ForKeyword_Reserved }, priority = 3000049)]
    #[regex(r#"from"#, |_| { LexemeKind::FromKeyword_Unreserved }, priority = 3000050)]
    #[regex(r#"function"#, |_| { LexemeKind::FunctionKeyword_Reserved }, priority = 3000051)]
    #[regex(r#"global"#, |_| { LexemeKind::GlobalKeyword_Unreserved }, priority = 3000052)]
    #[regex(r#"gwei"#, |lexer| { if LanguageVersion::V0_7_0 <= lexer.extras.language_version { LexemeKind::GweiKeyword_Reserved } else { LexemeKind::GweiKeyword_Unreserved } }, priority = 3000053)]
    #[regex(r#"hex"#, |_| { LexemeKind::HexKeyword_Reserved }, priority = 3000054)]
    #[regex(r#"hours"#, |_| { LexemeKind::HoursKeyword_Reserved }, priority = 3000055)]
    #[regex(r#"if"#, |_| { LexemeKind::IfKeyword_Reserved }, priority = 3000056)]
    #[regex(r#"immutable"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::ImmutableKeyword_Reserved } else { LexemeKind::ImmutableKeyword_Unreserved } }, priority = 3000057)]
    #[regex(r#"implements"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::ImplementsKeyword_Reserved } else { LexemeKind::ImplementsKeyword_Unreserved } }, priority = 3000058)]
    #[regex(r#"import"#, |_| { LexemeKind::ImportKeyword_Reserved }, priority = 3000059)]
    #[regex(r#"indexed"#, |_| { LexemeKind::IndexedKeyword_Reserved }, priority = 3000060)]
    #[regex(r#"in"#, |_| { LexemeKind::InKeyword_Reserved }, priority = 3000061)]
    #[regex(r#"inline"#, |_| { LexemeKind::InlineKeyword_Reserved }, priority = 3000062)]
    #[regex(r#"interface"#, |_| { LexemeKind::InterfaceKeyword_Reserved }, priority = 3000063)]
    #[regex(r#"internal"#, |_| { LexemeKind::InternalKeyword_Reserved }, priority = 3000064)]
    #[regex(r#"int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?"#, |_| { LexemeKind::IntKeyword_Reserved }, priority = 3000065)]
    #[regex(r#"is"#, |_| { LexemeKind::IsKeyword_Reserved }, priority = 3000066)]
    #[regex(r#"layout"#, |_| { LexemeKind::LayoutKeyword_Unreserved }, priority = 3000067)]
    #[regex(r#"let"#, |_| { LexemeKind::LetKeyword_Reserved }, priority = 3000068)]
    #[regex(r#"library"#, |_| { LexemeKind::LibraryKeyword_Reserved }, priority = 3000069)]
    #[regex(r#"macro"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::MacroKeyword_Reserved } else { LexemeKind::MacroKeyword_Unreserved } }, priority = 3000070)]
    #[regex(r#"mapping"#, |_| { LexemeKind::MappingKeyword_Reserved }, priority = 3000071)]
    #[regex(r#"match"#, |_| { LexemeKind::MatchKeyword_Reserved }, priority = 3000072)]
    #[regex(r#"memory"#, |_| { LexemeKind::MemoryKeyword_Reserved }, priority = 3000073)]
    #[regex(r#"minutes"#, |_| { LexemeKind::MinutesKeyword_Reserved }, priority = 3000074)]
    #[regex(r#"modifier"#, |_| { LexemeKind::ModifierKeyword_Reserved }, priority = 3000075)]
    #[regex(r#"mutable"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::MutableKeyword_Reserved } else { LexemeKind::MutableKeyword_Unreserved } }, priority = 3000076)]
    #[regex(r#"new"#, |_| { LexemeKind::NewKeyword_Reserved }, priority = 3000077)]
    #[regex(r#"null"#, |_| { LexemeKind::NullKeyword_Reserved }, priority = 3000078)]
    #[regex(r#"of"#, |_| { LexemeKind::OfKeyword_Reserved }, priority = 3000079)]
    #[regex(r#"override"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::OverrideKeyword_Reserved } else { LexemeKind::OverrideKeyword_Unreserved } }, priority = 3000080)]
    #[regex(r#"partial"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::PartialKeyword_Reserved } else { LexemeKind::PartialKeyword_Unreserved } }, priority = 3000081)]
    #[regex(r#"payable"#, |_| { LexemeKind::PayableKeyword_Reserved }, priority = 3000082)]
    #[regex(r#"pragma"#, |_| { LexemeKind::PragmaKeyword_Reserved }, priority = 3000083)]
    #[regex(r#"private"#, |_| { LexemeKind::PrivateKeyword_Reserved }, priority = 3000084)]
    #[regex(r#"promise"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::PromiseKeyword_Reserved } else { LexemeKind::PromiseKeyword_Unreserved } }, priority = 3000085)]
    #[regex(r#"public"#, |_| { LexemeKind::PublicKeyword_Reserved }, priority = 3000086)]
    #[regex(r#"pure"#, |_| { LexemeKind::PureKeyword_Reserved }, priority = 3000087)]
    #[regex(r#"receive"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version { LexemeKind::ReceiveKeyword_Reserved } else { LexemeKind::ReceiveKeyword_Unreserved } }, priority = 3000088)]
    #[regex(r#"reference"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::ReferenceKeyword_Reserved } else { LexemeKind::ReferenceKeyword_Unreserved } }, priority = 3000089)]
    #[regex(r#"relocatable"#, |_| { LexemeKind::RelocatableKeyword_Reserved }, priority = 3000090)]
    #[regex(r#"return"#, |_| { LexemeKind::ReturnKeyword_Reserved }, priority = 3000091)]
    #[regex(r#"returns"#, |_| { LexemeKind::ReturnsKeyword_Reserved }, priority = 3000092)]
    #[regex(r#"revert"#, |_| { LexemeKind::RevertKeyword_Unreserved }, priority = 3000093)]
    #[regex(r#"sealed"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::SealedKeyword_Reserved } else { LexemeKind::SealedKeyword_Unreserved } }, priority = 3000094)]
    #[regex(r#"seconds"#, |_| { LexemeKind::SecondsKeyword_Reserved }, priority = 3000095)]
    #[regex(r#"sizeof"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::SizeOfKeyword_Reserved } else { LexemeKind::SizeOfKeyword_Unreserved } }, priority = 3000096)]
    #[regex(r#"SMTChecker"#, |_| { LexemeKind::SMTCheckerKeyword_Unreserved }, priority = 3000097)]
    #[regex(r#"solidity"#, |_| { LexemeKind::SolidityKeyword_Unreserved }, priority = 3000098)]
    #[regex(r#"static"#, |_| { LexemeKind::StaticKeyword_Reserved }, priority = 3000099)]
    #[regex(r#"storage"#, |_| { LexemeKind::StorageKeyword_Reserved }, priority = 3000100)]
    #[regex(r#"string"#, |_| { LexemeKind::StringKeyword_Reserved }, priority = 3000101)]
    #[regex(r#"struct"#, |_| { LexemeKind::StructKeyword_Reserved }, priority = 3000102)]
    #[regex(r#"super"#, |lexer| { if LanguageVersion::V0_8_0 <= lexer.extras.language_version { LexemeKind::SuperKeyword_Reserved } else { LexemeKind::SuperKeyword_Unreserved } }, priority = 3000103)]
    #[regex(r#"supports"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::SupportsKeyword_Reserved } else { LexemeKind::SupportsKeyword_Unreserved } }, priority = 3000104)]
    #[regex(r#"switch"#, |_| { LexemeKind::SwitchKeyword_Reserved }, priority = 3000105)]
    #[regex(r#"szabo"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_0 { LexemeKind::SzaboKeyword_Reserved } else { LexemeKind::SzaboKeyword_Unreserved } }, priority = 3000106)]
    #[regex(r#"this"#, |lexer| { if LanguageVersion::V0_8_0 <= lexer.extras.language_version { LexemeKind::ThisKeyword_Reserved } else { LexemeKind::ThisKeyword_Unreserved } }, priority = 3000107)]
    #[regex(r#"throw"#, |_| { LexemeKind::ThrowKeyword_Reserved }, priority = 3000108)]
    #[regex(r#"transient"#, |_| { LexemeKind::TransientKeyword_Unreserved }, priority = 3000109)]
    #[regex(r#"true"#, |_| { LexemeKind::TrueKeyword_Reserved }, priority = 3000110)]
    #[regex(r#"try"#, |_| { LexemeKind::TryKeyword_Reserved }, priority = 3000111)]
    #[regex(r#"typedef"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::TypeDefKeyword_Reserved } else { LexemeKind::TypeDefKeyword_Unreserved } }, priority = 3000112)]
    #[regex(r#"type"#, |_| { LexemeKind::TypeKeyword_Reserved }, priority = 3000113)]
    #[regex(r#"typeof"#, |_| { LexemeKind::TypeOfKeyword_Reserved }, priority = 3000114)]
    #[regex(r#"ufixed"#, |_| { LexemeKind::UfixedKeyword_Reserved }, priority = 3000115)]
    #[regex(r#"ufixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176)x(8|16|24|32|40|48|56|64|72|80)"#, |_| { LexemeKind::UfixedKeyword_Reserved }, priority = 3000116)]
    #[regex(r#"ufixed(184x8|184x16|184x24|184x32|184x40|184x48|184x56|184x64|184x72|192x8|192x16|192x24|192x32|192x40|192x48|192x56|192x64|200x8|200x16|200x24|200x32|200x40|200x48|200x56|208x8|208x16|208x24|208x32|208x40|208x48|216x8|216x16|216x24|216x32|216x40|224x8|224x16|224x24|224x32|232x8|232x16|232x24|240x8|240x16|248x8)"#, |_| { LexemeKind::UfixedKeyword_Reserved }, priority = 3000117)]
    #[regex(r#"ufixed(184x80|192x72|192x80|200x64|200x72|200x80|208x56|208x64|208x72|208x80|216x48|216x56|216x64|216x72|216x80|224x40|224x48|224x56|224x64|224x72|224x80|232x32|232x40|232x48|232x56|232x64|232x72|232x80|240x24|240x32|240x40|240x48|240x56|240x64|240x72|240x80|248x16|248x24|248x32|248x40|248x48|248x56|248x64|248x72|248x80|256x8|256x16|256x24|256x32|256x40|256x48|256x56|256x64|256x72|256x80)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version { LexemeKind::UfixedKeyword_Reserved } else { LexemeKind::UfixedKeyword_Unreserved } }, priority = 3000118)]
    #[regex(r#"ufixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)x(0|1|2|3|4|5|6|7|9|10|11|12|13|14|15|17|18|19|20|21|22|23|25|26|27|28|29|30|31|33|34|35|36|37|38|39|41|42|43|44|45|46|47|49|50|51|52|53|54|55|57|58|59|60|61|62|63|65|66|67|68|69|70|71|73|74|75|76|77|78|79)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version { LexemeKind::UfixedKeyword_Reserved } else { LexemeKind::UfixedKeyword_Unreserved } }, priority = 3000119)]
    #[regex(r#"uint(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?"#, |_| { LexemeKind::UintKeyword_Reserved }, priority = 3000120)]
    #[regex(r#"unchecked"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version { LexemeKind::UncheckedKeyword_Reserved } else { LexemeKind::UncheckedKeyword_Unreserved } }, priority = 3000121)]
    #[regex(r#"using"#, |_| { LexemeKind::UsingKeyword_Reserved }, priority = 3000122)]
    #[regex(r#"var"#, |_| { LexemeKind::VarKeyword_Reserved }, priority = 3000123)]
    #[regex(r#"view"#, |_| { LexemeKind::ViewKeyword_Reserved }, priority = 3000124)]
    #[regex(r#"virtual"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version { LexemeKind::VirtualKeyword_Reserved } else { LexemeKind::VirtualKeyword_Unreserved } }, priority = 3000125)]
    #[regex(r#"weeks"#, |_| { LexemeKind::WeeksKeyword_Reserved }, priority = 3000126)]
    #[regex(r#"wei"#, |_| { LexemeKind::WeiKeyword_Reserved }, priority = 3000127)]
    #[regex(r#"while"#, |_| { LexemeKind::WhileKeyword_Reserved }, priority = 3000128)]
    #[regex(r#"years"#, |_| { LexemeKind::YearsKeyword_Reserved }, priority = 3000129)]
    #[regex(r#"\("#, |_| { LexemeKind::OpenParen }, priority = 2000130)]
    #[regex(r#"\)"#, |_| { LexemeKind::CloseParen }, priority = 2000131)]
    #[regex(r#"\["#, |_| { LexemeKind::OpenBracket }, priority = 2000132)]
    #[regex(r#"\]"#, |_| { LexemeKind::CloseBracket }, priority = 2000133)]
    #[regex(r#"\{"#, |_| { LexemeKind::OpenBrace }, priority = 2000134)]
    #[regex(r#"\}"#, |_| { LexemeKind::CloseBrace }, priority = 2000135)]
    #[regex(r#","#, |_| { LexemeKind::Comma }, priority = 2000136)]
    #[regex(r#"\."#, |_| { LexemeKind::Period }, priority = 2000137)]
    #[regex(r#"\?"#, |_| { LexemeKind::QuestionMark }, priority = 2000138)]
    #[regex(r#";"#, |_| { LexemeKind::Semicolon }, priority = 2000139)]
    #[regex(r#":"#, |_| { LexemeKind::Colon }, priority = 2000140)]
    #[regex(r#":="#, |_| { LexemeKind::ColonEqual }, priority = 2000141)]
    #[regex(r#"="#, |_| { LexemeKind::Equal }, priority = 2000142)]
    #[regex(r#"=:"#, |_| { LexemeKind::EqualColon }, priority = 2000143)]
    #[regex(r#"=="#, |_| { LexemeKind::EqualEqual }, priority = 2000144)]
    #[regex(r#"=>"#, |_| { LexemeKind::EqualGreaterThan }, priority = 2000145)]
    #[regex(r#"\*"#, |_| { LexemeKind::Asterisk }, priority = 2000146)]
    #[regex(r#"\*="#, |_| { LexemeKind::AsteriskEqual }, priority = 2000147)]
    #[regex(r#"\*\*"#, |_| { LexemeKind::AsteriskAsterisk }, priority = 2000148)]
    #[regex(r#"\|"#, |_| { LexemeKind::Bar }, priority = 2000149)]
    #[regex(r#"\|="#, |_| { LexemeKind::BarEqual }, priority = 2000150)]
    #[regex(r#"\|\|"#, |_| { LexemeKind::BarBar }, priority = 2000151)]
    #[regex(r#"&"#, |_| { LexemeKind::Ampersand }, priority = 2000152)]
    #[regex(r#"&="#, |_| { LexemeKind::AmpersandEqual }, priority = 2000153)]
    #[regex(r#"&&"#, |_| { LexemeKind::AmpersandAmpersand }, priority = 2000154)]
    #[regex(r#"<"#, |_| { LexemeKind::LessThan }, priority = 2000155)]
    #[regex(r#"<="#, |_| { LexemeKind::LessThanEqual }, priority = 2000156)]
    #[regex(r#"<<"#, |_| { LexemeKind::LessThanLessThan }, priority = 2000157)]
    #[regex(r#"<<="#, |_| { LexemeKind::LessThanLessThanEqual }, priority = 2000158)]
    #[regex(r#">"#, |_| { LexemeKind::GreaterThan }, priority = 2000159)]
    #[regex(r#">="#, |_| { LexemeKind::GreaterThanEqual }, priority = 2000160)]
    #[regex(r#">>"#, |_| { LexemeKind::GreaterThanGreaterThan }, priority = 2000161)]
    #[regex(r#">>="#, |_| { LexemeKind::GreaterThanGreaterThanEqual }, priority = 2000162)]
    #[regex(r#">>>"#, |_| { LexemeKind::GreaterThanGreaterThanGreaterThan }, priority = 2000163)]
    #[regex(r#">>>="#, |_| { LexemeKind::GreaterThanGreaterThanGreaterThanEqual }, priority = 2000164)]
    #[regex(r#"\+"#, |_| { LexemeKind::Plus }, priority = 2000165)]
    #[regex(r#"\+="#, |_| { LexemeKind::PlusEqual }, priority = 2000166)]
    #[regex(r#"\+\+"#, |_| { LexemeKind::PlusPlus }, priority = 2000167)]
    #[regex(r#"-"#, |_| { LexemeKind::Minus }, priority = 2000168)]
    #[regex(r#"-="#, |_| { LexemeKind::MinusEqual }, priority = 2000169)]
    #[regex(r#"--"#, |_| { LexemeKind::MinusMinus }, priority = 2000170)]
    #[regex(r#"->"#, |_| { LexemeKind::MinusGreaterThan }, priority = 2000171)]
    #[regex(r#"/"#, |_| { LexemeKind::Slash }, priority = 2000172)]
    #[regex(r#"/="#, |_| { LexemeKind::SlashEqual }, priority = 2000173)]
    #[regex(r#"%"#, |_| { LexemeKind::Percent }, priority = 2000174)]
    #[regex(r#"%="#, |_| { LexemeKind::PercentEqual }, priority = 2000175)]
    #[regex(r#"!"#, |_| { LexemeKind::Bang }, priority = 2000176)]
    #[regex(r#"!="#, |_| { LexemeKind::BangEqual }, priority = 2000177)]
    #[regex(r#"\^"#, |_| { LexemeKind::Caret }, priority = 2000178)]
    #[regex(r#"\^="#, |_| { LexemeKind::CaretEqual }, priority = 2000179)]
    #[regex(r#"~"#, |_| { LexemeKind::Tilde }, priority = 2000180)]
    #[regex(r#"0(x|X)(([0-9]|[a-f]|[A-F]))+(_(([0-9]|[a-f]|[A-F]))+)*"#, |_| { LexemeKind::HexLiteral }, priority = 2000181)]
    #[regex(r#"(([0-9]+(_[0-9]+)*))(\.(([0-9]+(_[0-9]+)*))?)?(((e|E)-?(([0-9]+(_[0-9]+)*))))?"#, |_| { LexemeKind::DecimalLiteral }, priority = 2000182)]
    #[regex(r#"\.(([0-9]+(_[0-9]+)*))(((e|E)-?(([0-9]+(_[0-9]+)*))))?"#, |_| { LexemeKind::DecimalLiteral }, priority = 2000183)]
    #[regex(r#"'(((\\([^xu]|((x(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))))|((u(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))))))|[^'\\\r\n])*'"#, |_| { LexemeKind::SingleQuotedStringLiteral }, priority = 2000184)]
    #[regex(r#""(((\\([^xu]|((x(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))))|((u(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))))))|[^"\\\r\n])*""#, |_| { LexemeKind::DoubleQuotedStringLiteral }, priority = 2000185)]
    #[regex(r#"hex'(((([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(_?(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))*))?'"#, |_| { LexemeKind::SingleQuotedHexStringLiteral }, priority = 2000186)]
    #[regex(r#"hex"(((([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(_?(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))*))?""#, |_| { LexemeKind::DoubleQuotedHexStringLiteral }, priority = 2000187)]
    #[regex(r#"unicode'(((\\(((n|r|t|'|"|\\|\r\n|\r|\n))|((x(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))))|((u(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))))))|[^'\\\r\n])*'"#, |_| { LexemeKind::SingleQuotedUnicodeStringLiteral }, priority = 2000188)]
    #[regex(r#"unicode"(((\\(((n|r|t|'|"|\\|\r\n|\r|\n))|((x(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))))|((u(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F]))(([0-9]|[a-f]|[A-F])))))))|[^"\\\r\n])*""#, |_| { LexemeKind::DoubleQuotedUnicodeStringLiteral }, priority = 2000189)]
    #[regex(r#"((_|\$|[a-z]|[A-Z]))((((_|\$|[a-z]|[A-Z]))|[0-9]))*"#, |_| { LexemeKind::Identifier }, priority = 2000190)]
    #[regex(r#"( |\t)+"#, |_| { LexemeKind::Whitespace }, priority = 1000191, allow_greedy = true)]
    #[regex(r#"\n|(\r\n?)"#, |_| { LexemeKind::EndOfLine }, priority = 1000192, allow_greedy = true)]
    #[regex(r#"//[^\r\n]*"#, |_| { LexemeKind::SingleLineComment }, priority = 1000193, allow_greedy = true)]
    #[regex(r#"/\*[^\*]*\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineComment }, priority = 1000194, allow_greedy = true)]
    #[regex(r#"///[^\r\n]*"#, |_| { LexemeKind::SingleLineNatSpecComment }, priority = 1000195, allow_greedy = true)]
    #[regex(r#"/\*\*([^/\*][^\*]*)?\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineNatSpecComment }, priority = 1000196, allow_greedy = true)]
    Lexeme(LexemeKind),
}

#[derive(Clone, Debug, Logos)]
#[logos(extras = ContextExtras)]
pub enum YulContext {
    #[regex(r#"((_|\$|[a-z]|[A-Z]))(((((_|\$|[a-z]|[A-Z]))|[0-9]))|\.)*"#, |_| { LexemeKind::YulIdentifier }, priority = 2000001)]
    #[regex(r#"0|([1-9][0-9]*)"#, |_| { LexemeKind::YulDecimalLiteral }, priority = 2000002)]
    #[regex(r#"0x(([0-9]|[a-f]|[A-F]))+"#, |_| { LexemeKind::YulHexLiteral }, priority = 2000003)]
    #[regex(r#"abstract"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAbstractKeyword_Reserved } else { LexemeKind::YulAbstractKeyword_Unreserved } }, priority = 3000004)]
    #[regex(r#"after"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAfterKeyword_Reserved } else { LexemeKind::YulAfterKeyword_Unreserved } }, priority = 3000005)]
    #[regex(r#"alias"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAliasKeyword_Reserved } else { LexemeKind::YulAliasKeyword_Unreserved } }, priority = 3000006)]
    #[regex(r#"anonymous"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAnonymousKeyword_Reserved } else { LexemeKind::YulAnonymousKeyword_Unreserved } }, priority = 3000007)]
    #[regex(r#"apply"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulApplyKeyword_Reserved } else { LexemeKind::YulApplyKeyword_Unreserved } }, priority = 3000008)]
    #[regex(r#"as"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAsKeyword_Reserved } else { LexemeKind::YulAsKeyword_Unreserved } }, priority = 3000009)]
    #[regex(r#"assembly"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAssemblyKeyword_Reserved } else { LexemeKind::YulAssemblyKeyword_Unreserved } }, priority = 3000010)]
    #[regex(r#"auto"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulAutoKeyword_Reserved } else { LexemeKind::YulAutoKeyword_Unreserved } }, priority = 3000011)]
    #[regex(r#"bool"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_5_10 { LexemeKind::YulBoolKeyword_Reserved } else { LexemeKind::YulBoolKeyword_Unreserved } }, priority = 3000012)]
    #[regex(r#"break"#, |_| { LexemeKind::YulBreakKeyword_Reserved }, priority = 3000013)]
    #[regex(r#"bytes(1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32)?"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulBytesKeyword_Reserved } else { LexemeKind::YulBytesKeyword_Unreserved } }, priority = 3000014)]
    #[regex(r#"calldata"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulCallDataKeyword_Reserved } else { LexemeKind::YulCallDataKeyword_Unreserved } }, priority = 3000015)]
    #[regex(r#"case"#, |_| { LexemeKind::YulCaseKeyword_Reserved }, priority = 3000016)]
    #[regex(r#"catch"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulCatchKeyword_Reserved } else { LexemeKind::YulCatchKeyword_Unreserved } }, priority = 3000017)]
    #[regex(r#"constant"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulConstantKeyword_Reserved } else { LexemeKind::YulConstantKeyword_Unreserved } }, priority = 3000018)]
    #[regex(r#"constructor"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulConstructorKeyword_Reserved } else { LexemeKind::YulConstructorKeyword_Unreserved } }, priority = 3000019)]
    #[regex(r#"continue"#, |_| { LexemeKind::YulContinueKeyword_Reserved }, priority = 3000020)]
    #[regex(r#"contract"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulContractKeyword_Reserved } else { LexemeKind::YulContractKeyword_Unreserved } }, priority = 3000021)]
    #[regex(r#"copyof"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulCopyOfKeyword_Reserved } else { LexemeKind::YulCopyOfKeyword_Unreserved } }, priority = 3000022)]
    #[regex(r#"days"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulDaysKeyword_Reserved } else { LexemeKind::YulDaysKeyword_Unreserved } }, priority = 3000023)]
    #[regex(r#"default"#, |_| { LexemeKind::YulDefaultKeyword_Reserved }, priority = 3000024)]
    #[regex(r#"define"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulDefineKeyword_Reserved } else { LexemeKind::YulDefineKeyword_Unreserved } }, priority = 3000025)]
    #[regex(r#"delete"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulDeleteKeyword_Reserved } else { LexemeKind::YulDeleteKeyword_Unreserved } }, priority = 3000026)]
    #[regex(r#"do"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulDoKeyword_Reserved } else { LexemeKind::YulDoKeyword_Unreserved } }, priority = 3000027)]
    #[regex(r#"else"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulElseKeyword_Reserved } else { LexemeKind::YulElseKeyword_Unreserved } }, priority = 3000028)]
    #[regex(r#"emit"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulEmitKeyword_Reserved } else { LexemeKind::YulEmitKeyword_Unreserved } }, priority = 3000029)]
    #[regex(r#"enum"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulEnumKeyword_Reserved } else { LexemeKind::YulEnumKeyword_Unreserved } }, priority = 3000030)]
    #[regex(r#"ether"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulEtherKeyword_Reserved } else { LexemeKind::YulEtherKeyword_Unreserved } }, priority = 3000031)]
    #[regex(r#"event"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulEventKeyword_Reserved } else { LexemeKind::YulEventKeyword_Unreserved } }, priority = 3000032)]
    #[regex(r#"external"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulExternalKeyword_Reserved } else { LexemeKind::YulExternalKeyword_Unreserved } }, priority = 3000033)]
    #[regex(r#"fallback"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFallbackKeyword_Reserved } else { LexemeKind::YulFallbackKeyword_Unreserved } }, priority = 3000034)]
    #[regex(r#"false"#, |_| { LexemeKind::YulFalseKeyword_Reserved }, priority = 3000035)]
    #[regex(r#"final"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFinalKeyword_Reserved } else { LexemeKind::YulFinalKeyword_Unreserved } }, priority = 3000036)]
    #[regex(r#"finney"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_0 { LexemeKind::YulFinneyKeyword_Reserved } else { LexemeKind::YulFinneyKeyword_Unreserved } }, priority = 3000037)]
    #[regex(r#"fixed"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFixedKeyword_Reserved } else { LexemeKind::YulFixedKeyword_Unreserved } }, priority = 3000038)]
    #[regex(r#"fixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176)x(8|16|24|32|40|48|56|64|72|80)"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFixedKeyword_Reserved } else { LexemeKind::YulFixedKeyword_Unreserved } }, priority = 3000039)]
    #[regex(r#"fixed(184x8|184x16|184x24|184x32|184x40|184x48|184x56|184x64|184x72|192x8|192x16|192x24|192x32|192x40|192x48|192x56|192x64|200x8|200x16|200x24|200x32|200x40|200x48|200x56|208x8|208x16|208x24|208x32|208x40|208x48|216x8|216x16|216x24|216x32|216x40|224x8|224x16|224x24|224x32|232x8|232x16|232x24|240x8|240x16|248x8)"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFixedKeyword_Reserved } else { LexemeKind::YulFixedKeyword_Unreserved } }, priority = 3000040)]
    #[regex(r#"fixed(184x80|192x72|192x80|200x64|200x72|200x80|208x56|208x64|208x72|208x80|216x48|216x56|216x64|216x72|216x80|224x40|224x48|224x56|224x64|224x72|224x80|232x32|232x40|232x48|232x56|232x64|232x72|232x80|240x24|240x32|240x40|240x48|240x56|240x64|240x72|240x80|248x16|248x24|248x32|248x40|248x48|248x56|248x64|248x72|248x80|256x8|256x16|256x24|256x32|256x40|256x48|256x56|256x64|256x72|256x80)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFixedKeyword_Reserved } else { LexemeKind::YulFixedKeyword_Unreserved } }, priority = 3000041)]
    #[regex(r#"fixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)x(0|1|2|3|4|5|6|7|9|10|11|12|13|14|15|17|18|19|20|21|22|23|25|26|27|28|29|30|31|33|34|35|36|37|38|39|41|42|43|44|45|46|47|49|50|51|52|53|54|55|57|58|59|60|61|62|63|65|66|67|68|69|70|71|73|74|75|76|77|78|79)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulFixedKeyword_Reserved } else { LexemeKind::YulFixedKeyword_Unreserved } }, priority = 3000042)]
    #[regex(r#"for"#, |_| { LexemeKind::YulForKeyword_Reserved }, priority = 3000043)]
    #[regex(r#"function"#, |_| { LexemeKind::YulFunctionKeyword_Reserved }, priority = 3000044)]
    #[regex(r#"gwei"#, |lexer| { if LanguageVersion::V0_7_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulGweiKeyword_Reserved } else { LexemeKind::YulGweiKeyword_Unreserved } }, priority = 3000045)]
    #[regex(r#"hex"#, |_| { LexemeKind::YulHexKeyword_Reserved }, priority = 3000046)]
    #[regex(r#"hours"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulHoursKeyword_Reserved } else { LexemeKind::YulHoursKeyword_Unreserved } }, priority = 3000047)]
    #[regex(r#"if"#, |_| { LexemeKind::YulIfKeyword_Reserved }, priority = 3000048)]
    #[regex(r#"immutable"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulImmutableKeyword_Reserved } else { LexemeKind::YulImmutableKeyword_Unreserved } }, priority = 3000049)]
    #[regex(r#"implements"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulImplementsKeyword_Reserved } else { LexemeKind::YulImplementsKeyword_Unreserved } }, priority = 3000050)]
    #[regex(r#"import"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulImportKeyword_Reserved } else { LexemeKind::YulImportKeyword_Unreserved } }, priority = 3000051)]
    #[regex(r#"indexed"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulIndexedKeyword_Reserved } else { LexemeKind::YulIndexedKeyword_Unreserved } }, priority = 3000052)]
    #[regex(r#"in"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_6_8 { LexemeKind::YulInKeyword_Reserved } else { LexemeKind::YulInKeyword_Unreserved } }, priority = 3000053)]
    #[regex(r#"inline"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulInlineKeyword_Reserved } else { LexemeKind::YulInlineKeyword_Unreserved } }, priority = 3000054)]
    #[regex(r#"interface"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulInterfaceKeyword_Reserved } else { LexemeKind::YulInterfaceKeyword_Unreserved } }, priority = 3000055)]
    #[regex(r#"internal"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulInternalKeyword_Reserved } else { LexemeKind::YulInternalKeyword_Unreserved } }, priority = 3000056)]
    #[regex(r#"int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulIntKeyword_Reserved } else { LexemeKind::YulIntKeyword_Unreserved } }, priority = 3000057)]
    #[regex(r#"is"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulIsKeyword_Reserved } else { LexemeKind::YulIsKeyword_Unreserved } }, priority = 3000058)]
    #[regex(r#"leave"#, |lexer| { if LanguageVersion::V0_7_1 <= lexer.extras.language_version { LexemeKind::YulLeaveKeyword_Reserved } else { LexemeKind::YulLeaveKeyword_Unreserved } }, priority = 3000059)]
    #[regex(r#"let"#, |_| { LexemeKind::YulLetKeyword_Reserved }, priority = 3000060)]
    #[regex(r#"library"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulLibraryKeyword_Reserved } else { LexemeKind::YulLibraryKeyword_Unreserved } }, priority = 3000061)]
    #[regex(r#"macro"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMacroKeyword_Reserved } else { LexemeKind::YulMacroKeyword_Unreserved } }, priority = 3000062)]
    #[regex(r#"mapping"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMappingKeyword_Reserved } else { LexemeKind::YulMappingKeyword_Unreserved } }, priority = 3000063)]
    #[regex(r#"match"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMatchKeyword_Reserved } else { LexemeKind::YulMatchKeyword_Unreserved } }, priority = 3000064)]
    #[regex(r#"memory"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMemoryKeyword_Reserved } else { LexemeKind::YulMemoryKeyword_Unreserved } }, priority = 3000065)]
    #[regex(r#"minutes"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMinutesKeyword_Reserved } else { LexemeKind::YulMinutesKeyword_Unreserved } }, priority = 3000066)]
    #[regex(r#"modifier"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulModifierKeyword_Reserved } else { LexemeKind::YulModifierKeyword_Unreserved } }, priority = 3000067)]
    #[regex(r#"mutable"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulMutableKeyword_Reserved } else { LexemeKind::YulMutableKeyword_Unreserved } }, priority = 3000068)]
    #[regex(r#"new"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulNewKeyword_Reserved } else { LexemeKind::YulNewKeyword_Unreserved } }, priority = 3000069)]
    #[regex(r#"null"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulNullKeyword_Reserved } else { LexemeKind::YulNullKeyword_Unreserved } }, priority = 3000070)]
    #[regex(r#"of"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulOfKeyword_Reserved } else { LexemeKind::YulOfKeyword_Unreserved } }, priority = 3000071)]
    #[regex(r#"override"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulOverrideKeyword_Reserved } else { LexemeKind::YulOverrideKeyword_Unreserved } }, priority = 3000072)]
    #[regex(r#"partial"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPartialKeyword_Reserved } else { LexemeKind::YulPartialKeyword_Unreserved } }, priority = 3000073)]
    #[regex(r#"payable"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPayableKeyword_Reserved } else { LexemeKind::YulPayableKeyword_Unreserved } }, priority = 3000074)]
    #[regex(r#"pragma"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPragmaKeyword_Reserved } else { LexemeKind::YulPragmaKeyword_Unreserved } }, priority = 3000075)]
    #[regex(r#"private"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPrivateKeyword_Reserved } else { LexemeKind::YulPrivateKeyword_Unreserved } }, priority = 3000076)]
    #[regex(r#"promise"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPromiseKeyword_Reserved } else { LexemeKind::YulPromiseKeyword_Unreserved } }, priority = 3000077)]
    #[regex(r#"public"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPublicKeyword_Reserved } else { LexemeKind::YulPublicKeyword_Unreserved } }, priority = 3000078)]
    #[regex(r#"pure"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulPureKeyword_Reserved } else { LexemeKind::YulPureKeyword_Unreserved } }, priority = 3000079)]
    #[regex(r#"receive"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulReceiveKeyword_Reserved } else { LexemeKind::YulReceiveKeyword_Unreserved } }, priority = 3000080)]
    #[regex(r#"reference"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulReferenceKeyword_Reserved } else { LexemeKind::YulReferenceKeyword_Unreserved } }, priority = 3000081)]
    #[regex(r#"relocatable"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulRelocatableKeyword_Reserved } else { LexemeKind::YulRelocatableKeyword_Unreserved } }, priority = 3000082)]
    #[regex(r#"returns"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulReturnsKeyword_Reserved } else { LexemeKind::YulReturnsKeyword_Unreserved } }, priority = 3000083)]
    #[regex(r#"sealed"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulSealedKeyword_Reserved } else { LexemeKind::YulSealedKeyword_Unreserved } }, priority = 3000084)]
    #[regex(r#"seconds"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulSecondsKeyword_Reserved } else { LexemeKind::YulSecondsKeyword_Unreserved } }, priority = 3000085)]
    #[regex(r#"sizeof"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulSizeOfKeyword_Reserved } else { LexemeKind::YulSizeOfKeyword_Unreserved } }, priority = 3000086)]
    #[regex(r#"static"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulStaticKeyword_Reserved } else { LexemeKind::YulStaticKeyword_Unreserved } }, priority = 3000087)]
    #[regex(r#"storage"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulStorageKeyword_Reserved } else { LexemeKind::YulStorageKeyword_Unreserved } }, priority = 3000088)]
    #[regex(r#"string"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulStringKeyword_Reserved } else { LexemeKind::YulStringKeyword_Unreserved } }, priority = 3000089)]
    #[regex(r#"struct"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulStructKeyword_Reserved } else { LexemeKind::YulStructKeyword_Unreserved } }, priority = 3000090)]
    #[regex(r#"super"#, |lexer| { if LanguageVersion::V0_8_0 <= lexer.extras.language_version { LexemeKind::YulSuperKeyword_Reserved } else { LexemeKind::YulSuperKeyword_Unreserved } }, priority = 3000091)]
    #[regex(r#"supports"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulSupportsKeyword_Reserved } else { LexemeKind::YulSupportsKeyword_Unreserved } }, priority = 3000092)]
    #[regex(r#"switch"#, |_| { LexemeKind::YulSwitchKeyword_Reserved }, priority = 3000093)]
    #[regex(r#"szabo"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_0 { LexemeKind::YulSzaboKeyword_Reserved } else { LexemeKind::YulSzaboKeyword_Unreserved } }, priority = 3000094)]
    #[regex(r#"this"#, |lexer| { if LanguageVersion::V0_8_0 <= lexer.extras.language_version { LexemeKind::YulThisKeyword_Reserved } else { LexemeKind::YulThisKeyword_Unreserved } }, priority = 3000095)]
    #[regex(r#"throw"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulThrowKeyword_Reserved } else { LexemeKind::YulThrowKeyword_Unreserved } }, priority = 3000096)]
    #[regex(r#"true"#, |_| { LexemeKind::YulTrueKeyword_Reserved }, priority = 3000097)]
    #[regex(r#"try"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulTryKeyword_Reserved } else { LexemeKind::YulTryKeyword_Unreserved } }, priority = 3000098)]
    #[regex(r#"typedef"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulTypeDefKeyword_Reserved } else { LexemeKind::YulTypeDefKeyword_Unreserved } }, priority = 3000099)]
    #[regex(r#"type"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulTypeKeyword_Reserved } else { LexemeKind::YulTypeKeyword_Unreserved } }, priority = 3000100)]
    #[regex(r#"typeof"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulTypeOfKeyword_Reserved } else { LexemeKind::YulTypeOfKeyword_Unreserved } }, priority = 3000101)]
    #[regex(r#"ufixed"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUfixedKeyword_Reserved } else { LexemeKind::YulUfixedKeyword_Unreserved } }, priority = 3000102)]
    #[regex(r#"ufixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176)x(8|16|24|32|40|48|56|64|72|80)"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUfixedKeyword_Reserved } else { LexemeKind::YulUfixedKeyword_Unreserved } }, priority = 3000103)]
    #[regex(r#"ufixed(184x8|184x16|184x24|184x32|184x40|184x48|184x56|184x64|184x72|192x8|192x16|192x24|192x32|192x40|192x48|192x56|192x64|200x8|200x16|200x24|200x32|200x40|200x48|200x56|208x8|208x16|208x24|208x32|208x40|208x48|216x8|216x16|216x24|216x32|216x40|224x8|224x16|224x24|224x32|232x8|232x16|232x24|240x8|240x16|248x8)"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUfixedKeyword_Reserved } else { LexemeKind::YulUfixedKeyword_Unreserved } }, priority = 3000104)]
    #[regex(r#"ufixed(184x80|192x72|192x80|200x64|200x72|200x80|208x56|208x64|208x72|208x80|216x48|216x56|216x64|216x72|216x80|224x40|224x48|224x56|224x64|224x72|224x80|232x32|232x40|232x48|232x56|232x64|232x72|232x80|240x24|240x32|240x40|240x48|240x56|240x64|240x72|240x80|248x16|248x24|248x32|248x40|248x48|248x56|248x64|248x72|248x80|256x8|256x16|256x24|256x32|256x40|256x48|256x56|256x64|256x72|256x80)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUfixedKeyword_Reserved } else { LexemeKind::YulUfixedKeyword_Unreserved } }, priority = 3000105)]
    #[regex(r#"ufixed(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)x(0|1|2|3|4|5|6|7|9|10|11|12|13|14|15|17|18|19|20|21|22|23|25|26|27|28|29|30|31|33|34|35|36|37|38|39|41|42|43|44|45|46|47|49|50|51|52|53|54|55|57|58|59|60|61|62|63|65|66|67|68|69|70|71|73|74|75|76|77|78|79)"#, |lexer| { if LanguageVersion::V0_4_14 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUfixedKeyword_Reserved } else { LexemeKind::YulUfixedKeyword_Unreserved } }, priority = 3000106)]
    #[regex(r#"uint(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUintKeyword_Reserved } else { LexemeKind::YulUintKeyword_Unreserved } }, priority = 3000107)]
    #[regex(r#"unchecked"#, |lexer| { if LanguageVersion::V0_5_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUncheckedKeyword_Reserved } else { LexemeKind::YulUncheckedKeyword_Unreserved } }, priority = 3000108)]
    #[regex(r#"using"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulUsingKeyword_Reserved } else { LexemeKind::YulUsingKeyword_Unreserved } }, priority = 3000109)]
    #[regex(r#"var"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_6_5 { LexemeKind::YulVarKeyword_Reserved } else { LexemeKind::YulVarKeyword_Unreserved } }, priority = 3000110)]
    #[regex(r#"view"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulViewKeyword_Reserved } else { LexemeKind::YulViewKeyword_Unreserved } }, priority = 3000111)]
    #[regex(r#"virtual"#, |lexer| { if LanguageVersion::V0_6_0 <= lexer.extras.language_version && lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulVirtualKeyword_Reserved } else { LexemeKind::YulVirtualKeyword_Unreserved } }, priority = 3000112)]
    #[regex(r#"weeks"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulWeeksKeyword_Reserved } else { LexemeKind::YulWeeksKeyword_Unreserved } }, priority = 3000113)]
    #[regex(r#"wei"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulWeiKeyword_Reserved } else { LexemeKind::YulWeiKeyword_Unreserved } }, priority = 3000114)]
    #[regex(r#"while"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulWhileKeyword_Reserved } else { LexemeKind::YulWhileKeyword_Unreserved } }, priority = 3000115)]
    #[regex(r#"years"#, |lexer| { if lexer.extras.language_version < LanguageVersion::V0_7_1 { LexemeKind::YulYearsKeyword_Reserved } else { LexemeKind::YulYearsKeyword_Unreserved } }, priority = 3000116)]
    #[regex(r#"( |\t)+"#, |_| { LexemeKind::Whitespace }, priority = 1000117, allow_greedy = true)]
    #[regex(r#"\n|(\r\n?)"#, |_| { LexemeKind::EndOfLine }, priority = 1000118, allow_greedy = true)]
    #[regex(r#"//[^\r\n]*"#, |_| { LexemeKind::SingleLineComment }, priority = 1000119, allow_greedy = true)]
    #[regex(r#"/\*[^\*]*\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineComment }, priority = 1000120, allow_greedy = true)]
    #[regex(r#"///[^\r\n]*"#, |_| { LexemeKind::SingleLineNatSpecComment }, priority = 1000121, allow_greedy = true)]
    #[regex(r#"/\*\*([^/\*][^\*]*)?\*+([^/\*][^\*]*\*+)*/"#, |_| { LexemeKind::MultiLineNatSpecComment }, priority = 1000122, allow_greedy = true)]
    Lexeme(LexemeKind),
}
