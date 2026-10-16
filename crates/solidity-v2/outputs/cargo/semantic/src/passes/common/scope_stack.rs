use crate::binder::ScopeId;

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // used to validate that pushes and pops pair up, and to find the
    // enclosing scopes of a given kind.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

/// The scopes enclosing the current point of a pass's traversal. Each frame
/// is pushed by a node and popped when leaving it, and its lexical scope can
/// move forward in between, as declarations come into view.
#[derive(Default)]
pub(crate) struct ScopeStack {
    frames: Vec<ScopeFrame>,
}

impl ScopeStack {
    /// Pushes a frame for `scope_id`, where names also resolve.
    #[inline]
    pub(crate) fn push(&mut self, scope_id: ScopeId) {
        self.frames.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
    }

    /// Pushes a frame for `scope_id`, while names keep resolving in the
    /// enclosing lexical scope.
    #[inline]
    pub(crate) fn push_structural(&mut self, scope_id: ScopeId) {
        let lexical_scope_id = self.current();
        self.frames.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id,
        });
    }

    /// Makes `lexical_scope_id` the scope names resolve in, keeping the
    /// current frame.
    #[inline]
    pub(crate) fn replace(&mut self, lexical_scope_id: ScopeId) {
        self.frames
            .last_mut()
            .expect("scope stack cannot be empty")
            .lexical_scope_id = lexical_scope_id;
    }

    /// Pops the current frame, which must have been pushed for `scope_id`.
    #[inline]
    pub(crate) fn pop(&mut self, scope_id: ScopeId) {
        let frame = self
            .frames
            .pop()
            .expect("attempt to pop an empty scope stack");
        assert_eq!(frame.structural_scope_id, scope_id);
    }

    /// The scope names resolve in at the current point of the traversal.
    #[inline]
    pub(crate) fn current(&self) -> ScopeId {
        self.frames
            .last()
            .expect("scope stack cannot be empty")
            .lexical_scope_id
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// The lexical scope of each frame, innermost first.
    #[inline]
    pub(crate) fn lexical_scopes(&self) -> impl Iterator<Item = ScopeId> + '_ {
        self.frames.iter().rev().map(|frame| frame.lexical_scope_id)
    }

    /// The structural scope of each frame, innermost first.
    #[inline]
    pub(crate) fn structural_scopes(&self) -> impl Iterator<Item = ScopeId> + '_ {
        self.frames
            .iter()
            .rev()
            .map(|frame| frame.structural_scope_id)
    }
}
