mod alias_following;
mod binder;
mod contract_dependencies;
mod getter_overrides;
mod support;
mod typing;
mod user_defined_operator_functions;

use support::{
    Analyse, Analysis, AnalysisBuilder, diagnostic_kind, find_function, only_diagnostic,
};
