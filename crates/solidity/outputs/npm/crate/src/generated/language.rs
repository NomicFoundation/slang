// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(deprecated, unused_imports)]
use semver::Version;

#[allow(unused_imports)]
use super::{kinds::*, parse_output::*, parser_function::*, scanner_function::*};

#[napi]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_equal_to_or_greater_than_0_4_21: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_4_22: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_5_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_5_3: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_11: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_2: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_5: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_7_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_18: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_19: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_8: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Shared with Rust
    #[error("Unsupported Solidity language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
    #[error("Production '{0:?}' is not valid in this version of Solidity.")]
    InvalidProductionVersion(ProductionKind),

    // TypeScript-specific
    #[error("Invalid semantic version '{0}'.")]
    InvalidSemanticVersion(String),
}

impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}

const VERSIONS: &'static [&'static str] = &[
    "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18", "0.4.19",
    "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26", "0.5.0", "0.5.1",
    "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8", "0.5.9", "0.5.10", "0.5.11",
    "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16", "0.5.17", "0.6.0", "0.6.1", "0.6.2", "0.6.3",
    "0.6.4", "0.6.5", "0.6.6", "0.6.7", "0.6.8", "0.6.9", "0.6.10", "0.6.11", "0.6.12", "0.7.0",
    "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1", "0.8.2", "0.8.3",
    "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11", "0.8.12", "0.8.13",
    "0.8.14", "0.8.15", "0.8.16", "0.8.17", "0.8.18", "0.8.19",
];

