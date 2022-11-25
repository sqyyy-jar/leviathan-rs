use crate::resolver::{CompileTask, Package, Signature, Overloader};
use leviathan_common::structure::{Namespace, Type};
use std::str::FromStr;

macro_rules! namespace {
    ($namespace:expr) => {
        Namespace::from_str($namespace).unwrap()
    };
}

macro_rules! signature {
    ($args:expr) => {
        Overloader::from(Signature::new($args))
    }
}

macro_rules! rgstr {
    ($ins:expr, $name:expr, $signature:expr) => {
        $ins.functions.insert($name.into(), $signature);
    }
}

pub fn load_rt(task: &mut CompileTask) {
    task.prelude.push(namespace!("rt"));
    task.root.packages.insert("rt".into(), Package::new());
    let ins = task.root.packages.get_mut("rt").unwrap();
    rgstr!(ins, "+", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "add", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "-", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "sub", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "*", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "mul", signature!(vec![Type::Float, Type::Float]));
    // TODO rgstr!(ins, "/", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "div", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "%", signature!(vec![Type::Float, Type::Float]));
    rgstr!(ins, "mod", signature!(vec![Type::Float, Type::Float]));
}
