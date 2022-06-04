type TT = std::ops::Range<usize>;

enum Choice2<A, B> {
    A(A),
    B(B),
}

enum Choice3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

enum Choice7<A, B, C, D, E, F, G> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
}

type T1 = ();
type TN = usize;

type GComment = TN;
type GEOF = T1;
type GHexDigit = T1;
type GIdentifierStart = T1;
type GNumber = TN;
type GWhitespace = TN;
type GIgnore = Vec<Choice2<GComment, GWhitespace>>;
type GIdentifierFollow = T1;
type GStringChar = Choice2<T1, (T1, Choice3<T1, T1, (T1, Vec<GHexDigit>, T1)>)>;
type GRawIdentifier = (GIdentifierStart, Vec<GIdentifierFollow>);
type GSingleCharString = (T1, GStringChar, T1);
type GString = (T1, Vec<GStringChar>, T1);
type GGrouped = (T1, GIgnore, GExpression, T1, GIgnore);
type GOptional = (T1, GIgnore, GExpression, T1, GIgnore);
type GRepetitionPrefix = Choice2<
    (
        GNumber,
        GIgnore,
        Option<(T1, GIgnore, Option<(GNumber, GIgnore)>)>,
    ),
    (T1, GIgnore, GNumber, GIgnore),
>;
type GRepetitionSeparator = (T1, GIgnore, GExpression);
type GIdentifier = Choice2<(T1, GRawIdentifier, T1), GRawIdentifier>;
type GCharRange = (
    GSingleCharString,
    GIgnore,
    T1,
    GIgnore,
    GSingleCharString,
    GIgnore,
);
type GRepeated = (
    Option<GRepetitionPrefix>,
    T1,
    GIgnore,
    GExpression,
    Option<GRepetitionSeparator>,
    T1,
    GIgnore,
);
type GProductionReference = (GIdentifier, GIgnore);
type GPrimary = Choice7<
    GProductionReference,
    GGrouped,
    GOptional,
    GRepeated,
    GCharRange,
    (GEOF, GIgnore),
    (GString, GIgnore),
>;
type GNegation = (Option<(T1, GIgnore)>, GPrimary);
type GDifference = (GNegation, Option<(T1, GIgnore, GNegation)>);
type GSequence = Vec<GDifference>;
struct GExpressionData(GSequence, Vec<(T1, GIgnore, GSequence)>);
type GExpression = GExpressionData;
type GProduction = (GIdentifier, GIgnore, T1, GIgnore, GExpression, T1, GIgnore);
type GGrammar = (GIgnore, Vec<GProduction>);
