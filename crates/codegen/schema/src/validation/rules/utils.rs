pub fn is_a_keyword_scanner(reference_name: &str) -> bool {
    // TODO: when Keywords are implemented, we can allow referencing them from scanners.
    // In the meantime, let's just skip them from this validation.
    // https://github.com/NomicFoundation/slang/issues/505

    return match reference_name {
        "BytesKeyword"
        | "SignedFixedType"
        | "UnsignedFixedType"
        | "SignedIntegerType"
        | "UnsignedIntegerType" => true,
        keyword if keyword.contains("Keyword") || keyword.contains("ReservedWord") => true,
        _ => false,
    };
}
