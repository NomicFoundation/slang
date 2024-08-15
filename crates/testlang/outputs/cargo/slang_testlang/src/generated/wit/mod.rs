// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "./interface/generated/_types.rs"]
mod interface;
mod utils;
mod wrappers;

struct World;

crate::wit::interface::export!(World with_types_in crate::wit::interface);
