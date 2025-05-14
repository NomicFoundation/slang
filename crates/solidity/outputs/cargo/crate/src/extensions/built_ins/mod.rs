use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum BuiltInTag {
    // fields of global `msg`
    MsgSender,
    // require() functions
    RequireBool,
    RequireBoolString,
    RequireBoolError,
}
