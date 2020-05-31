use super::{CompoundSelector, Pseudo, SelectorList, SimpleSelector};
use std::fmt::{self, Display, Write};

/// A complex selector.
///
/// A complex selector is composed of `CompoundSelector`s separated by
/// `Combinator`s. It selects elements based on their parent selectors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ComplexSelector {
    /// The components of this selector.
    ///
    /// This is never empty.
    ///
    /// Descendant combinators aren't explicitly represented here. If two
    /// `CompoundSelector`s are adjacent to one another, there's an implicit
    /// descendant combinator between them.
    ///
    /// It's possible for multiple `Combinator`s to be adjacent to one another.
    /// This isn't valid CSS, but Sass supports it for CSS hack purposes.
    pub components: Vec<ComplexSelectorComponent>,

    /// Whether a line break should be emitted *before* this selector.
    pub line_break: bool,
}

impl fmt::Display for ComplexSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_component = None;

        for component in &self.components {
            if let Some(c) = last_component {
                if !omit_spaces_around(c) && !omit_spaces_around(component) {
                    f.write_char(' ')?;
                }
            }
            write!(f, "{}", component)?;
            last_component = Some(component);
        }
        Ok(())
    }
}

/// When `style` is `OutputStyle::compressed`, omit spaces around combinators.
fn omit_spaces_around(component: &ComplexSelectorComponent) -> bool {
    // todo: compressed
    let is_compressed = false;
    is_compressed && matches!(component, ComplexSelectorComponent::Combinator(..))
}

impl ComplexSelector {
    // pub fn specificity(&self) -> Specificity {
    //     let mut min = 0;
    //     let mut max = 0;
    //     for component in self.components.iter() {
    //         todo!()
    //         // min += simple.min_specificity();
    //         // max += simple.max_specificity();
    //     }
    //     Specificity::new(min, max)
    // }

    pub fn is_invisible(&self) -> bool {
        self.components
            .iter()
            .any(ComplexSelectorComponent::is_invisible)
    }

    /// Returns whether `self` is a superselector of `other`.
    ///
    /// That is, whether `self` matches every element that `other` matches, as well
    /// as possibly additional elements.
    pub fn is_super_selector(&self, other: &Self) -> bool {
        if let Some(ComplexSelectorComponent::Combinator(..)) = self.components.last() {
            return false;
        }
        if let Some(ComplexSelectorComponent::Combinator(..)) = other.components.last() {
            return false;
        }

        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            let remaining1 = self.components.len() - i1;
            let remaining2 = other.components.len() - i2;

            if remaining1 == 0 || remaining2 == 0 || remaining1 > remaining2 {
                return false;
            }

            let compound1 = match self.components.get(i1) {
                Some(ComplexSelectorComponent::Compound(c)) => c,
                Some(ComplexSelectorComponent::Combinator(..)) => return false,
                None => unreachable!(),
            };

            if let ComplexSelectorComponent::Combinator(..) = other.components[i2] {
                return false;
            }

            if remaining1 == 1 {
                let parents = other
                    .components
                    .iter()
                    .take(other.components.len() - 1)
                    .skip(i2)
                    .cloned()
                    .collect();
                return compound1.is_super_selector(
                    other.components.last().unwrap().as_compound(),
                    &Some(parents),
                );
            }

            let mut after_super_selector = i2 + 1;
            while after_super_selector < other.components.len() {
                if let Some(ComplexSelectorComponent::Compound(compound2)) =
                    other.components.get(after_super_selector - 1)
                {
                    if compound1.is_super_selector(
                        compound2,
                        &Some(
                            other
                                .components
                                .iter()
                                .take(after_super_selector - 1)
                                .skip(i2 + 1)
                                .cloned()
                                .collect(),
                        ),
                    ) {
                        break;
                    }
                }

                after_super_selector += 1;
            }

            if after_super_selector == other.components.len() {
                return false;
            }

            if let Some(ComplexSelectorComponent::Combinator(combinator1)) =
                self.components.get(i1 + 1)
            {
                let combinator2 = match other.components.get(after_super_selector) {
                    Some(ComplexSelectorComponent::Combinator(c)) => c,
                    Some(ComplexSelectorComponent::Compound(..)) => return false,
                    None => unreachable!(),
                };

                if combinator1 == &Combinator::FollowingSibling {
                    if combinator2 == &Combinator::Child {
                        return false;
                    }
                } else if combinator1 != combinator2 {
                    return false;
                }

                if remaining1 == 3 && remaining2 > 3 {
                    return false;
                }

                i1 += 2;
                i2 = after_super_selector + 1;
            } else if let Some(ComplexSelectorComponent::Combinator(combinator2)) =
                other.components.get(after_super_selector)
            {
                if combinator2 != &Combinator::Child {
                    return false;
                }
                i1 += 1;
                i2 = after_super_selector + 1;
            } else {
                i1 += 1;
                i2 = after_super_selector;
            }
        }
    }

    pub fn contains_parent_selector(&self) -> bool {
        self.components.iter().any(|c| {
            if let ComplexSelectorComponent::Compound(compound) = c {
                compound.components.iter().any(|simple| {
                    if simple.is_parent() {
                        return true;
                    }
                    if let SimpleSelector::Pseudo(Pseudo {
                        selector: Some(sel),
                        ..
                    }) = simple
                    {
                        return sel.contains_parent_selector();
                    }
                    false
                })
            } else {
                false
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub(crate) enum Combinator {
    /// Matches the right-hand selector if it's immediately adjacent to the
    /// left-hand selector in the DOM tree.
    ///
    /// `'+'`
    NextSibling,

    /// Matches the right-hand selector if it's a direct child of the left-hand
    /// selector in the DOM tree.
    ///
    /// `'>'`
    Child,

    /// Matches the right-hand selector if it comes after the left-hand selector
    /// in the DOM tree.
    ///
    /// `'~'`
    FollowingSibling,
}

impl Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Self::NextSibling => '+',
            Self::Child => '>',
            Self::FollowingSibling => '~',
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ComplexSelectorComponent {
    Combinator(Combinator),
    Compound(CompoundSelector),
}

impl ComplexSelectorComponent {
    pub fn is_invisible(&self) -> bool {
        match self {
            Self::Combinator(..) => false,
            Self::Compound(c) => c.is_invisible(),
        }
    }

    pub fn is_compound(&self) -> bool {
        matches!(self, Self::Compound(..))
    }

    pub fn is_combinator(&self) -> bool {
        matches!(self, Self::Combinator(..))
    }

    pub fn resolve_parent_selectors(self, parent: SelectorList) -> Option<Vec<ComplexSelector>> {
        match self {
            Self::Compound(c) => c.resolve_parent_selectors(parent),
            Self::Combinator(..) => todo!(),
        }
    }

    pub fn as_compound(&self) -> &CompoundSelector {
        match self {
            Self::Compound(c) => c,
            Self::Combinator(..) => unreachable!(),
        }
    }
}

impl Display for ComplexSelectorComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compound(c) => write!(f, "{}", c),
            Self::Combinator(c) => write!(f, "{}", c),
        }
    }
}
