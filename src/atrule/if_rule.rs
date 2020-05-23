use codemap::{Spanned, Span};

use peekmore::{PeekMore, PeekMoreIterator};

use super::ruleset_eval;

use crate::error::SassResult;
use crate::scope::Scope;
use crate::selector::Selector;
use crate::utils::{
    devour_whitespace, devour_whitespace_or_comment, peek_ident_no_interpolation,
    read_until_closing_curly_brace, read_until_open_curly_brace,
};
use crate::value::Value;
use crate::{Stmt, Token};

#[derive(Debug, Clone)]
pub(crate) struct If {
    pub branches: Vec<Branch>,
    pub else_: Vec<Token>,
}

#[derive(Debug, Clone)]
pub(crate) struct Branch {
    pub cond: Vec<Token>,
    pub toks: Vec<Token>,
}

impl Branch {
    pub fn new(cond: Vec<Token>, toks: Vec<Token>) -> Branch {
        Branch { cond, toks }
    }
}

impl If {
    pub fn from_tokens<I: Iterator<Item = Token>>(
        toks: &mut PeekMoreIterator<I>,
        span_before: Span,
    ) -> SassResult<If> {
        devour_whitespace_or_comment(toks)?;
        let mut branches = Vec::new();
        let init_cond = read_until_open_curly_brace(toks);
        if toks.next().is_none() {
            return Err(("Expected expression.", span_before).into());
        }
        devour_whitespace_or_comment(toks)?;
        let mut init_toks = read_until_closing_curly_brace(toks);
        if let Some(tok) = toks.next() {
            init_toks.push(tok);
        } else {
            return Err(("expected \"}\".", span_before).into())
        }
        devour_whitespace(toks);

        branches.push(Branch::new(init_cond, init_toks));

        let mut else_ = Vec::new();

        loop {
            match toks.peek() {
                Some(Token { kind: '@', .. }) => {
                    toks.peek_forward(1);
                    let mut ident = peek_ident_no_interpolation(toks, false)?;
                    ident.node.make_ascii_lowercase();
                    if ident.as_str() != "else" {
                        toks.reset_view();
                        break;
                    }
                    toks.take(4).for_each(drop);
                }
                Some(..) | None => break,
            }
            devour_whitespace(toks);
            if let Some(tok) = toks.next() {
                devour_whitespace(toks);
                match tok.kind.to_ascii_lowercase() {
                    'i' if toks.next().unwrap().kind.to_ascii_lowercase() == 'f' => {
                        toks.next();
                        let cond = read_until_open_curly_brace(toks);
                        toks.next();
                        devour_whitespace(toks);
                        branches.push(Branch::new(cond, read_until_closing_curly_brace(toks)));
                        toks.next();
                        devour_whitespace(toks);
                    }
                    '{' => {
                        else_ = read_until_closing_curly_brace(toks);
                        toks.next();
                        break;
                    }
                    _ => {
                        return Err(("expected \"{\".", tok.pos()).into());
                    }
                }
            } else {
                break;
            }
        }
        devour_whitespace(toks);

        Ok(If { branches, else_ })
    }

    pub fn eval(
        self,
        scope: &mut Scope,
        super_selector: &Selector,
        content: Option<&[Spanned<Stmt>]>,
    ) -> SassResult<Vec<Spanned<Stmt>>> {
        let mut stmts = Vec::new();
        let mut toks = Vec::new();
        let mut found_true = false;
        for branch in self.branches {
            let val = Value::from_vec(branch.cond, scope, super_selector)?;
            if val.node.is_true(val.span)? {
                toks = branch.toks;
                found_true = true;
                break;
            }
        }
        if !found_true {
            toks = self.else_;
        }
        ruleset_eval(
            &mut toks.into_iter().peekmore(),
            scope,
            super_selector,
            false,
            content,
            &mut stmts,
        )?;
        Ok(stmts)
    }
}
