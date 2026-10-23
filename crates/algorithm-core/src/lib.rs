use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Symbol(String),
    Array(Vec<Value>),
    Null,
}

#[derive(Default, Debug)]
pub struct Context {
    values: HashMap<String, Value>,
}

impl Context {
    pub fn set(&mut self, name: impl Into<String>, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

pub trait Algorithm: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, context: &mut Context) -> Result<Value, String>;
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(String, Value),
    Return(Value),
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, children: Vec::new() }
    }

    pub fn add(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}

#[derive(Clone, Debug)]
pub struct Specification {
    pub problem: String,
    pub goals: Vec<String>,
    pub constraints: Vec<String>,
}
