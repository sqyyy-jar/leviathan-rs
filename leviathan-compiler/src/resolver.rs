use std::collections::HashMap;

use leviathan_common::structure::Structure;

pub struct Resolver {
    functions: Vec<()>,
    prelude: Vec<String>,
    scope: HashMap<String, String>,
}

pub struct ResolvedStructure {}

pub fn resolve(structure: Structure) {
    todo!("resolve to ResolvedStructure")
}

pub trait ResolverPlugin {
    fn run(resolver: &mut Resolver);
}

pub struct FunctionSignature {
    name: String,
    arguments: Vec<(String, String)>,
}
