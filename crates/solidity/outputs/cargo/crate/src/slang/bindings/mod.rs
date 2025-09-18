//! This module contains APIs used to resolve symbol bindings in source code. These APIs can be used
//! for code analysis features such as "Go to Definition" or "Find References".
#[path = "binding_rules.generated.rs"]
mod binding_rules;

use std::rc::Rc;

use semver::Version;

use crate::cst::KindTypes;

// A utility for building a [`BindingGraph`] for a collection of source files.
pub(crate) type BindingGraphBuilder = metaslang_bindings::BindingGraphBuilder<KindTypes>;

/// A graph that contains name binding information for all source files within the compilation unit.
/// It stores cursors to all definitions and references, and can resolve the edges between them.
///
/// Most cursors pointing to identifier terminals will resolve to either a definition or a reference.
/// For example, in `contract A is B {}`, the cursor to identifier `A` will resolve to a definition,
/// and the cursor to identifier `B` will resolve to a reference.
///
/// However, in some cases, cursors to identifiers can resolve to both at the same time.
/// For example, in `import {X} from "library"`, the cursor to identifier `X` will resolve to a
/// definition (the local import), and also to a reference (to the symbol exported from `"library"`).
///
/// This graph is error-tolerant, and will return `undefined` for any identifiers that cannot be resolved.
/// For example, when there are syntactic/semantic errors, or missing source files.
///
/// For more information on identifier terminals, see the [`TerminalKindExtensions::is_identifier`][`crate::cst::TerminalKindExtensions::is_identifier`] API.
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
/// Abstract representation of the location of a built-in object.
pub type BuiltInLocation = metaslang_bindings::BuiltInLocation;

#[cfg(feature = "__private_testing_utils")]
#[allow(missing_docs)]
// Create a new `BindingGraphBuilder` with the specified language version and resolver.
// Exposed to test the functionality, but users should use the `CompilationBuilder` instead.
pub fn create_with_resolver(
    version: Version,
    resolver: Rc<dyn metaslang_bindings::PathResolver<KindTypes>>,
) -> BindingGraphBuilder {
    create_with_resolver_internal(version, resolver)
}

pub(crate) fn create_with_resolver_internal(
    version: Version,
    resolver: Rc<dyn metaslang_bindings::PathResolver<KindTypes>>,
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
#[allow(missing_docs)]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}
