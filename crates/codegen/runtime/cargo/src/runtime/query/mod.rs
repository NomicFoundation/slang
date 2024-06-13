use metaslang_cst::query;

use crate::cst::KindTypes;

pub type Query = query::Query<KindTypes>;
pub type QueryMatch = query::QueryMatch<KindTypes>;
pub type QueryMatchIterator = query::QueryMatchIterator<KindTypes>;
