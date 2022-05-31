use chumsky::prelude::*;
use chumsky::Parser;

pub trait NomicParser {
    fn excluding<'a, S, SO>(self, subtrahend: S) -> BoxedParser<'a, char, Vec<char>, Simple<char>>
    where
        Self: 'a + Sized + Parser<char, Vec<char>, Error = Simple<char>>,
        S: 'a + Parser<char, SO, Error = Simple<char>>;
}

impl<M> NomicParser for M
where
    M: Parser<char, Vec<char>, Error = Simple<char>>,
{
    fn excluding<'a, S, SO>(self, subtrahend: S) -> BoxedParser<'a, char, Vec<char>, Simple<char>>
    where
        Self: 'a + Sized + Parser<char, Vec<char>, Error = Simple<char>>,
        S: 'a + Parser<char, SO, Error = Simple<char>>,
    {
        let subtrahend = subtrahend.then(end());
        self.try_map(
            move |s: Vec<char>, span| match subtrahend.parse(s.as_slice()) {
                Ok(_) => Err(Simple::custom(
                    span,
                    "Matches subtrahend of difference operator",
                )),
                Err(_) => Ok(s),
            },
        )
        .boxed()
    }
}
