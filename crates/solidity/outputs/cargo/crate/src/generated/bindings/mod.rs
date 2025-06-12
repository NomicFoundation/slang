// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! This module contains APIs used to resolve symbol bindings in source code. These APIs can be used
//! for code analysis features such as "Go to Definition" or "Find References".
#[path = "generated/binding_rules.rs"]
mod binding_rules;

use std::rc::Rc;

use semver::Version;

use crate::cst::KindTypes;

/// A utility for building a [`BindingGraph`] for a collection of source files.
pub type BindingGraphBuilder = metaslang_bindings::BindingGraphBuilder<KindTypes>;
/// A `BindingGraph` is constructed from one or many source files. Once constructed,
/// it allows the user to link symbol references to their definitions, and vice versa.
pub type BindingGraph = metaslang_bindings::BindingGraph<KindTypes>;
/// A `Definition` represents the location where a symbol is originally defined. From this you
/// can find more information about the location of this definition in the parse tree, or navigate to
/// [references][`Reference`] to this defintion.
pub type Definition = metaslang_bindings::Definition<KindTypes>;
/// A `Reference` represents a location where a symbol definition is referenced, i.e. anywhere
/// a symbol is 'used' in a piece of code. From this you can find more information about the location
/// of the reference in the parse tree, or you can navigate to the [definitions][`Definition`] this
/// references.
///
/// Note that most references have a single definition, but some have multiple, such as when a symbol
/// is imported from another file, and renamed (re-defined) in the current file.
pub type Reference = metaslang_bindings::Reference<KindTypes>;
/// A `BindingLocation` is used to point the user to a definition or reference in the parse tree.
/// A `BindingLocation` can either be a [`UserFile`][`BindingLocation::UserFile`] or a [`BuiltIn`][`BindingLocation::BuiltIn`].
/// Only `UserFile`'s give the user a [`Cursor`][`crate::cst::Cursor`] into a location in the
/// parse tree.
pub type BindingLocation = metaslang_bindings::BindingLocation<KindTypes>;
/// `UserFileLocation` provides a [`Cursor`][`crate::cst::Cursor`] pointing to a [`Node`][`crate::cst::Node`] in the
/// parse tree where the referenced [`Definition`] or [`Reference`] is located.
pub type UserFileLocation = metaslang_bindings::UserFileLocation<KindTypes>;

pub use metaslang_bindings::{BuiltInLocation, PathResolver};

/// Create a new `BindingGraphBuilder` with the specified language version `version` and the [`PathResolver`]
/// `resolver`.
pub fn create_with_resolver(
    version: Version,
    resolver: Rc<dyn PathResolver<KindTypes>>,
) -> BindingGraphBuilder {
    let mut binding_graph = BindingGraphBuilder::create(
        version.clone(),
        binding_rules::BINDING_RULES_SOURCE,
        resolver,
    );

    crate::extensions::bindings::add_built_ins(&mut binding_graph, version);

    binding_graph
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}
