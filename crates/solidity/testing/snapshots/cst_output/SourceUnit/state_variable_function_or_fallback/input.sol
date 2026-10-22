contract A {
    // State variables: an identifier followed by `;` or `=` is the variable name.
    function() a;
    function() external b;
    function() internal c; 
    function() payable d; 
    function() external e = f; 
    function() external onlyOwner; 

    // Unnamed (fallback) functions: no identifier is followed by `;` or `=`.
    function() external; // function without an identifier, not a state variable
    function() external { } // function with a body, not a state variable
    function() payable onlyOwner { } // function with a body, not a state variable
    function() onlyOwner(1); // function with an `onlyOwner(1)` modifier, not a state variable
    function() x external; // function with an `x` modifier before an attribute, not a state variable
}
