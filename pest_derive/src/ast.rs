// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use quote::Ident;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub name: Ident,
    pub ty: RuleType,
    pub expr: Expr
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleType {
    Normal,
    Silent,
    Atomic,
    CompoundAtomic,
    NonAtomic
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Str(String),
    Insens(String),
    Range(String, String),
    Ident(Ident),
    PosPred(Box<Expr>),
    NegPred(Box<Expr>),
    Seq(Box<Expr>, Box<Expr>),
    Choice(Box<Expr>, Box<Expr>),
    Opt(Box<Expr>),
    Rep(Box<Expr>),
    RepOnce(Box<Expr>),
    RepMinMax(Box<Expr>, u32, u32),
    Push(Box<Expr>)
}

impl Expr {
    pub fn map_top_down<F>(self, mut f: F) -> Expr where F: FnMut(Expr) -> Expr {
        pub fn map_internal<F>(expr: Expr, f: &mut F) -> Expr where F: FnMut(Expr) -> Expr {
            let expr = f(expr);

            match expr {
                // TODO: Use box syntax when it gets stabilized.
                Expr::PosPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::PosPred(mapped)
                }
                Expr::NegPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::NegPred(mapped)
                }
                Expr::Seq(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    Expr::Seq(mapped_lhs, mapped_rhs)
                }
                Expr::Choice(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    Expr::Choice(mapped_lhs, mapped_rhs)
                }
                Expr::Rep(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Rep(mapped)
                }
                Expr::RepOnce(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::RepOnce(mapped)
                }
                Expr::RepMinMax(expr, min, max) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::RepMinMax(mapped, min, max)
                }
                Expr::Opt(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Opt(mapped)
                }
                Expr::Push(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Push(mapped)
                }
                expr => expr
            }
        }

        map_internal(self, &mut f)
    }

    pub fn map_bottom_up<F>(self, mut f: F) -> Expr where F: FnMut(Expr) -> Expr {
        pub fn map_internal<F>(expr: Expr, f: &mut F) -> Expr where F: FnMut(Expr) -> Expr {
            let mapped = match expr {
                Expr::PosPred(expr) => {
                    // TODO: Use box syntax when it gets stabilized.
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::PosPred(mapped)
                }
                Expr::NegPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::NegPred(mapped)
                }
                Expr::Seq(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    Expr::Seq(mapped_lhs, mapped_rhs)
                }
                Expr::Choice(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    Expr::Choice(mapped_lhs, mapped_rhs)
                }
                Expr::Rep(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Rep(mapped)
                }
                Expr::RepOnce(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::RepOnce(mapped)
                }
                Expr::RepMinMax(expr, min, max) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::RepMinMax(mapped, min, max)
                }
                Expr::Opt(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Opt(mapped)
                }
                Expr::Push(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    Expr::Push(mapped)
                }
                expr => expr
            };

            f(mapped)
        }

        map_internal(self, &mut f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let expr = Expr::Choice(
            Box::new(Expr::Seq(
                Box::new(Expr::Ident(Ident::new("a"))),
                Box::new(Expr::Str("b".to_owned()))
            )),
            Box::new(Expr::PosPred(
                Box::new(Expr::NegPred(
                    Box::new(Expr::Rep(
                        Box::new(Expr::RepOnce(
                            Box::new(Expr::Opt(
                                Box::new(Expr::Choice(
                                    Box::new(Expr::Insens("c".to_owned())),
                                    Box::new(Expr::Push(
                                        Box::new(Expr::Range("'d'".to_owned(), "'e'".to_owned()))
                                    ))
                                ))
                            ))
                        ))
                    ))
                ))
            ))
        );

        assert_eq!(expr.clone().map_bottom_up(|expr| expr).map_top_down(|expr| expr), expr);
    }
}
