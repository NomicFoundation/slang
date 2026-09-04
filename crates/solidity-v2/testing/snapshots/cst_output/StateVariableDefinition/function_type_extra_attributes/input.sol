contract C {
    // The attributes of a function type without a return are greedily parsed,
    // so the ones that follow a repeated visibility belong to the state
    // variable. Those that are not valid there are reported as errors.
    function() internal public x;
    function() internal public payable y;
    function() internal public external z;
    function() internal public view constant w = f;
    function() internal external v;
    function() internal public pure u;
}
