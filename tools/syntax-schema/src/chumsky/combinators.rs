#![allow(deprecated)] // TODO: Don't allow this
use chumsky::debug::{Debugger, Silent, Verbose};
use chumsky::error::{Error, Located};
use chumsky::Parser;
use chumsky::Stream;

// ([], Ok((out, alt_err))) => parsing successful,
// alt_err = potential alternative error should a different number of optional patterns be parsed
// ([x, ...], Ok((out, alt_err)) => parsing failed, but recovery occurred so parsing may continue
// ([...], Err(err)) => parsing failed, recovery failed, and one or more errors were produced
// TODO: Change `alt_err` from `Option<Located<I, E>>` to `Vec<Located<I, E>>`
type PResult<I, O, E> = (
    Vec<Located<I, E>>,
    Result<(O, Option<Located<I, E>>), Located<I, E>>,
);

// Shorthand for a stream with the given input and error type.
type StreamOf<'a, I, E> = Stream<'a, I, <E as Error<I>>::Span>;

#[derive(Copy, Clone)]
pub struct Excluding<A, B>(pub(crate) A, pub(crate) B);

impl<I: Clone, O, A: Parser<I, O, Error = E>, B, E: Error<I>> Parser<I, O> for Excluding<A, B> {
    type Error = E;

    #[inline]
    fn parse_inner<D: Debugger>(
        &self,
        debugger: &mut D,
        stream: &mut StreamOf<I, E>,
    ) -> PResult<I, O, E> {
        todo!("This is not yet implemented correctly");
        #[allow(deprecated)]
        debugger.invoke(&self.0, stream)
    }

    #[inline]
    fn parse_inner_verbose(&self, d: &mut Verbose, s: &mut StreamOf<I, E>) -> PResult<I, O, E> {
        #[allow(deprecated)]
        self.parse_inner(d, s)
    }

    #[inline]
    fn parse_inner_silent(&self, d: &mut Silent, s: &mut StreamOf<I, E>) -> PResult<I, O, E> {
        #[allow(deprecated)]
        self.parse_inner(d, s)
    }
}

pub trait SlangParserExtensions<I, O> {
    fn excluding<P>(self, other: P) -> Excluding<Self, P>
    where
        I: Clone,
        Self: Sized + Parser<I, O>;
}

impl<T, I, O> SlangParserExtensions<I, O> for T
where
    I: Clone,
    T: Sized + Parser<I, O>,
{
    fn excluding<P>(self, other: P) -> Excluding<Self, P> {
        Excluding(self, other)
    }
}
