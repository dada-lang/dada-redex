use std::sync::Arc;

use fn_error_context::context;
use formality_core::Fallible;

use crate::grammar::{Decl, Program};

mod accesses;
mod blocks;
mod classes;
mod env;
mod expressions;
mod flow;
mod in_flight;
mod is_;
mod liens;
mod liveness;
mod methods;
mod places;
mod predicates;
mod statements;
mod subtypes;
mod types;

#[cfg(test)]
mod tests;

mod quantifiers;

#[context("check program `{program:?}`")]
pub fn check_program(program: &Arc<Program>) -> Fallible<()> {
    for decl in &program.decls {
        check_decl(program, decl)?;
    }
    Ok(())
}

fn check_decl(program: &Arc<Program>, decl: &Decl) -> Fallible<()> {
    match decl {
        Decl::ClassDecl(class_decl) => classes::check_class(program, class_decl),
    }
}
