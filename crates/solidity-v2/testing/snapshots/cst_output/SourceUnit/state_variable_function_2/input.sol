contract C {
    // external works!
    function () external public x;
    function () external internal y;
    function () external private z;

    // internal fails for another reason on public, but should parse
    function () internal public xx;
    function () internal internal yy;
    function () internal private zz;

    // public doesn't parse on solc, but we should parse and later validate
    function () public public xxx;
    function () public internal yyy;
    function () public private zzz;

    // private doesn't parse on solc, but we should parse and later validate
    function () private public xxxx;
    function () private internal yyyy;
    function () private private zzzz;
}
