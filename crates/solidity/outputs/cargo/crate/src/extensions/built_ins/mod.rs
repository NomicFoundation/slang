use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum BuiltInTag {
    // fields of global `msg`
    MsgSender,
    // require() functions
    RequireBool,
    RequireBoolString,
    RequireBoolError,
}
