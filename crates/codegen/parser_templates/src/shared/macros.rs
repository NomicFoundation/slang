#[allow(unused_macros)]
macro_rules! scan_predicate {
    ($stream:ident, $predicate:expr) => {
        if let Some(c) = $stream.next() {
            if $predicate(c) {
                true
            } else {
                $stream.undo();
                false
            }
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_chars {
    ($stream:ident, $($char:literal),+) => {
        if $( $stream.next() == Some($char) )&&* {
            true
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_trie {
    ($stream:ident, EMPTY, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        ({
            match $stream.next() {
                $($(Some($terminal))|* => true,)?
                $(Some($prefix) => $subtree,)*
                _ => { $stream.undo(); true }
            }
        })
    };
    ($stream:ident, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        match $stream.next() {
            $($(Some($terminal))|* => true,)?
            $(Some($prefix) => $subtree,)*
            _ => { $stream.undo(); false }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_sequence {
    ($($expr:expr),*) => {
        $(($expr))&&*
    };
}

#[allow(unused_macros)]
macro_rules! scan_choice {
    ($stream:ident, $($expr:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                if ($expr) { break true }
                $stream.set_position(save);
            )*
            break false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_zero_or_more {
    ($stream:ident, $expr:expr) => {
        loop {
            let save = $stream.position();
            if !($expr) {
                $stream.set_position(save);
                break true;
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_one_or_more {
    ($stream:ident, $expr:expr) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < 1 {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_repeated {
    ($stream:ident, $expr:expr, $min:literal, $max:literal) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < $min {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
            if count == $max {
                break true;
            }
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_optional {
    ($stream:ident, $expr:expr) => {{
        let save = $stream.position();
        if !($expr) {
            $stream.set_position(save)
        }
        true
    }};
}

#[allow(unused_macros)]
macro_rules! scan_difference {
    ($stream:ident, $minuend:expr, $subtrahend:expr) => {{
        let start = $stream.position();
        ($minuend)
            && ({
                let end = $stream.position();
                $stream.set_position(start);
                if ($subtrahend) && (end == $stream.position()) {
                    false
                } else {
                    $stream.set_position(end);
                    true
                }
            })
    }};
}

#[allow(unused_macros)]
macro_rules! scan_not_followed_by {
    ($stream:ident, $expression:expr, $not_followed_by:expr) => {
        ($expression)
            && ({
                let end = $stream.position();
                let following = $not_followed_by;
                $stream.set_position(end);
                !following
            })
    };
}
