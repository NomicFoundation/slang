// Make sure we don't panic when encountering an unwinding close brace in a precedence parser

{
    a.b('
        }');
}
