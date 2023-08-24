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
macro_rules! scan_none_of {
    ($stream:ident, $($char:literal),+) => {
        if let Some(c) = $stream.next() {
            if $(c != $char)&&* {
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
macro_rules! scan_char_range {
    ($stream:ident, $from:literal , $to:literal) => {
        if let Some(c) = $stream.next() {
            if $from <= c && c <= $to {
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
macro_rules! scan_sequence {
    ($($scanner:expr),*) => {
        $(($scanner))&&*
    };
}

#[allow(unused_macros)]
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

#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! scan_one_or_more {
    ($stream:ident, $scanner:expr) => {{
        let mut count = 0;
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

#[allow(unused_macros)]
macro_rules! scan_optional {
    ($stream:ident, $scanner:expr) => {{
        let save = $stream.position();
        if !($scanner) {
            $stream.set_position(save)
        }
        true
    }};
}

#[allow(unused_macros)]
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
