//! # Liveness and cancellation
//!
//! When variables are dead, subtyping allows for *cancellation*, so e.g. if `d1` is dead,
//! then `shared[d1] leased[d2] Foo` is a subtype of `leased[d2] Foo`. Cancellation only
//! applies when we have a shared/leased permission applies to a leased permission.
//!
//! Consideration to test:
//!
//! * C1. Cancellation can remove "relative" permissions like `shared` and `leased`, but not owned permissions
//!   like `my` or `our` nor generic permissions (since in that case we do not know which variables they
//!   may refer to)
//! * C2. Cancellation can only occur if all variables in the permission are dead: so `shared[d1, d2]` can only
//!   be canceled if `d1` and `d2` are both dead.
//! * C3. Cancellation cannot convert a shared permission into a leased permission.
//! * C4. Subtyping must account for future cancellation. So e.g., `leased[d1, d2] Foo` cannot be a subtype of
//!   `leased[d1] leased[d2] Foo` since, if `d1` later goes dead, the supertype could be upcast
//!   to `leased[d2] Foo` but the subtype could not. That would be unsound.

use crate::{dada_lang::term, type_system::check_program};
use formality_core::{test, test_util::ResultTestExt};

// C1. Cancellation can remove "relative" permissions like `shared` and `leased`.

#[test]
fn c1_remove_relative_shared() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: shared[m] Data = m.share;
                let q: shared[p] shared[m] Data = p.share;
                let r: shared[m] Data = q.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c1_remove_relative_leased() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: leased[m] Data = m.lease;
                let q: leased[p] leased[m] Data = p.lease;
                let r: leased[m] Data = q.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

// C1. Cancellation and `my` permission are not very relevant.
//
// The `my my` type here is equivalent to `my` so this just becomes
// ownership transfer.

#[test]
fn c1_remove_my() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: my my Data = m.give;
                let q: my Data = p.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

// C1. Cancellation cannot remove owned permissions `our`.

