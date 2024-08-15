#[path = "./interface/generated/_types.rs"]
mod interface;
mod utils;
mod wrappers;

struct World;

crate::wit::interface::export!(World with_types_in crate::wit::interface);
