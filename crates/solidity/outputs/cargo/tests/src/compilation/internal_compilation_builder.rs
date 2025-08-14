use anyhow::Result;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity::utils::LanguageFacts;

const MAIN_ID: &str = "MAIN-ID";
const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";

contract Counter is Ownable {}
"#;

const OWNABLE_ID: &str = "ONWABLE-ID";
const OWNABLE_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

abstract contract Ownable {}
"#;

struct Config {}

impl CompilationBuilderConfig for Config {
    type Error = ();

    fn read_file(&mut self, file_id: &str) -> std::result::Result<Option<String>, Self::Error> {
        match file_id {
            MAIN_ID => Ok(Some(MAIN_SOL_CONTENTS.to_owned())),
            OWNABLE_ID => Ok(Some(OWNABLE_SOL_CONTENTS.to_owned())),
            _ => Ok(None),
        }
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> std::result::Result<Option<String>, Self::Error> {
        assert_eq!(source_file_id, MAIN_ID);
        assert_eq!(import_path_cursor.node().unparse(), "\"ownable.sol\"");
        Ok(Some(OWNABLE_ID.to_owned()))
    }
}

#[test]
fn test_build_compilation_unit() -> Result<()> {
    let mut builder = CompilationBuilder::new(LanguageFacts::LATEST_VERSION, Config {})?;

    let main_add_file_response = builder.add_file(MAIN_ID);
    assert!(main_add_file_response.is_ok());
    let compilation_unit = builder.build();
    assert_eq!(compilation_unit.files().len(), 2);
    assert!(compilation_unit.file(MAIN_ID).is_some());
    assert!(compilation_unit.file(OWNABLE_ID).is_some());

    Ok(())
}
