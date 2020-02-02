//! # Convert from SCSS AST to CSS
use crate::atrule::AtRule;
use crate::{RuleSet, SassResult, Selector, Stmt, Style, StyleSheet};
use std::fmt;
use std::io::Write;

#[derive(Debug, Clone)]
enum Toplevel {
    RuleSet(Selector, Vec<BlockEntry>),
    MultilineComment(String),
    AtRule(AtRule),
    Newline,
}

#[derive(Debug, Clone)]
enum BlockEntry {
    Style(Style),
    MultilineComment(String),
    AtRule(AtRule),
}

impl fmt::Display for BlockEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockEntry::Style(s) => writeln!(f, "  {}", s),
            BlockEntry::MultilineComment(s) => writeln!(f, "  /*{}*/", s),
            BlockEntry::AtRule(r) => writeln!(f, "{}", r),
        }
    }
}

impl Toplevel {
    const fn new_rule(selector: Selector) -> Self {
        Toplevel::RuleSet(selector, Vec::new())
    }

    fn push_style(&mut self, s: Style) {
        if let Toplevel::RuleSet(_, entries) = self {
            entries.push(BlockEntry::Style(s));
        }
    }

    fn push_comment(&mut self, s: String) {
        if let Toplevel::RuleSet(_, entries) = self {
            entries.push(BlockEntry::MultilineComment(s));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Css {
    blocks: Vec<Toplevel>,
}

impl Css {
    pub const fn new() -> Self {
        Css { blocks: Vec::new() }
    }

    pub fn from_stylesheet(s: StyleSheet) -> Self {
        Css::new().parse_stylesheet(s)
    }

    fn parse_stmt(&mut self, stmt: Stmt) -> Vec<Toplevel> {
        match stmt {
            Stmt::RuleSet(RuleSet {
                selector,
                super_selector,
                rules,
            }) => {
                let mut vals = vec![Toplevel::new_rule(super_selector.zip(&selector))];
                for rule in rules {
                    match rule {
                        Stmt::RuleSet(_) => vals.extend(self.parse_stmt(rule)),
                        Stmt::Style(s) => vals
                            .get_mut(0)
                            .expect("expected block to exist")
                            .push_style(s),
                        Stmt::MultilineComment(s) => vals
                            .get_mut(0)
                            .expect("expected block to exist")
                            .push_comment(s),
                        Stmt::AtRule(_) => todo!("at rule inside css block"),
                    };
                }
                vals
            }
            Stmt::MultilineComment(s) => vec![Toplevel::MultilineComment(s)],
            Stmt::Style(_) => panic!("expected toplevel element, found style"),
            Stmt::AtRule(r) => vec![Toplevel::AtRule(r)],
        }
    }

    fn parse_stylesheet(mut self, s: StyleSheet) -> Css {
        let mut is_first = true;
        for stmt in s.0 {
            let v = self.parse_stmt(stmt);
            // this is how we print newlines between unrelated styles
            // it could probably be refactored
            if !v.is_empty() {
                if let Toplevel::MultilineComment(..) = v[0] {
                } else if is_first {
                    is_first = false;
                } else {
                    self.blocks.push(Toplevel::Newline);
                }
            }
            self.blocks.extend(v);
        }
        self
    }

    pub fn pretty_print<W: Write>(self, buf: &mut W) -> SassResult<()> {
        let mut has_written = false;
        for block in self.blocks {
            match block {
                Toplevel::RuleSet(selector, styles) => {
                    if styles.is_empty() {
                        continue;
                    }
                    has_written = true;
                    writeln!(buf, "{} {{", selector)?;
                    for style in styles {
                        write!(buf, "{}", style)?;
                    }
                    writeln!(buf, "}}")?;
                }
                Toplevel::MultilineComment(s) => {
                    has_written = true;
                    writeln!(buf, "/*{}*/", s)?;
                }
                Toplevel::AtRule(r) => {
                    has_written = true;
                    writeln!(buf, "{}", r)?;
                }
                Toplevel::Newline => {
                    if has_written {
                        writeln!(buf)?
                    }
                }
            }
        }
        Ok(())
    }
}
