#![allow(unused_imports, unused_macros)] // Used in the output crates

macro_rules! scan_chars {
    ($stream:ident, $($char:literal),+) => {
        if $( $stream.next() == Some($char) )&&+ {
            true
        } else {
            $stream.undo();
            false
        }
    };
}

pub(crate) use scan_chars;

macro_rules! scan_none_of {
    ($stream:ident, $($char:literal),+) => {
        if let Some(c) = $stream.next() {
            if $(c != $char)&&+ {
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

pub(crate) use scan_none_of;

macro_rules! scan_char_range {
    ($stream:ident, $from:literal..=$to:literal) => {
        if let Some(c) = $stream.next() {
            #[allow(clippy::manual_is_ascii_check)]
            if ($from..=$to).contains(&c) {
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

pub(crate) use scan_char_range;

macro_rules! scan_sequence {
    ($($scanner:expr),*) => {
        $(($scanner))&&*
    };
}

pub(crate) use scan_sequence;

macro_rules! scan_choice {
    ($stream:ident, $($scanner:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                if ($scanner) { break true }
                $stream.set_position(save);
            )*
            break false
        }
    };
}

pub(crate) use scan_choice;

macro_rules! scan_keyword_choice {
    ($stream:ident, $ident_len:expr, $($scanner:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                {
                    if let result @ (KeywordScan::Present(..) | KeywordScan::Reserved(..)) = ($scanner) {
                        if $ident_len == $stream.position() - save {
                            break result;
                        }
                    }
                }
                $stream.set_position(save);
            )*
            break KeywordScan::Absent;
        }
    };
}

pub(crate) use scan_keyword_choice;

macro_rules! scan_zero_or_more {
    ($stream:ident, $scanner:expr) => {
        loop {
            let save = $stream.position();
            if !($scanner) {
                $stream.set_position(save);
                break true;
            }
        }
    };
}

pub(crate) use scan_zero_or_more;

macro_rules! scan_one_or_more {
    ($stream:ident, $scanner:expr) => {{
        let mut count = 0;
        #[allow(clippy::redundant_else)]
        loop {
            let save = $stream.position();
            if !($scanner) {
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

pub(crate) use scan_one_or_more;

macro_rules! scan_optional {
    ($stream:ident, $scanner:expr) => {{
        let save = $stream.position();
        if !($scanner) {
            $stream.set_position(save)
        }
        true
    }};
}

pub(crate) use scan_optional;

macro_rules! scan_not_followed_by {
    ($stream:ident, $scanner:expr, $not_followed_by:expr) => {
        ($scanner)
            && ({
                let end = $stream.position();
                let following = $not_followed_by;
                $stream.set_position(end);
                !following
            })
    };
}

pub(crate) use scan_not_followed_by;
