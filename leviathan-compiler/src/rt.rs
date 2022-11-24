use crate::resolver::Resolver;
use leviathan_common::structure::Namespace;
use std::str::FromStr;

macro_rules! namespace {
    ($arg:expr) => {
        Namespace::from_str($arg).unwrap()
    };
}

pub fn load_rt(resolver: &mut Resolver) {
    resolver.prelude.push(namespace!("rt"));
}
