#[crate_id = "data"];
#[crate_type = "rlib"];

#[feature(globs)];

extern mod extra;

pub mod card;
pub mod extiter;
pub mod monoid;
pub mod union_find;
