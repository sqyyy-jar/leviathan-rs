use leviathan_common::{
    prelude::*,
    structure::{Expression, ExpressionType, Function, Namespace, Structure, Type},
};
use std::collections::{HashMap, LinkedList};

#[derive(Debug)]
pub struct CompileTask {
    pub functions: Vec<Function>,
    pub prelude: Vec<Namespace>,
    pub root: Package,
}

impl CompileTask {
    pub fn new() -> Self {
        Self {
            functions: Vec::with_capacity(0),
            prelude: Vec::with_capacity(0),
            root: Package::new(),
        }
    }

    pub fn create_package_mut(&mut self, namespace: &Namespace, keep: usize) -> &mut Package {
        let mut iter = namespace.0.iter();
        let first = iter.next().unwrap();
        if !self.root.packages.contains_key(first) {
            self.root.packages.insert(first.clone(), Package::new());
        }
        let mut current = self.root.packages.get_mut(first).unwrap();
        while let Some(next) = iter.next() {
            if iter.len() <= keep {
                if !current.packages.contains_key(next) {
                    current.packages.insert(next.clone(), Package::new());
                }
                return current.packages.get_mut(next).unwrap();
            }
            if !current.packages.contains_key(next) {
                current.packages.insert(next.clone(), Package::new());
            }
            current = current.packages.get_mut(next).unwrap();
        }
        return current;
    }

    pub fn resolve_package(&self, namespace: &Namespace, keep: usize) -> Option<&Package> {
        let mut iter = namespace.0.iter();
        if iter.len() <= keep {
            return Some(&self.root);
        }
        let first = iter.next().unwrap();
        if !self.root.packages.contains_key(first) {
            return None;
        }
        let mut current = self.root.packages.get(first).unwrap();
        if iter.len() <= keep {
            return Some(current);
        }
        while let Some(next) = iter.next() {
            if iter.len() <= keep {
                if !current.packages.contains_key(next) {
                    return None;
                }
                return Some(current.packages.get(next).unwrap());
            }
            if !current.packages.contains_key(next) {
                return None;
            }
            current = current.packages.get(next).unwrap();
        }
        return Some(current);
    }

    pub fn resolve_package_mut(
        &mut self,
        namespace: &Namespace,
        keep: usize,
    ) -> Option<&mut Package> {
        let mut iter = namespace.0.iter();
        if iter.len() <= keep {
            return Some(&mut self.root);
        }
        let first = iter.next().unwrap();
        if !self.root.packages.contains_key(first) {
            return None;
        }
        let mut current = self.root.packages.get_mut(first).unwrap();
        if iter.len() <= keep {
            return Some(current);
        }
        while let Some(next) = iter.next() {
            if iter.len() <= keep {
                if !current.packages.contains_key(next) {
                    return None;
                }
                return Some(current.packages.get_mut(next).unwrap());
            }
            if !current.packages.contains_key(next) {
                return None;
            }
            current = current.packages.get_mut(next).unwrap();
        }
        return Some(current);
    }

    pub fn register_function(&mut self, function: Function) -> Result<()> {
        let signature = Signature::from(&function);
        self.register_signature(&function.name, signature)?;
        self.functions.push(function);
        Ok(())
    }

    pub fn register_signature(
        &mut self,
        namespace: &Namespace,
        signature: Signature,
    ) -> Result<()> {
        let mut iter = namespace.0.iter();
        let first = iter.next().unwrap();
        if !self.root.packages.contains_key(first) {
            self.root.packages.insert(first.clone(), Package::new());
        }
        let mut current = self.root.packages.get_mut(first).unwrap();
        while let Some(next) = iter.next() {
            if iter.len() == 0 {
                if !current.functions.contains_key(next) {
                    current
                        .functions
                        .insert(next.clone(), Overloader::from(signature));
                } else {
                    current
                        .functions
                        .get_mut(next)
                        .unwrap()
                        .0
                        .push_back(signature);
                }
                return Ok(());
            }
            if !current.packages.contains_key(next) {
                current.packages.insert(next.clone(), Package::new());
            }
            current = current.packages.get_mut(next).unwrap();
        }
        unreachable!()
    }

    pub fn load_structure(&mut self, structure: Structure) -> Result<()> {
        let Structure {
            namespace: _,
            imports: _,
            functions,
        } = structure;
        for function in functions {
            self.register_function(function)?;
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        for function in &self.functions {
            self.validate_function(function)?;
        }
        Ok(())
    }

    fn validate_function(&self, function: &Function) -> Result<()> {
        self.validate_expression(&function.code)
    }

    fn validate_expression(&self, expression: &Expression) -> Result<()> {
        match &expression.value {
            ExpressionType::Invoke {
                operator,
                arguments,
            } => self.validate_invokation(operator, arguments),
            ExpressionType::List(list) => {
                for value in list {
                    self.validate_expression(value)?;
                }
                Ok(())
            }
            ExpressionType::Map(map) => {
                for (_, value) in map {
                    self.validate_expression(value)?;
                }
                Ok(())
            }
            ExpressionType::Identifier(_) => Err(Error::Generic(
                "Arbitrary identifiers are not implemented yet".into(),
            )),
            ExpressionType::Atom(_) => Ok(()),
            ExpressionType::String(_) => Ok(()),
            ExpressionType::Integer(_) => Ok(()),
            ExpressionType::Float(_) => Ok(()),
            ExpressionType::Bool(_) => Ok(()),
        }
    }

    fn validate_invokation(
        &self,
        operator: &Namespace,
        _arguments: &Vec<Expression>,
    ) -> Result<()> {
        let package = self.resolve_package(operator, 1);
        if let Some(package) = package {
            if package.functions.contains_key(operator.0.last().unwrap()) {
                for _overload in &package.functions.get(operator.0.last().unwrap()).unwrap().0 {
                    // TODO implement overloads
                    return Ok(());
                }
            }
        } else {
            println!("Tried namespace {:?}", operator);
        }
        for prelude in &self.prelude {
            let package = self.resolve_package(&prelude.clone_merge(operator), 1);
            if let Some(package) = package {
                if package.functions.contains_key(operator.0.last().unwrap()) {
                    for _overload in &package.functions.get(operator.0.last().unwrap()).unwrap().0 {
                        // TODO implement overloads
                        return Ok(());
                    }
                }
            } else {
                println!("Tried namespace {:?}/{:?}", prelude, operator);
            }
        }
        Err(Error::Generic(format!("Could not resolve {:?}", operator)))
    }
}

#[derive(Debug)]
pub struct Package {
    pub packages: HashMap<String, Package>,
    pub functions: HashMap<String, Overloader>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            packages: HashMap::with_capacity(0),
            functions: HashMap::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct Overloader(LinkedList<Signature>);

impl Overloader {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }
}

impl From<Signature> for Overloader {
    fn from(signature: Signature) -> Self {
        Self({
            let mut it = LinkedList::new();
            it.push_back(signature);
            it
        })
    }
}

#[derive(Debug)]
pub struct Signature {
    pub arguments: Vec<Type>,
}

impl Signature {
    pub fn new(arguments: Vec<Type>) -> Self {
        Self { arguments }
    }
}

impl From<&Function> for Signature {
    fn from(_function: &Function) -> Self {
        Self {
            arguments: Vec::new(), // TODO
        }
    }
}
