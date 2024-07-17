use metaslang_cst::KindTypes;
use metaslang_graph_builder::functions::Functions;
use semver::Version;

pub fn default_functions<KT: KindTypes + 'static>(version: Version) -> Functions<KT> {
    let mut functions = Functions::stdlib();
    version::add_version_functions(&mut functions, version);
    functions
}

pub mod version {
    use metaslang_cst::KindTypes;
    use metaslang_graph_builder::functions::{Function, Functions, Parameters};
    use metaslang_graph_builder::graph::{Graph, Value};
    use metaslang_graph_builder::ExecutionError;
    use semver::{Version, VersionReq};

    pub fn add_version_functions<KT: KindTypes + 'static>(
        functions: &mut Functions<KT>,
        version: Version,
    ) {
        functions.add("version-matches".into(), VersionMatches { version });
    }

    struct VersionMatches {
        version: Version,
    }

    impl<KT: KindTypes> Function<KT> for VersionMatches {
        fn call(
            &self,
            _graph: &mut Graph<KT>,
            parameters: &mut dyn Parameters,
        ) -> Result<Value, ExecutionError> {
            let requirements_str = parameters.param()?.into_string()?;
            parameters.finish()?;

            let Ok(requirements) = VersionReq::parse(&requirements_str) else {
                return Err(ExecutionError::FunctionFailed(
                    "version-matches".into(),
                    format!("Failed to parse version requirements {requirements_str}"),
                ));
            };
            let result = requirements.matches(&self.version);
            Ok(result.into())
        }
    }
}
