use std::sync::Arc;

use metaslang_cst::kinds::KindTypes;
use metaslang_graph_builder::functions::Functions;
use semver::Version;

use crate::PathResolver;

pub fn default_functions<KT: KindTypes + 'static>(
    version: Version,
    path_resolver: Arc<dyn PathResolver + Sync + Send>,
) -> Functions<KT> {
    let mut functions = Functions::stdlib();
    version::add_version_functions(&mut functions, version);
    resolver::add_functions(&mut functions, path_resolver);
    functions
}

mod version {
    use metaslang_cst::kinds::KindTypes;
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

mod resolver {
    use std::sync::Arc;

    use metaslang_cst::kinds::KindTypes;
    use metaslang_graph_builder::functions::{Function, Functions, Parameters};
    use metaslang_graph_builder::graph::{Graph, Value};
    use metaslang_graph_builder::ExecutionError;

    use crate::{FileDescriptor, PathResolver};

    pub fn add_functions<KT: KindTypes + 'static>(
        functions: &mut Functions<KT>,
        path_resolver: Arc<dyn PathResolver + Sync + Send>,
    ) {
        functions.add("resolve-path".into(), ResolvePath { path_resolver });
        functions.add("is-system-file".into(), IsSystemFile {});
    }

    struct ResolvePath {
        path_resolver: Arc<dyn PathResolver + Sync + Send>,
    }

    impl<KT: KindTypes> Function<KT> for ResolvePath {
        fn call(
            &self,
            _graph: &mut Graph<KT>,
            parameters: &mut dyn Parameters,
        ) -> Result<Value, ExecutionError> {
            let context_path = parameters.param()?.into_string()?;
            let path_to_resolve = parameters.param()?.into_string()?;
            parameters.finish()?;

            let context_file_descriptor = FileDescriptor::from_string(&context_path);
            let Ok(FileDescriptor::User(context_user_path)) = context_file_descriptor else {
                // Since the path resolver should only map to user paths from
                // user paths, it is an error to attempt to resolve a path in
                // the context of a system file
                return Err(ExecutionError::FunctionFailed(
                    "resolve-path".into(),
                    "Cannot execute with a non-user context file path".into(),
                ));
            };

            let resolved_path = self
                .path_resolver
                .as_ref()
                .resolve_path(&context_user_path, &path_to_resolve)
                .map_or_else(
                    || {
                        // In case we cannot resolve the path, we return a special value that is unique
                        // per context/path pait. This way, we can still run incrementally and resolve
                        // other symbols in the file:
                        format!("__SLANG_UNRESOLVED_PATH__{context_path}__{path_to_resolve}__")
                    },
                    |resolved_path| FileDescriptor::User(resolved_path).as_string(),
                );

            Ok(resolved_path.into())
        }
    }

    struct IsSystemFile {}

    impl<KT: KindTypes> Function<KT> for IsSystemFile {
        fn call(
            &self,
            _graph: &mut Graph<KT>,
            parameters: &mut dyn Parameters,
        ) -> Result<Value, ExecutionError> {
            let file_path = parameters.param()?.into_string()?;
            parameters.finish()?;

            let Ok(file_descriptor) = FileDescriptor::from_string(&file_path) else {
                return Err(ExecutionError::FunctionFailed(
                    "is-system-file".into(),
                    "Parameter is not a valid file path".into(),
                ));
            };
            Ok(file_descriptor.is_system().into())
        }
    }
}
