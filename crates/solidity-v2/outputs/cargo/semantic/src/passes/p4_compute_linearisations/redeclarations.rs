//! The redeclaration check: reports a member that illegally reuses the name of
//! a member inherited from a base contract or interface.

use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_ir::ir;

use super::Lineariser;
use crate::binder::Definition;

impl<'a> Lineariser<'a> {
    /// Reports members of this base that redeclare a same-named inherited member.
    ///
    /// Only members in this type's namespace can clash: its own declarations
    /// (even `private` ones, which still shadow a visible inherited member), plus
    /// a proper base's members that are visible to it. A proper base's own
    /// private member isn't inherited, so it never clashes.
    ///
    /// `declared_here` is `true` when the members are those from the
    /// contract/interface being processed by the `Lineariser`.
    pub(super) fn check_redeclarations(&mut self, members: &[&'a Definition], declared_here: bool) {
        if self.members_by_name.is_empty() {
            // Nothing inherited to clash with yet: always true for the first
            // base folded, and for every base of a type without any.
            return;
        }
        for definition in members {
            if !declared_here && !definition.is_internally_visible() {
                continue;
            }
            let name = definition.identifier().unparse();
            let Some(slot) = self.members_by_name.get(name) else {
                continue;
            };
            if slot.clashes_with(definition) && self.reported.insert(definition.node_id()) {
                // TODO: should we emit a different diagnostic for SDR[1259]
                let file_id = self
                    .file_node_mapper
                    .file_id_from_node_id(definition.node_id());
                self.diagnostics.push(
                    file_id.to_owned(),
                    definition.identifier().range.clone(),
                    IdentifierRedeclaration,
                );
            }
        }
    }

    /// Records this base's inheritable members so more-derived bases are checked
    /// against them. Private members (and external functions) aren't inherited,
    /// so reusing their name is allowed.
    pub(super) fn record_members(&mut self, members: &[&'a Definition]) {
        for definition in members {
            if definition.is_internally_visible() {
                let kind = MemberKind::of(definition);
                self.members_by_name
                    .entry(definition.identifier().unparse())
                    .and_modify(|slot| *slot = slot.merge(kind))
                    .or_insert(kind);
            }
        }
    }
}

/// The kind of a named member, for redeclaration purposes. Used both to classify
/// a member and to summarise the members occupying a name across a hierarchy.
///
/// Same-named members coexist only when they're all the same overloadable kind
/// (all functions, all modifiers, or all events); Solidity allows overloading
/// functions/events and overriding functions/modifiers. Once members of
/// differing kinds share a name the slot collapses to [`Self::Other`], under
/// which any further same-named member is a redeclaration — so accumulating
/// members only ever moves a name's slot towards `Other`, never back.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum MemberKind {
    Function,
    Modifier,
    Event,
    /// A `public` state variable. As a *derived* member its getter may override
    /// an inherited function; as an inherited member (or name slot) it never
    /// lets a same-named member coexist, behaving like [`Self::Other`].
    StateVarPublic,
    /// Any other named member (non-public state variables, structs, enums,
    /// errors, ...), or a name slot occupied by members of differing kinds:
    /// never coexists with a same-named member.
    Other,
}

impl MemberKind {
    /// Classifies `definition`.
    fn of(definition: &Definition) -> Self {
        match definition {
            Definition::Function(_) => Self::Function,
            Definition::Modifier(_) => Self::Modifier,
            Definition::Event(_) => Self::Event,
            Definition::StateVariable(state_variable)
                if matches!(
                    state_variable.ir_node.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                Self::StateVarPublic
            }
            _ => Self::Other,
        }
    }

    /// Folds another same-named member into a name's slot; differing kinds
    /// collapse to [`Self::Other`].
    fn merge(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::Other
        }
    }

    /// Whether a `derived` member redeclares the members occupying this slot. A
    /// function — or a public state variable's getter — may share its name only
    /// with functions; a modifier only with modifiers; an event only with
    /// events; anything else with nothing.
    fn clashes_with(self, derived: &Definition) -> bool {
        let derived = Self::of(derived);
        match self {
            Self::Function => !matches!(derived, Self::Function | Self::StateVarPublic),
            Self::Modifier => derived != Self::Modifier,
            Self::Event => derived != Self::Event,
            Self::StateVarPublic | Self::Other => true,
        }
    }
}