#[test]
fn c1_remove_our() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: our my Data = m.give;
                let q: my Data = p.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : our my Data = m . give ; let q : my Data = p . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : our my Data = m . give ; let q : my Data = p . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : our my Data = m . give ; let q : my Data = p . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let m : my Data = new Data () ; let p : our my Data = m . give ; let q : my Data = p . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let m : my Data = new Data () ; let p : our my Data = m . give ; let q : my Data = p . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let m : my Data = new Data () ;, let p : our my Data = m . give ;, let q : my Data = p . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let p : our my Data = m . give ;, let q : my Data = p . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let q : my Data = p . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: let q : my Data = p . give ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr_as { expr: p . give, as_ty: my Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                                   judgment `sub { a: our my Data, b: my Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                     the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: our my Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: my Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                         the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                           judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                                   judgment `sub_perms { perms_a: RedPerms { copied: true, shared_from: {}, leased_from: {}, variables: {} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: our my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                     the rule "sub-some" failed at step #0 (src/file.rs:LL:CC) because
                                                                       condition evaluted to false: `perms_a.is_copy(&env).implies(perms_b.is_copy(&env))`"#]]);
}

// C1. Cancellation cannot remove generic permissions `our`.

#[test]
fn c1_remove_generic_permissions() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self, p: P my Data) {
                let q: my Data = p.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self p : ^perm0_0 my Data) -> () { let q : my Data = p . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let q : my Data = p . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let q : my Data = p . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let q : my Data = p . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let q : my Data = p . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let q : my Data = p . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                   judgment `type_statement { statement: let q : my Data = p . give ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                       judgment `type_expr_as { expr: p . give, as_ty: my Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `sub { a: !perm_0 my Data, b: my Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                             the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: !perm_0 my Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: my Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                 the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                   judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {!perm_0} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                     the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                       judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {!perm_0} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                         the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                           judgment `sub_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {!perm_0} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, p: !perm_0 my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub-some" failed at step #2 (src/file.rs:LL:CC) because
                                                               condition evaluted to false: `perms_a.layout(&env) == perms_b.layout(&env)`"#]]);
}

// C2. Cancellation can only occur if all variables in the permission are dead.

#[test]
fn c2_shared_shared_one_of_one_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: shared[m] Data = m.share;
                let q: shared[p] shared[m] Data = p.share;
                let r: shared[m] Data = q.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c2_shared_shared_two_of_two_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: shared[m] Data = m.share;
                let q: shared[m] Data = m.share;
                let r: shared[p, q] shared[m] Data = p.share;
                let s: shared[m] Data = r.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c2_shared_shared_one_of_two_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: shared[m] Data = m.share;
                let q: shared[m] Data = m.share;
                let r: shared[p, q] shared[m] Data = p.share;
                let s: shared[m] Data = r.give;
                q.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                       judgment `sub { a: shared [m] Data, b: (), live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                         the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: shared [m] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: (), live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                             the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                               judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(()) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                 the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                   judgment had no applicable rules: `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(()) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }`"#]]);
}

#[test]
fn c2_leased_leased_one_of_one_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: leased[m] Data = m.lease;
                let q: leased[p] leased[m] Data = p.lease;
                let r: leased[m] Data = q.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c2_leased_leased_two_of_two_variables_dead() {
    check_program(&term(
        "
        class Data {}
        class Pair {
            a: my Data;
            b: my Data;
        }
        class Main {
            fn test[perm P](my self) {
                let m: my Pair = new Pair(new Data(), new Data());
                let p: leased[m.a] Data = m.a.lease;
                let q: leased[m.b] Data = m.b.lease;
                let r: leased[p, q] leased[m] Data = p.lease;
                let s: leased[m] Data = r.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c2_leased_leased_one_of_two_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: leased[m] Data = m.lease;
                let q: leased[m] Data = m.lease;
                let r: leased[p, q] leased[m] Data = p.lease;
                let s: leased[m] Data = r.give;
                q.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : leased [m] Data = m . lease ; let r : leased [p, q] leased [m] Data = p . lease ; let s : leased [m] Data = r . give ; q . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : leased [m] Data = m . lease ; let r : leased [p, q] leased [m] Data = p . lease ; let s : leased [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : leased [m] Data = m . lease ; let r : leased [p, q] leased [m] Data = p . lease ; let s : leased [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : leased [m] Data = m . lease ; let r : leased [p, q] leased [m] Data = p . lease ; let s : leased [m] Data = r . give ; q . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : leased [m] Data = m . lease ; let r : leased [p, q] leased [m] Data = p . lease ; let s : leased [m] Data = r . give ; q . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let m : my Data = new Data () ;, let p : leased [m] Data = m . lease ;, let q : leased [m] Data = m . lease ;, let r : leased [p, q] leased [m] Data = p . lease ;, let s : leased [m] Data = r . give ;, q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let p : leased [m] Data = m . lease ;, let q : leased [m] Data = m . lease ;, let r : leased [p, q] leased [m] Data = p . lease ;, let s : leased [m] Data = r . give ;, q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let q : leased [m] Data = m . lease ;, let r : leased [p, q] leased [m] Data = p . lease ;, let s : leased [m] Data = r . give ;, q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: let q : leased [m] Data = m . lease ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {p, q}, traversed: {} } }` failed at the following rule(s):
                                             the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr_as { expr: m . lease, as_ty: leased [m] Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {p}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr { expr: m . lease, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {p}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "share|lease place" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `access_permitted { access: lease, place: m, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {p}, traversed: {} } }` failed at the following rule(s):
                                                         the rule "access_permitted" failed at step #0 (src/file.rs:LL:CC) because
                                                           judgment `env_permits_access { access: lease, place: m, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {p}, traversed: {} } }` failed at the following rule(s):
                                                             the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `parameters_permit_access { parameters: [leased [m] Data], access: lease, place: m, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                   judgment `parameter_permits_access { parameter: leased [m] Data, access: lease, place: m, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                     the rule "parameter" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `lien_permit_access { lien: leased(m), access: lease, accessed_place: m, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                         the rule "leased" failed at step #0 (src/file.rs:LL:CC) because
                                                                           judgment `leased_place_permits_access { leased_place: m, access: lease, accessed_place: m }` failed at the following rule(s):
                                                                             the rule "lease-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `place_disjoint_from(&accessed_place, &leased_place)`
                                                                                 &accessed_place = m
                                                                                 &leased_place = m"#]]);
}

// C3. Cancellation cannot convert a shared permission into a leased permission.

#[test]
fn c3_shared_leased_one_of_one_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: leased[m] Data = m.lease;
                let q: shared[p] leased[m] Data = p.share;
                let r: leased[m] Data = q.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : shared [p] leased [m] Data = p . share ; let r : leased [m] Data = q . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : shared [p] leased [m] Data = p . share ; let r : leased [m] Data = q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : shared [p] leased [m] Data = p . share ; let r : leased [m] Data = q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : shared [p] leased [m] Data = p . share ; let r : leased [m] Data = q . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let m : my Data = new Data () ; let p : leased [m] Data = m . lease ; let q : shared [p] leased [m] Data = p . share ; let r : leased [m] Data = q . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let m : my Data = new Data () ;, let p : leased [m] Data = m . lease ;, let q : shared [p] leased [m] Data = p . share ;, let r : leased [m] Data = q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let p : leased [m] Data = m . lease ;, let q : shared [p] leased [m] Data = p . share ;, let r : leased [m] Data = q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let q : shared [p] leased [m] Data = p . share ;, let r : leased [m] Data = q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let r : leased [m] Data = q . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let r : leased [m] Data = q . give ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr_as { expr: q . give, as_ty: leased [m] Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                                       judgment `sub { a: shared [p] leased [m] Data, b: leased [m] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                         the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                           judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: shared [p] leased [m] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: leased [m] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                               judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {p}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                                   judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {p}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                     the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                                       judgment `sub_perms { perms_a: RedPerms { copied: true, shared_from: {p}, leased_from: {m}, variables: {} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: leased [m] Data, q: shared [p] leased [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                         the rule "sub-some" failed at step #0 (src/file.rs:LL:CC) because
                                                                           condition evaluted to false: `perms_a.is_copy(&env).implies(perms_b.is_copy(&env))`"#]]);
}

#[test]
fn c3_shared_leased_two_of_two_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: leased[m] Data = m.share;
                let q: leased[m] Data = m.share;
                let r: shared[p, q] leased[m] Data = p.share;
                let s: shared[m] Data = r.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : leased [m] Data = m . share ; let q : leased [m] Data = m . share ; let r : shared [p, q] leased [m] Data = p . share ; let s : shared [m] Data = r . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . share ; let q : leased [m] Data = m . share ; let r : shared [p, q] leased [m] Data = p . share ; let s : shared [m] Data = r . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . share ; let q : leased [m] Data = m . share ; let r : shared [p, q] leased [m] Data = p . share ; let s : shared [m] Data = r . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let m : my Data = new Data () ; let p : leased [m] Data = m . share ; let q : leased [m] Data = m . share ; let r : shared [p, q] leased [m] Data = p . share ; let s : shared [m] Data = r . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let m : my Data = new Data () ; let p : leased [m] Data = m . share ; let q : leased [m] Data = m . share ; let r : shared [p, q] leased [m] Data = p . share ; let s : shared [m] Data = r . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let m : my Data = new Data () ;, let p : leased [m] Data = m . share ;, let q : leased [m] Data = m . share ;, let r : shared [p, q] leased [m] Data = p . share ;, let s : shared [m] Data = r . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let p : leased [m] Data = m . share ;, let q : leased [m] Data = m . share ;, let r : shared [p, q] leased [m] Data = p . share ;, let s : shared [m] Data = r . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                       judgment `type_statement { statement: let p : leased [m] Data = m . share ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {m, p}, traversed: {} } }` failed at the following rule(s):
                                         the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                           judgment `type_expr_as { expr: m . share, as_ty: leased [m] Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {m}, traversed: {} } }` failed at the following rule(s):
                                             the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `sub { a: shared [m] my Data, b: leased [m] Data, live_after: LivePlaces { accessed: {m}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                 the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: shared [m] my Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: leased [m] Data, live_after: LivePlaces { accessed: {m}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                     the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                       judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {m}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                         the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {m}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                               judgment `sub_perms { perms_a: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {m}, variables: {} }, live_after: LivePlaces { accessed: {m}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "sub-some" failed at step #0 (src/file.rs:LL:CC) because
                                                                   condition evaluted to false: `perms_a.is_copy(&env).implies(perms_b.is_copy(&env))`"#]]);
}

#[test]
fn c3_shared_leased_one_of_two_variables_dead() {
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let m: my Data = new Data();
                let p: shared[m] Data = m.share;
                let q: shared[m] Data = m.share;
                let r: shared[p, q] shared[m] Data = p.share;
                let s: shared[m] Data = r.give;
                q.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let m : my Data = new Data () ; let p : shared [m] Data = m . share ; let q : shared [m] Data = m . share ; let r : shared [p, q] shared [m] Data = p . share ; let s : shared [m] Data = r . give ; q . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                       judgment `sub { a: shared [m] Data, b: (), live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                         the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: shared [m] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: (), live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                             the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                               judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(()) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                 the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                   judgment had no applicable rules: `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {m}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, ty: NamedTy(()) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, m: my Data, p: shared [m] Data, q: shared [m] Data, r: shared [@ fresh(0), p] shared [m] Data, s: shared [m] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }`"#]]);
}

// C4. Subtyping must account for future cancellation.

#[test]
fn c4_shared_d1d2d3_not_subtype_of_shared_d1_shared_d2d3() {
    // This is interesting. It fails because `shared[d1] shared[d2, d3]`
    // is equivalent to `shared[d2, d3]` and there is clearly no subtyping relation.
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let d1: my Data = new Data();
                let d2: my Data = new Data();
                let d3: my Data = new Data();
                let s1: shared[d1, d2, d3] Data = d1.share;
                let s2: shared[d1] shared[d2, d3] Data = s1.give;
            }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Main { fn test [perm] (my self) -> () { let d1 : my Data = new Data () ; let d2 : my Data = new Data () ; let d3 : my Data = new Data () ; let s1 : shared [d1, d2, d3] Data = d1 . share ; let s2 : shared [d1] shared [d2, d3] Data = s1 . give ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let d1 : my Data = new Data () ; let d2 : my Data = new Data () ; let d3 : my Data = new Data () ; let s1 : shared [d1, d2, d3] Data = d1 . share ; let s2 : shared [d1] shared [d2, d3] Data = s1 . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let d1 : my Data = new Data () ; let d2 : my Data = new Data () ; let d3 : my Data = new Data () ; let s1 : shared [d1, d2, d3] Data = d1 . share ; let s2 : shared [d1] shared [d2, d3] Data = s1 . give ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let d1 : my Data = new Data () ; let d2 : my Data = new Data () ; let d3 : my Data = new Data () ; let s1 : shared [d1, d2, d3] Data = d1 . share ; let s2 : shared [d1] shared [d2, d3] Data = s1 . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let d1 : my Data = new Data () ; let d2 : my Data = new Data () ; let d3 : my Data = new Data () ; let s1 : shared [d1, d2, d3] Data = d1 . share ; let s2 : shared [d1] shared [d2, d3] Data = s1 . give ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let d1 : my Data = new Data () ;, let d2 : my Data = new Data () ;, let d3 : my Data = new Data () ;, let s1 : shared [d1, d2, d3] Data = d1 . share ;, let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let d2 : my Data = new Data () ;, let d3 : my Data = new Data () ;, let s1 : shared [d1, d2, d3] Data = d1 . share ;, let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let d3 : my Data = new Data () ;, let s1 : shared [d1, d2, d3] Data = d1 . share ;, let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let s1 : shared [d1, d2, d3] Data = d1 . share ;, let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                               judgment `type_statements_with_final_ty { statements: [let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                                   judgment `type_statement { statement: let s2 : shared [d1] shared [d2, d3] Data = s1 . give ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `type_expr_as { expr: s1 . give, as_ty: shared [d1] shared [d2, d3] Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                         the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `sub { a: shared [d1, d2, d3] Data, b: shared [d1] shared [d2, d3] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                               judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: shared [d1, d2, d3] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: shared [d1] shared [d2, d3] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                                   judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {d1, d2, d3}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: true, shared_from: {d2, d3}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                     the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: true, shared_from: {d1, d2, d3}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: true, shared_from: {d2, d3}, leased_from: {}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                         the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                                           judgment `sub_perms { perms_a: RedPerms { copied: true, shared_from: {d1, d2, d3}, leased_from: {}, variables: {} }, perms_b: RedPerms { copied: true, shared_from: {d2, d3}, leased_from: {}, variables: {} }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                             the rule "sub-some" failed at step #3 (src/file.rs:LL:CC) because
                                                                               judgment `covered { place_a: d1, places_b: {d2, d3}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                                 the rule "dead" failed at step #3 (src/file.rs:LL:CC) because
                                                                                   condition evaluted to false: `perms_place.is_lent(&env)`
                                                                                     perms_place = RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }
                                                                                     &env = Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: my Data, d2: my Data, d3: my Data, s1: shared [d1, d2, d3] Data}, assumptions: {relative(!perm_0), atomic(!perm_0)}, fresh: 0 }
                                                                                 the rule "prefix" failed at step #0 (src/file.rs:LL:CC) because
                                                                                   condition evaluted to false: `places_b.iter().any(|place_b| place_b.is_prefix_of(&place_a))`"#]]);
}

#[test]
fn c4_leased_d1d2d3_subtype_of_leased_d1_leased_d2d3() {
    // This one succeeds because `leased[d1, d2, d3]` and `leased[d1] leased[d2, d3]` are
    // equivalent. Leased steps in the chain are unioned.
    check_program(&term(
        "
        class Data { }
        class Main {
            fn test[perm P](my self) {
                let d1: my Data = new Data();
                let d2: my Data = new Data();
                let d3: my Data = new Data();
                let s1: leased[d1, d2, d3] Data = d1.lease;
                let s2: leased[d1] leased[d2, d3] Data = s1.give;
            }
        }
        ",
    ))
    .assert_ok(expect_test::expect!["()"]);
}

#[test]
fn c4_leased_d1d2_leased_pair_not_subtype_of_leased_d2() {
    // This one fails because you after cancelling `d1` you don't get `d2`.
    check_program(&term(
        "
        class Pair {
            a: my Data;
            b: my Data;
        }
        class Data { }
        class Main {
            fn test[perm P](my self, pair: P Pair) where leased(P) {
                let d1: leased[pair.a] Data = pair.a.lease;
                let d2: leased[pair.b] Data = pair.b.lease;
                let s1: leased[d1, d2] Data = d1.lease;
                let s2: leased[d2] Data = s1.give;
                let _x = self.give.consume(pair.give, s2.give);
            }

            fn consume[perm P](my self, pair: P Pair, from_b: leased[pair.b] Data) where leased(P) { (); }
        }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Pair { a : my Data ; b : my Data ; } class Data { } class Main { fn test [perm] (my self pair : ^perm0_0 Pair) -> () where leased(^perm0_0) { let d1 : leased [pair . a] Data = pair . a . lease ; let d2 : leased [pair . b] Data = pair . b . lease ; let s1 : leased [d1, d2] Data = d1 . lease ; let s2 : leased [d2] Data = s1 . give ; let _x = self . give . consume (pair . give, s2 . give) ; } fn consume [perm] (my self pair : ^perm0_0 Pair, from_b : leased [pair . b] Data) -> () where leased(^perm0_0) { () ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `test`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let d1 : leased [pair . a] Data = pair . a . lease ; let d2 : leased [pair . b] Data = pair . b . lease ; let s1 : leased [d1, d2] Data = d1 . lease ; let s2 : leased [d2] Data = s1 . give ; let _x = self . give . consume (pair . give, s2 . give) ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let d1 : leased [pair . a] Data = pair . a . lease ; let d2 : leased [pair . b] Data = pair . b . lease ; let s1 : leased [d1, d2] Data = d1 . lease ; let s2 : leased [d2] Data = s1 . give ; let _x = self . give . consume (pair . give, s2 . give) ; }, as_ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let d1 : leased [pair . a] Data = pair . a . lease ; let d2 : leased [pair . b] Data = pair . b . lease ; let s1 : leased [d1, d2] Data = d1 . lease ; let s2 : leased [d2] Data = s1 . give ; let _x = self . give . consume (pair . give, s2 . give) ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let d1 : leased [pair . a] Data = pair . a . lease ; let d2 : leased [pair . b] Data = pair . b . lease ; let s1 : leased [d1, d2] Data = d1 . lease ; let s2 : leased [d2] Data = s1 . give ; let _x = self . give . consume (pair . give, s2 . give) ; }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let d1 : leased [pair . a] Data = pair . a . lease ;, let d2 : leased [pair . b] Data = pair . b . lease ;, let s1 : leased [d1, d2] Data = d1 . lease ;, let s2 : leased [d2] Data = s1 . give ;, let _x = self . give . consume (pair . give, s2 . give) ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let d2 : leased [pair . b] Data = pair . b . lease ;, let s1 : leased [d1, d2] Data = d1 . lease ;, let s2 : leased [d2] Data = s1 . give ;, let _x = self . give . consume (pair . give, s2 . give) ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let s1 : leased [d1, d2] Data = d1 . lease ;, let s2 : leased [d2] Data = s1 . give ;, let _x = self . give . consume (pair . give, s2 . give) ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let s2 : leased [d2] Data = s1 . give ;, let _x = self . give . consume (pair . give, s2 . give) ;], ty: (), env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let s2 : leased [d2] Data = s1 . give ;, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {self, pair, s2}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr_as { expr: s1 . give, as_ty: leased [d2] Data, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 }, live_after: LivePlaces { accessed: {self, pair}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                                       judgment `sub { a: leased [d1, d2] Data, b: leased [d2] Data, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                         the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                           judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: leased [d1, d2] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: leased [d2] Data, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                             the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                               judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {d1, d2, pair . a, pair . b}, variables: {!perm_0} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {d2, pair . b}, variables: {!perm_0} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                 the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                                   judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {d1, d2, pair . a, pair . b}, variables: {!perm_0} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {d2, pair . b}, variables: {!perm_0} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                     the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                                       judgment `sub_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {d1, d2, pair . a, pair . b}, variables: {!perm_0} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {d2, pair . b}, variables: {!perm_0} }, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                         the rule "sub-some" failed at step #4 (src/file.rs:LL:CC) because
                                                                           judgment `"flat_map"` failed at the following rule(s):
                                                                             failed at (src/file.rs:LL:CC) because
                                                                               judgment `"flat_map"` failed at the following rule(s):
                                                                                 failed at (src/file.rs:LL:CC) because
                                                                                   judgment `covered { place_a: pair . a, places_b: {d2, pair . b}, live_after: LivePlaces { accessed: {self, pair}, traversed: {} }, env: Env { program: "...", universe: universe(1), in_scope_vars: [!perm_0], local_variables: {self: my Main, d1: leased [pair . a] Data, d2: leased [pair . b] Data, pair: !perm_0 Pair, s1: leased [d1, d2] Data}, assumptions: {leased(!perm_0), relative(!perm_0), atomic(!perm_0)}, fresh: 0 } }` failed at the following rule(s):
                                                                                     the rule "prefix" failed at step #0 (src/file.rs:LL:CC) because
                                                                                       condition evaluted to false: `places_b.iter().any(|place_b| place_b.is_prefix_of(&place_a))`"#]]);
}
