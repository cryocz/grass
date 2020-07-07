use std::hash::{Hash, Hasher};

use codemap::Span;

use crate::{args::FuncArgs, scope::Scope, Token};

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub scope: Scope,
    pub args: FuncArgs,
    pub body: Vec<Token>,
    pos: Span,
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Function {}

impl Function {
    pub fn new(scope: Scope, args: FuncArgs, body: Vec<Token>, pos: Span) -> Self {
        Function {
            scope,
            args,
            body,
            pos,
        }
    }
}