#[napi]
impl Language {
    #[napi(constructor)]
    pub fn new(version: String) -> Result<Self, napi::Error> {
        let version =
            Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
        if VERSIONS.contains(&version.to_string().as_str()) {
            Ok(Self {
                version_is_equal_to_or_greater_than_0_4_21: Version::parse("0.4.21").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_4_22: Version::parse("0.4.22").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_5_0: Version::parse("0.5.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_5_3: Version::parse("0.5.3").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_0: Version::parse("0.6.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_11: Version::parse("0.6.11").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_2: Version::parse("0.6.2").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_5: Version::parse("0.6.5").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_7_0: Version::parse("0.7.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_0: Version::parse("0.8.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_18: Version::parse("0.8.18").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_19: Version::parse("0.8.19").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_8: Version::parse("0.8.8").unwrap()
                    <= version,
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version).into())
        }
    }

    #[napi(getter)]
    pub fn version(&self) -> String {
        self.version.to_string()
    }

    #[napi]
    pub fn supported_versions() -> Vec<String> {
        return VERSIONS.iter().map(|v| v.to_string()).collect();
    }

    #[napi]
    pub fn parse(
        &self,
        production_kind: ProductionKind,
        input: String,
    ) -> Result<ParseOutput, napi::Error> {
        let input = input.as_str();
        match production_kind {
            ProductionKind::AbicoderKeyword => {
                Language::abicoder_keyword.scan(self, input, TokenKind::AbicoderKeyword)
            }
            ProductionKind::AbstractKeyword => Language::abstract_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::AbstractKeyword,
            ),
            ProductionKind::AddressKeyword => {
                Language::address_keyword.scan(self, input, TokenKind::AddressKeyword)
            }
            ProductionKind::Ampersand => {
                Language::ampersand.scan(self, input, TokenKind::Ampersand)
            }
            ProductionKind::AmpersandAmpersand => {
                Language::ampersand_ampersand.scan(self, input, TokenKind::AmpersandAmpersand)
            }
            ProductionKind::AmpersandEqual => {
                Language::ampersand_equal.scan(self, input, TokenKind::AmpersandEqual)
            }
            ProductionKind::AnonymousKeyword => {
                Language::anonymous_keyword.scan(self, input, TokenKind::AnonymousKeyword)
            }
            ProductionKind::AsKeyword => {
                Language::as_keyword.scan(self, input, TokenKind::AsKeyword)
            }
            ProductionKind::AsciiStringLiteral => {
                Language::ascii_string_literal.scan(self, input, TokenKind::AsciiStringLiteral)
            }
            ProductionKind::AssemblyKeyword => {
                Language::assembly_keyword.scan(self, input, TokenKind::AssemblyKeyword)
            }
            ProductionKind::Asterisk => Language::asterisk.scan(self, input, TokenKind::Asterisk),
            ProductionKind::AsteriskAsterisk => {
                Language::asterisk_asterisk.scan(self, input, TokenKind::AsteriskAsterisk)
            }
            ProductionKind::AsteriskEqual => {
                Language::asterisk_equal.scan(self, input, TokenKind::AsteriskEqual)
            }
            ProductionKind::Bang => Language::bang.scan(self, input, TokenKind::Bang),
            ProductionKind::BangEqual => {
                Language::bang_equal.scan(self, input, TokenKind::BangEqual)
            }
            ProductionKind::Bar => Language::bar.scan(self, input, TokenKind::Bar),
            ProductionKind::BarBar => Language::bar_bar.scan(self, input, TokenKind::BarBar),
            ProductionKind::BarEqual => Language::bar_equal.scan(self, input, TokenKind::BarEqual),
            ProductionKind::BoolKeyword => {
                Language::bool_keyword.scan(self, input, TokenKind::BoolKeyword)
            }
            ProductionKind::BreakKeyword => {
                Language::break_keyword.scan(self, input, TokenKind::BreakKeyword)
            }
            ProductionKind::ByteKeyword => {
                Language::byte_keyword__sparse_dispatch.scan(self, input, TokenKind::ByteKeyword)
            }
            ProductionKind::CalldataKeyword => Language::calldata_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::CalldataKeyword,
            ),
            ProductionKind::Caret => Language::caret.scan(self, input, TokenKind::Caret),
            ProductionKind::CaretEqual => {
                Language::caret_equal.scan(self, input, TokenKind::CaretEqual)
            }
            ProductionKind::CaseKeyword => {
                Language::case_keyword.scan(self, input, TokenKind::CaseKeyword)
            }
            ProductionKind::CatchKeyword => {
                Language::catch_keyword__sparse_dispatch.scan(self, input, TokenKind::CatchKeyword)
            }
            ProductionKind::CloseBrace => {
                Language::close_brace.scan(self, input, TokenKind::CloseBrace)
            }
            ProductionKind::CloseBracket => {
                Language::close_bracket.scan(self, input, TokenKind::CloseBracket)
            }
            ProductionKind::CloseParen => {
                Language::close_paren.scan(self, input, TokenKind::CloseParen)
            }
            ProductionKind::Colon => Language::colon.scan(self, input, TokenKind::Colon),
            ProductionKind::ColonEqual => {
                Language::colon_equal.scan(self, input, TokenKind::ColonEqual)
            }
            ProductionKind::Comma => Language::comma.scan(self, input, TokenKind::Comma),
            ProductionKind::ConstantKeyword => {
                Language::constant_keyword.scan(self, input, TokenKind::ConstantKeyword)
            }
            ProductionKind::ConstructorKeyword => Language::constructor_keyword__sparse_dispatch
                .scan(self, input, TokenKind::ConstructorKeyword),
            ProductionKind::ContinueKeyword => {
                Language::continue_keyword.scan(self, input, TokenKind::ContinueKeyword)
            }
            ProductionKind::ContractKeyword => {
                Language::contract_keyword.scan(self, input, TokenKind::ContractKeyword)
            }
            ProductionKind::DaysKeyword => {
                Language::days_keyword.scan(self, input, TokenKind::DaysKeyword)
            }
            ProductionKind::DecimalLiteral => {
                Language::decimal_literal.scan(self, input, TokenKind::DecimalLiteral)
            }
            ProductionKind::DefaultKeyword => {
                Language::default_keyword.scan(self, input, TokenKind::DefaultKeyword)
            }
            ProductionKind::DeleteKeyword => {
                Language::delete_keyword.scan(self, input, TokenKind::DeleteKeyword)
            }
            ProductionKind::DoKeyword => {
                Language::do_keyword.scan(self, input, TokenKind::DoKeyword)
            }
            ProductionKind::ElseKeyword => {
                Language::else_keyword.scan(self, input, TokenKind::ElseKeyword)
            }
            ProductionKind::EmitKeyword => {
                Language::emit_keyword__sparse_dispatch.scan(self, input, TokenKind::EmitKeyword)
            }
            ProductionKind::EndOfLine => {
                Language::end_of_line.scan(self, input, TokenKind::EndOfLine)
            }
            ProductionKind::EnumKeyword => {
                Language::enum_keyword.scan(self, input, TokenKind::EnumKeyword)
            }
            ProductionKind::Equal => Language::equal.scan(self, input, TokenKind::Equal),
            ProductionKind::EqualEqual => {
                Language::equal_equal.scan(self, input, TokenKind::EqualEqual)
            }
            ProductionKind::EqualGreaterThan => {
                Language::equal_greater_than.scan(self, input, TokenKind::EqualGreaterThan)
            }
            ProductionKind::ErrorKeyword => {
                Language::error_keyword.scan(self, input, TokenKind::ErrorKeyword)
            }
            ProductionKind::EtherKeyword => {
                Language::ether_keyword.scan(self, input, TokenKind::EtherKeyword)
            }
            ProductionKind::EventKeyword => {
                Language::event_keyword.scan(self, input, TokenKind::EventKeyword)
            }
            ProductionKind::ExperimentalKeyword => {
                Language::experimental_keyword.scan(self, input, TokenKind::ExperimentalKeyword)
            }
            ProductionKind::ExternalKeyword => {
                Language::external_keyword.scan(self, input, TokenKind::ExternalKeyword)
            }
            ProductionKind::FallbackKeyword => {
                Language::fallback_keyword.scan(self, input, TokenKind::FallbackKeyword)
            }
            ProductionKind::FalseKeyword => {
                Language::false_keyword.scan(self, input, TokenKind::FalseKeyword)
            }
            ProductionKind::FinneyKeyword => Language::finney_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::FinneyKeyword,
            ),
            ProductionKind::FixedBytesType => {
                Language::fixed_bytes_type.scan(self, input, TokenKind::FixedBytesType)
            }
            ProductionKind::ForKeyword => {
                Language::for_keyword.scan(self, input, TokenKind::ForKeyword)
            }
            ProductionKind::FromKeyword => {
                Language::from_keyword.scan(self, input, TokenKind::FromKeyword)
            }
            ProductionKind::FunctionKeyword => {
                Language::function_keyword.scan(self, input, TokenKind::FunctionKeyword)
            }
            ProductionKind::GlobalKeyword => {
                Language::global_keyword.scan(self, input, TokenKind::GlobalKeyword)
            }
            ProductionKind::GreaterThan => {
                Language::greater_than.scan(self, input, TokenKind::GreaterThan)
            }
            ProductionKind::GreaterThanEqual => {
                Language::greater_than_equal.scan(self, input, TokenKind::GreaterThanEqual)
            }
            ProductionKind::GreaterThanGreaterThan => Language::greater_than_greater_than.scan(
                self,
                input,
                TokenKind::GreaterThanGreaterThan,
            ),
            ProductionKind::GreaterThanGreaterThanEqual => {
                Language::greater_than_greater_than_equal.scan(
                    self,
                    input,
                    TokenKind::GreaterThanGreaterThanEqual,
                )
            }
            ProductionKind::GreaterThanGreaterThanGreaterThan => {
                Language::greater_than_greater_than_greater_than.scan(
                    self,
                    input,
                    TokenKind::GreaterThanGreaterThanGreaterThan,
                )
            }
            ProductionKind::GreaterThanGreaterThanGreaterThanEqual => {
                Language::greater_than_greater_than_greater_than_equal.scan(
                    self,
                    input,
                    TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                )
            }
            ProductionKind::GweiKeyword => {
                Language::gwei_keyword__sparse_dispatch.scan(self, input, TokenKind::GweiKeyword)
            }
            ProductionKind::HexLiteral => {
                Language::hex_literal.scan(self, input, TokenKind::HexLiteral)
            }
            ProductionKind::HexStringLiteral => {
                Language::hex_string_literal.scan(self, input, TokenKind::HexStringLiteral)
            }
            ProductionKind::HoursKeyword => {
                Language::hours_keyword.scan(self, input, TokenKind::HoursKeyword)
            }
            ProductionKind::Identifier => {
                Language::identifier.scan(self, input, TokenKind::Identifier)
            }
            ProductionKind::IfKeyword => {
                Language::if_keyword.scan(self, input, TokenKind::IfKeyword)
            }
            ProductionKind::ImmutableKeyword => Language::immutable_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::ImmutableKeyword,
            ),
            ProductionKind::ImportKeyword => {
                Language::import_keyword.scan(self, input, TokenKind::ImportKeyword)
            }
            ProductionKind::IndexedKeyword => {
                Language::indexed_keyword.scan(self, input, TokenKind::IndexedKeyword)
            }
            ProductionKind::InterfaceKeyword => {
                Language::interface_keyword.scan(self, input, TokenKind::InterfaceKeyword)
            }
            ProductionKind::InternalKeyword => {
                Language::internal_keyword.scan(self, input, TokenKind::InternalKeyword)
            }
            ProductionKind::IsKeyword => {
                Language::is_keyword.scan(self, input, TokenKind::IsKeyword)
            }
            ProductionKind::LeaveKeyword => {
                Language::leave_keyword__sparse_dispatch.scan(self, input, TokenKind::LeaveKeyword)
            }
            ProductionKind::LessThan => Language::less_than.scan(self, input, TokenKind::LessThan),
            ProductionKind::LessThanEqual => {
                Language::less_than_equal.scan(self, input, TokenKind::LessThanEqual)
            }
            ProductionKind::LessThanLessThan => {
                Language::less_than_less_than.scan(self, input, TokenKind::LessThanLessThan)
            }
            ProductionKind::LessThanLessThanEqual => Language::less_than_less_than_equal.scan(
                self,
                input,
                TokenKind::LessThanLessThanEqual,
            ),
            ProductionKind::LetKeyword => {
                Language::let_keyword.scan(self, input, TokenKind::LetKeyword)
            }
            ProductionKind::LibraryKeyword => {
                Language::library_keyword.scan(self, input, TokenKind::LibraryKeyword)
            }
            ProductionKind::MappingKeyword => {
                Language::mapping_keyword.scan(self, input, TokenKind::MappingKeyword)
            }
            ProductionKind::MemoryKeyword => {
                Language::memory_keyword.scan(self, input, TokenKind::MemoryKeyword)
            }
            ProductionKind::Minus => Language::minus.scan(self, input, TokenKind::Minus),
            ProductionKind::MinusEqual => {
                Language::minus_equal.scan(self, input, TokenKind::MinusEqual)
            }
            ProductionKind::MinusGreaterThan => {
                Language::minus_greater_than.scan(self, input, TokenKind::MinusGreaterThan)
            }
            ProductionKind::MinusMinus => {
                Language::minus_minus.scan(self, input, TokenKind::MinusMinus)
            }
            ProductionKind::MinutesKeyword => {
                Language::minutes_keyword.scan(self, input, TokenKind::MinutesKeyword)
            }
            ProductionKind::ModifierKeyword => {
                Language::modifier_keyword.scan(self, input, TokenKind::ModifierKeyword)
            }
            ProductionKind::MultilineComment => {
                Language::multiline_comment.scan(self, input, TokenKind::MultilineComment)
            }
            ProductionKind::NewKeyword => {
                Language::new_keyword.scan(self, input, TokenKind::NewKeyword)
            }
            ProductionKind::OpenBrace => {
                Language::open_brace.scan(self, input, TokenKind::OpenBrace)
            }
            ProductionKind::OpenBracket => {
                Language::open_bracket.scan(self, input, TokenKind::OpenBracket)
            }
            ProductionKind::OpenParen => {
                Language::open_paren.scan(self, input, TokenKind::OpenParen)
            }
            ProductionKind::OverrideKeyword => {
                Language::override_keyword.scan(self, input, TokenKind::OverrideKeyword)
            }
            ProductionKind::PayableKeyword => {
                Language::payable_keyword.scan(self, input, TokenKind::PayableKeyword)
            }
            ProductionKind::Percent => Language::percent.scan(self, input, TokenKind::Percent),
            ProductionKind::PercentEqual => {
                Language::percent_equal.scan(self, input, TokenKind::PercentEqual)
            }
            ProductionKind::Period => Language::period.scan(self, input, TokenKind::Period),
            ProductionKind::Plus => Language::plus.scan(self, input, TokenKind::Plus),
            ProductionKind::PlusEqual => {
                Language::plus_equal.scan(self, input, TokenKind::PlusEqual)
            }
            ProductionKind::PlusPlus => Language::plus_plus.scan(self, input, TokenKind::PlusPlus),
            ProductionKind::PragmaKeyword => {
                Language::pragma_keyword.scan(self, input, TokenKind::PragmaKeyword)
            }
            ProductionKind::PrivateKeyword => {
                Language::private_keyword.scan(self, input, TokenKind::PrivateKeyword)
            }
            ProductionKind::PublicKeyword => {
                Language::public_keyword.scan(self, input, TokenKind::PublicKeyword)
            }
            ProductionKind::PureKeyword => {
                Language::pure_keyword.scan(self, input, TokenKind::PureKeyword)
            }
            ProductionKind::QuestionMark => {
                Language::question_mark.scan(self, input, TokenKind::QuestionMark)
            }
            ProductionKind::ReceiveKeyword => {
                Language::receive_keyword.scan(self, input, TokenKind::ReceiveKeyword)
            }
            ProductionKind::ReturnKeyword => {
                Language::return_keyword.scan(self, input, TokenKind::ReturnKeyword)
            }
            ProductionKind::ReturnsKeyword => {
                Language::returns_keyword.scan(self, input, TokenKind::ReturnsKeyword)
            }
            ProductionKind::RevertKeyword => {
                Language::revert_keyword.scan(self, input, TokenKind::RevertKeyword)
            }
            ProductionKind::SecondsKeyword => {
                Language::seconds_keyword.scan(self, input, TokenKind::SecondsKeyword)
            }
            ProductionKind::Semicolon => {
                Language::semicolon.scan(self, input, TokenKind::Semicolon)
            }
            ProductionKind::SignedFixedType => {
                Language::signed_fixed_type.scan(self, input, TokenKind::SignedFixedType)
            }
            ProductionKind::SignedIntegerType => {
                Language::signed_integer_type.scan(self, input, TokenKind::SignedIntegerType)
            }
            ProductionKind::SingleLineComment => {
                Language::single_line_comment.scan(self, input, TokenKind::SingleLineComment)
            }
            ProductionKind::Slash => Language::slash.scan(self, input, TokenKind::Slash),
            ProductionKind::SlashEqual => {
                Language::slash_equal.scan(self, input, TokenKind::SlashEqual)
            }
            ProductionKind::SolidityKeyword => {
                Language::solidity_keyword.scan(self, input, TokenKind::SolidityKeyword)
            }
            ProductionKind::StorageKeyword => {
                Language::storage_keyword.scan(self, input, TokenKind::StorageKeyword)
            }
            ProductionKind::StringKeyword => {
                Language::string_keyword.scan(self, input, TokenKind::StringKeyword)
            }
            ProductionKind::StructKeyword => {
                Language::struct_keyword.scan(self, input, TokenKind::StructKeyword)
            }
            ProductionKind::SwitchKeyword => {
                Language::switch_keyword.scan(self, input, TokenKind::SwitchKeyword)
            }
            ProductionKind::SzaboKeyword => {
                Language::szabo_keyword__sparse_dispatch.scan(self, input, TokenKind::SzaboKeyword)
            }
            ProductionKind::ThrowKeyword => {
                Language::throw_keyword__sparse_dispatch.scan(self, input, TokenKind::ThrowKeyword)
            }
            ProductionKind::Tilde => Language::tilde.scan(self, input, TokenKind::Tilde),
            ProductionKind::TrueKeyword => {
                Language::true_keyword.scan(self, input, TokenKind::TrueKeyword)
            }
            ProductionKind::TryKeyword => {
                Language::try_keyword__sparse_dispatch.scan(self, input, TokenKind::TryKeyword)
            }
            ProductionKind::TypeKeyword => {
                Language::type_keyword__sparse_dispatch.scan(self, input, TokenKind::TypeKeyword)
            }
            ProductionKind::UncheckedKeyword => Language::unchecked_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::UncheckedKeyword,
            ),
            ProductionKind::UnicodeStringLiteral => {
                Language::unicode_string_literal__sparse_dispatch.scan(
                    self,
                    input,
                    TokenKind::UnicodeStringLiteral,
                )
            }
            ProductionKind::UnsignedFixedType => {
                Language::unsigned_fixed_type.scan(self, input, TokenKind::UnsignedFixedType)
            }
            ProductionKind::UnsignedIntegerType => {
                Language::unsigned_integer_type.scan(self, input, TokenKind::UnsignedIntegerType)
            }
            ProductionKind::UsingKeyword => {
                Language::using_keyword.scan(self, input, TokenKind::UsingKeyword)
            }
            ProductionKind::VarKeyword => {
                Language::var_keyword__sparse_dispatch.scan(self, input, TokenKind::VarKeyword)
            }
            ProductionKind::VersionPragmaValue => {
                Language::version_pragma_value.scan(self, input, TokenKind::VersionPragmaValue)
            }
            ProductionKind::ViewKeyword => {
                Language::view_keyword.scan(self, input, TokenKind::ViewKeyword)
            }
            ProductionKind::VirtualKeyword => Language::virtual_keyword__sparse_dispatch.scan(
                self,
                input,
                TokenKind::VirtualKeyword,
            ),
            ProductionKind::WeeksKeyword => {
                Language::weeks_keyword.scan(self, input, TokenKind::WeeksKeyword)
            }
            ProductionKind::WeiKeyword => {
                Language::wei_keyword.scan(self, input, TokenKind::WeiKeyword)
            }
            ProductionKind::WhileKeyword => {
                Language::while_keyword.scan(self, input, TokenKind::WhileKeyword)
            }
            ProductionKind::Whitespace => {
                Language::whitespace.scan(self, input, TokenKind::Whitespace)
            }
            ProductionKind::YearsKeyword => {
                Language::years_keyword__sparse_dispatch.scan(self, input, TokenKind::YearsKeyword)
            }
            ProductionKind::YulDecimalLiteral => {
                Language::yul_decimal_literal.scan(self, input, TokenKind::YulDecimalLiteral)
            }
            ProductionKind::YulHexLiteral => {
                Language::yul_hex_literal.scan(self, input, TokenKind::YulHexLiteral)
            }
            ProductionKind::YulIdentifier => {
                Language::yul_identifier.scan(self, input, TokenKind::YulIdentifier)
            }
            ProductionKind::ABICoderPragma => Language::abi_coder_pragma.parse(self, input),
            ProductionKind::AddressType => Language::address_type.parse(self, input),
            ProductionKind::ArgumentsDeclaration => {
                Language::arguments_declaration.parse(self, input)
            }
            ProductionKind::ArrayExpression => Language::array_expression.parse(self, input),
            ProductionKind::ArrayValuesList => Language::array_values_list.parse(self, input),
            ProductionKind::AsciiStringLiteralsList => {
                Language::ascii_string_literals_list.parse(self, input)
            }
            ProductionKind::AssemblyFlagsList => Language::assembly_flags_list.parse(self, input),
            ProductionKind::AssemblyStatement => Language::assembly_statement.parse(self, input),
            ProductionKind::Block => Language::block.parse(self, input),
            ProductionKind::BreakStatement => Language::break_statement.parse(self, input),
            ProductionKind::CatchClause => {
                Language::catch_clause__sparse_dispatch.parse(self, input)
            }
            ProductionKind::CatchClauseError => {
                Language::catch_clause_error__sparse_dispatch.parse(self, input)
            }
            ProductionKind::CatchClausesList => {
                Language::catch_clauses_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ConstantDefinition => Language::constant_definition.parse(self, input),
            ProductionKind::ConstructorAttributesList => {
                Language::constructor_attributes_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ConstructorDefinition => {
                Language::constructor_definition__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ContinueStatement => Language::continue_statement.parse(self, input),
            ProductionKind::ContractDefinition => Language::contract_definition.parse(self, input),
            ProductionKind::ContractMembersList => {
                Language::contract_members_list.parse(self, input)
            }
            ProductionKind::DeconstructionImport => {
                Language::deconstruction_import.parse(self, input)
            }
            ProductionKind::DeconstructionImportSymbol => {
                Language::deconstruction_import_symbol.parse(self, input)
            }
            ProductionKind::DeconstructionImportSymbolsList => {
                Language::deconstruction_import_symbols_list.parse(self, input)
            }
            ProductionKind::DeleteStatement => Language::delete_statement.parse(self, input),
            ProductionKind::DoWhileStatement => Language::do_while_statement.parse(self, input),
            ProductionKind::EmitStatement => {
                Language::emit_statement__sparse_dispatch.parse(self, input)
            }
            ProductionKind::EndOfFileTrivia => Language::end_of_file_trivia.parse(self, input),
            ProductionKind::EnumDefinition => Language::enum_definition.parse(self, input),
            ProductionKind::ErrorDefinition => Language::error_definition.parse(self, input),
            ProductionKind::ErrorParameter => Language::error_parameter.parse(self, input),
            ProductionKind::ErrorParametersList => {
                Language::error_parameters_list.parse(self, input)
            }
            ProductionKind::EventDefinition => Language::event_definition.parse(self, input),
            ProductionKind::EventParameter => Language::event_parameter.parse(self, input),
            ProductionKind::EventParametersList => {
                Language::event_parameters_list.parse(self, input)
            }
            ProductionKind::ExperimentalPragma => Language::experimental_pragma.parse(self, input),
            ProductionKind::Expression => Language::expression.parse(self, input),
            ProductionKind::ExpressionStatement => {
                Language::expression_statement.parse(self, input)
            }
            ProductionKind::FallbackFunctionAttributesList => {
                Language::fallback_function_attributes_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::FallbackFunctionDefinition => {
                Language::fallback_function_definition__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ForStatement => Language::for_statement.parse(self, input),
            ProductionKind::FunctionAttributesList => {
                Language::function_attributes_list.parse(self, input)
            }
            ProductionKind::FunctionCallOptions => {
                Language::function_call_options__sparse_dispatch.parse(self, input)
            }
            ProductionKind::FunctionDefinition => Language::function_definition.parse(self, input),
            ProductionKind::FunctionType => Language::function_type.parse(self, input),
            ProductionKind::FunctionTypeAttributesList => {
                Language::function_type_attributes_list.parse(self, input)
            }
            ProductionKind::HexStringLiteralsList => {
                Language::hex_string_literals_list.parse(self, input)
            }
            ProductionKind::IdentifierPath => Language::identifier_path.parse(self, input),
            ProductionKind::IdentifierPathsList => {
                Language::identifier_paths_list.parse(self, input)
            }
            ProductionKind::IdentifiersList => Language::identifiers_list.parse(self, input),
            ProductionKind::IfStatement => Language::if_statement.parse(self, input),
            ProductionKind::ImportDirective => Language::import_directive.parse(self, input),
            ProductionKind::InheritanceSpecifier => {
                Language::inheritance_specifier.parse(self, input)
            }
            ProductionKind::InheritanceType => Language::inheritance_type.parse(self, input),
            ProductionKind::InheritanceTypesList => {
                Language::inheritance_types_list.parse(self, input)
            }
            ProductionKind::InterfaceDefinition => {
                Language::interface_definition.parse(self, input)
            }
            ProductionKind::InterfaceMembersList => {
                Language::interface_members_list.parse(self, input)
            }
            ProductionKind::LeadingTrivia => Language::leading_trivia.parse(self, input),
            ProductionKind::LibraryDefinition => Language::library_definition.parse(self, input),
            ProductionKind::LibraryMembersList => Language::library_members_list.parse(self, input),
            ProductionKind::MappingKeyType => Language::mapping_key_type.parse(self, input),
            ProductionKind::MappingType => Language::mapping_type.parse(self, input),
            ProductionKind::MappingValueType => Language::mapping_value_type.parse(self, input),
            ProductionKind::ModifierAttributesList => {
                Language::modifier_attributes_list.parse(self, input)
            }
            ProductionKind::ModifierDefinition => Language::modifier_definition.parse(self, input),
            ProductionKind::ModifierInvocation => Language::modifier_invocation.parse(self, input),
            ProductionKind::NamedArgument => Language::named_argument.parse(self, input),
            ProductionKind::NamedArgumentsDeclaration => {
                Language::named_arguments_declaration.parse(self, input)
            }
            ProductionKind::NamedArgumentsList => Language::named_arguments_list.parse(self, input),
            ProductionKind::NamedImport => Language::named_import.parse(self, input),
            ProductionKind::NewExpression => Language::new_expression.parse(self, input),
            ProductionKind::NumericExpression => Language::numeric_expression.parse(self, input),
            ProductionKind::OverrideSpecifier => Language::override_specifier.parse(self, input),
            ProductionKind::Parameter => Language::parameter.parse(self, input),
            ProductionKind::ParametersDeclaration => {
                Language::parameters_declaration.parse(self, input)
            }
            ProductionKind::ParametersList => Language::parameters_list.parse(self, input),
            ProductionKind::PathImport => Language::path_import.parse(self, input),
            ProductionKind::PositionalArgumentsList => {
                Language::positional_arguments_list.parse(self, input)
            }
            ProductionKind::PragmaDirective => Language::pragma_directive.parse(self, input),
            ProductionKind::ReceiveFunctionAttributesList => {
                Language::receive_function_attributes_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ReceiveFunctionDefinition => {
                Language::receive_function_definition__sparse_dispatch.parse(self, input)
            }
            ProductionKind::ReturnStatement => Language::return_statement.parse(self, input),
            ProductionKind::ReturnsDeclaration => Language::returns_declaration.parse(self, input),
            ProductionKind::RevertStatement => Language::revert_statement.parse(self, input),
            ProductionKind::SourceUnit => Language::source_unit.parse(self, input),
            ProductionKind::SourceUnitMembersList => {
                Language::source_unit_members_list.parse(self, input)
            }
            ProductionKind::StateVariableAttributesList => {
                Language::state_variable_attributes_list.parse(self, input)
            }
            ProductionKind::StateVariableDefinition => {
                Language::state_variable_definition.parse(self, input)
            }
            ProductionKind::Statement => Language::statement.parse(self, input),
            ProductionKind::StatementsList => Language::statements_list.parse(self, input),
            ProductionKind::StructDefinition => Language::struct_definition.parse(self, input),
            ProductionKind::StructMember => Language::struct_member.parse(self, input),
            ProductionKind::StructMembersList => Language::struct_members_list.parse(self, input),
            ProductionKind::ThrowStatement => {
                Language::throw_statement__sparse_dispatch.parse(self, input)
            }
            ProductionKind::TrailingTrivia => Language::trailing_trivia.parse(self, input),
            ProductionKind::TryStatement => {
                Language::try_statement__sparse_dispatch.parse(self, input)
            }
            ProductionKind::TupleDeconstructionStatement => {
                Language::tuple_deconstruction_statement.parse(self, input)
            }
            ProductionKind::TupleExpression => Language::tuple_expression.parse(self, input),
            ProductionKind::TupleMember => Language::tuple_member.parse(self, input),
            ProductionKind::TupleMembersList => Language::tuple_members_list.parse(self, input),
            ProductionKind::TupleValuesList => Language::tuple_values_list.parse(self, input),
            ProductionKind::TypeExpression => {
                Language::type_expression__sparse_dispatch.parse(self, input)
            }
            ProductionKind::TypeName => Language::type_name.parse(self, input),
            ProductionKind::UncheckedBlock => {
                Language::unchecked_block__sparse_dispatch.parse(self, input)
            }
            ProductionKind::UnicodeStringLiteralsList => {
                Language::unicode_string_literals_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::UnnamedFunctionAttributesList => {
                Language::unnamed_function_attributes_list__sparse_dispatch.parse(self, input)
            }
            ProductionKind::UnnamedFunctionDefinition => {
                Language::unnamed_function_definition__sparse_dispatch.parse(self, input)
            }
            ProductionKind::UserDefinedValueTypeDefinition => {
                Language::user_defined_value_type_definition__sparse_dispatch.parse(self, input)
            }
            ProductionKind::UsingDirective => Language::using_directive.parse(self, input),
            ProductionKind::UsingDirectiveDeconstruction => {
                Language::using_directive_deconstruction.parse(self, input)
            }
            ProductionKind::UsingDirectivePath => Language::using_directive_path.parse(self, input),
            ProductionKind::UsingDirectiveSymbol => {
                Language::using_directive_symbol.parse(self, input)
            }
            ProductionKind::UsingDirectiveSymbolsList => {
                Language::using_directive_symbols_list.parse(self, input)
            }
            ProductionKind::VariableDeclaration => {
                Language::variable_declaration.parse(self, input)
            }
            ProductionKind::VariableDeclarationStatement => {
                Language::variable_declaration_statement.parse(self, input)
            }
            ProductionKind::VersionPragma => Language::version_pragma.parse(self, input),
            ProductionKind::VersionPragmaExpression => {
                Language::version_pragma_expression.parse(self, input)
            }
            ProductionKind::VersionPragmaExpressionsList => {
                Language::version_pragma_expressions_list.parse(self, input)
            }
            ProductionKind::VersionPragmaSpecifier => {
                Language::version_pragma_specifier.parse(self, input)
            }
            ProductionKind::WhileStatement => Language::while_statement.parse(self, input),
            ProductionKind::YulAssignmentStatement => {
                Language::yul_assignment_statement.parse(self, input)
            }
            ProductionKind::YulBlock => Language::yul_block.parse(self, input),
            ProductionKind::YulBreakStatement => Language::yul_break_statement.parse(self, input),
            ProductionKind::YulContinueStatement => {
                Language::yul_continue_statement.parse(self, input)
            }
            ProductionKind::YulDeclarationStatement => {
                Language::yul_declaration_statement.parse(self, input)
            }
            ProductionKind::YulExpression => Language::yul_expression.parse(self, input),
            ProductionKind::YulExpressionsList => Language::yul_expressions_list.parse(self, input),
            ProductionKind::YulForStatement => Language::yul_for_statement.parse(self, input),
            ProductionKind::YulFunctionDefinition => {
                Language::yul_function_definition.parse(self, input)
            }
            ProductionKind::YulIdentifierPath => Language::yul_identifier_path.parse(self, input),
            ProductionKind::YulIdentifierPathsList => {
                Language::yul_identifier_paths_list.parse(self, input)
            }
            ProductionKind::YulIdentifiersList => Language::yul_identifiers_list.parse(self, input),
            ProductionKind::YulIfStatement => Language::yul_if_statement.parse(self, input),
            ProductionKind::YulLeaveStatement => {
                Language::yul_leave_statement__sparse_dispatch.parse(self, input)
            }
            ProductionKind::YulParametersDeclaration => {
                Language::yul_parameters_declaration.parse(self, input)
            }
            ProductionKind::YulReturnsDeclaration => {
                Language::yul_returns_declaration.parse(self, input)
            }
            ProductionKind::YulStatement => Language::yul_statement.parse(self, input),
            ProductionKind::YulStatementsList => Language::yul_statements_list.parse(self, input),
            ProductionKind::YulSwitchCase => Language::yul_switch_case.parse(self, input),
            ProductionKind::YulSwitchCasesList => {
                Language::yul_switch_cases_list.parse(self, input)
            }
            ProductionKind::YulSwitchStatement => Language::yul_switch_statement.parse(self, input),
        }
        .ok_or_else(|| Error::InvalidProductionVersion(production_kind).into())
    }
}
