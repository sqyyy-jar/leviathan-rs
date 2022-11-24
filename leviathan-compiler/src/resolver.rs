use leviathan_common::{
    prelude::*,
    structure::{Expression, ExpressionType, Function, Namespace, Structure},
};
use std::collections::{HashMap, LinkedList};

pub struct Resolver {
    pub prelude: Vec<Namespace>,
    pub functions: HashMap<Namespace, FunctionOverloader>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            prelude: Vec::with_capacity(0),
            functions: HashMap::with_capacity(0),
        }
    }

    pub fn load_structure(&mut self, structure: Structure) -> Result<()> {
        let Structure {
            namespace,
            imports: _,
            functions,
        } = structure;
        for function in functions {
            let function_namespace = namespace.clone_with_package(function.name.clone());
            if !self.functions.contains_key(&function_namespace) {
                self.functions
                    .insert(function_namespace, FunctionOverloader::with(function));
            } else {
                self.functions
                    .get_mut(&function_namespace)
                    .unwrap()
                    .0
                    .push_back(function);
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        for node in &self.functions {
            for function in &(node.1).0 {
                self.validate_function(function)?;
            }
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
        let function_overloader;
        if self.functions.contains_key(operator) {
            function_overloader = self.functions.get(operator).unwrap();
        } else {
            'outer: {
                for prelude in &self.prelude {
                    let merged = prelude.clone_merge(operator);
                    if self.functions.contains_key(&merged) {
                        function_overloader = self.functions.get(&merged).unwrap();
                        break 'outer;
                    }
                }
                return Err(Error::Generic("Function not found".into()));
            }
        }
        for _overload in &function_overloader.0 {
            return Ok(());
        }
        Err(Error::Generic("Function overload not found".into()))
    }
}

pub struct FunctionOverloader(pub LinkedList<Function>);

impl FunctionOverloader {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn with(function: Function) -> Self {
        Self({
            let mut it = LinkedList::new();
            it.push_back(function);
            it
        })
    }
}
