// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function foo() virtual public virtual {}
    function bar() virtual public virtual virtual {}
}
