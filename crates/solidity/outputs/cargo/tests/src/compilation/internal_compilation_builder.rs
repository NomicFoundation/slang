use anyhow::Result;
use slang_solidity::compilation::InternalCompilationBuilder;
use slang_solidity::utils::LanguageFacts;

const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";

contract Counter is Ownable {}
"#;

const OWNABLE_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

abstract contract Ownable {}
"#;

#[test]
fn test_build_compilation_unit() -> Result<()> {
    let mut internal_builder = InternalCompilationBuilder::create(LanguageFacts::LATEST_VERSION)?;

    let main_id = "MAIN-ID";
    let ownable_id = "OWNABLE-ID";

    let main_add_file_response = internal_builder.add_file(main_id.to_string(), MAIN_SOL_CONTENTS);
    assert_eq!(main_add_file_response.import_paths.len(), 1);
    assert_eq!(
        main_add_file_response.import_paths[0].node().unparse(),
        "\"ownable.sol\""
    );
    internal_builder.resolve_import(
        main_id,
        &main_add_file_response.import_paths[0],
        ownable_id.to_string(),
    )?;

    let ownable_add_file_response =
        internal_builder.add_file(ownable_id.to_string(), OWNABLE_SOL_CONTENTS);
    assert!(ownable_add_file_response.import_paths.is_empty());

    let compilation_unit = internal_builder.build();
    assert_eq!(compilation_unit.files().len(), 2);
    assert!(compilation_unit.file(main_id).is_some());
    assert!(compilation_unit.file(ownable_id).is_some());

    Ok(())
}
