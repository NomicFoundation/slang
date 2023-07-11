mod legacy;

pub mod syntax;

// Make sure codegen runs before building for tests.
#[cfg(test)]
use solidity_npm_build as _;

#[macro_use]
extern crate napi_derive;
