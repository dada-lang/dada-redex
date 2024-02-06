use formality_core::test;
use formality_core::test_util::ResultTestExt;

use crate::{dada_lang::term, type_system::check_program};

/// Check sharing a field from a leased value errs.
#[test]
#[allow(non_snake_case)]
fn share_field_of_leased_value() {
    check_program(
        &term(
            "
                class Data { }

                class Foo {
                    i: Data;
                }

                class Main {
                    fn main(my self) {
                        let foo = new Foo(new Data());
                        let bar = foo.lease;
                        let i = foo.i.share;
                        bar.give;
                        ();
                    }
                }
            ",
        ),
    ).assert_err(
        expect_test::expect![[r#"
            check program `class Data { } class Foo { i : Data ; } class Main { fn main (my self) -> () { let foo = new Foo (new Data ()) ; let bar = foo . lease ; let i = foo . i . share ; bar . give ; () ; } }`

            Caused by:
                0: check class named `Main`
                1: check method named `main`
                2: check function body
                3: judgment `can_type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . lease ; let i = foo . i . share ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . lease ; let i = foo . i . share ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_expr { expr: { let foo = new Foo (new Data ()) ; let bar = foo . lease ; let i = foo . i . share ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_block { block: { let foo = new Foo (new Data ()) ; let bar = foo . lease ; let i = foo . i . share ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let foo = new Foo (new Data ()) ;, let bar = foo . lease ;, let i = foo . i . share ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let bar = foo . lease ;, let i = foo . i . share ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let i = foo . i . share ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let i = foo . i . share ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr { expr: foo . i . share, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                     the rule "share|lease place" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `access_permitted { access: share, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                         the rule "access_permitted" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `env_permits_access { access: share, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                             the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `parameters_permit_access { parameters: [leased {foo} Foo], access: share, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                 the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                   judgment `parameter_permits_access { parameter: leased {foo} Foo, access: share, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                     the rule "ty" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `terms_permit_access { terms: Terms { unique: true, shared: false, leased: true, vars: {}, named_tys: {(Terms { unique: true, shared: false, leased: true, vars: {}, named_tys: {}, shared_places: {}, leased_places: {foo} }, Foo)}, shared_places: {}, leased_places: {foo} }, access: share, accessed_place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: leased {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                         the rule "terms" failed at step #2 (src/file.rs:LL:CC) because
                                                                           judgment `leased_place_permits_access { leased_place: foo, access: share, accessed_place: foo . i }` failed at the following rule(s):
                                                                             the rule "lease-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `place_disjoint_from(&accessed_place, &leased_place)`
                                                                                 &accessed_place = foo . i
                                                                                 &leased_place = foo"#]],
    )
}

/// Check sharing a field from a shared value is ok.
#[test]
#[allow(non_snake_case)]
fn share_field_of_shared_value() {
    check_program(&term(
        "
            class Data { }

            class Foo {
                i: Data;
            }

            class Main {
                fn main(my self) {
                    let foo = new Foo(new Data());
                    let bar = foo.share;
                    let i = foo.i.share;
                    bar.give;
                    ();
                }
            }
        ",
    ))
    .assert_ok(expect_test::expect!["()"])
}

/// Check leasing a field from a shared value is not ok.
#[test]
#[allow(non_snake_case)]
fn lease_field_of_shared_value() {
    check_program(
        &term(
            "
            class Data { }

            class Foo {
                i: Data;
            }

            class Main {
                fn main(my self) {
                    let foo = new Foo(new Data());
                    let bar = foo.share;
                    let i = foo.i.lease;
                    bar.give;
                    ();
                }
            }
        ",
        ),
    ).assert_err(
        expect_test::expect![[r#"
            check program `class Data { } class Foo { i : Data ; } class Main { fn main (my self) -> () { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . lease ; bar . give ; () ; } }`

            Caused by:
                0: check class named `Main`
                1: check method named `main`
                2: check function body
                3: judgment `can_type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . lease ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . lease ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_expr { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . lease ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_block { block: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . lease ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let foo = new Foo (new Data ()) ;, let bar = foo . share ;, let i = foo . i . lease ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let bar = foo . share ;, let i = foo . i . lease ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let i = foo . i . lease ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let i = foo . i . lease ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr { expr: foo . i . lease, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                     the rule "share|lease place" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `access_permitted { access: lease, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                         the rule "access_permitted" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `env_permits_access { access: lease, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                             the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `parameters_permit_access { parameters: [shared {foo} Foo], access: lease, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                 the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                   judgment `parameter_permits_access { parameter: shared {foo} Foo, access: lease, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                     the rule "ty" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `terms_permit_access { terms: Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {(Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {}, shared_places: {foo}, leased_places: {} }, Foo)}, shared_places: {foo}, leased_places: {} }, access: lease, accessed_place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                         the rule "terms" failed at step #1 (src/file.rs:LL:CC) because
                                                                           judgment `shared_place_permits_access { shared_place: foo, access: lease, accessed_place: foo . i }` failed at the following rule(s):
                                                                             the rule "share-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `place_disjoint_from(&accessed_place, &shared_place)`
                                                                                 &accessed_place = foo . i
                                                                                 &shared_place = foo"#]],
    )
}

/// Check giving a field from a shared value is not ok.
#[test]
#[allow(non_snake_case)]
fn give_field_of_shared_value() {
    check_program(
        &term(
            "
            class Data { }

            class Foo {
                i: Data;
            }

            class Main {
                fn main(my self) {
                    let foo = new Foo(new Data());
                    let bar = foo.share;
                    let i = foo.i.give;
                    bar.give;
                    ();
                }
            }
        ",
        ),
    ).assert_err(
        expect_test::expect![[r#"
            check program `class Data { } class Foo { i : Data ; } class Main { fn main (my self) -> () { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . give ; bar . give ; () ; } }`

            Caused by:
                0: check class named `Main`
                1: check method named `main`
                2: check function body
                3: judgment `can_type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . give ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr_as { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . give ; bar . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_expr { expr: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . give ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_block { block: { let foo = new Foo (new Data ()) ; let bar = foo . share ; let i = foo . i . give ; bar . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let foo = new Foo (new Data ()) ;, let bar = foo . share ;, let i = foo . i . give ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let bar = foo . share ;, let i = foo . i . give ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let i = foo . i . give ;, bar . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let i = foo . i . give ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr { expr: foo . i . give, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                     the rule "give place" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `access_permitted { access: give, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                         the rule "access_permitted" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `env_permits_access { access: give, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {bar} } }` failed at the following rule(s):
                                                             the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `parameters_permit_access { parameters: [shared {foo} Foo], access: give, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                 the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                   judgment `parameter_permits_access { parameter: shared {foo} Foo, access: give, place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                     the rule "ty" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `terms_permit_access { terms: Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {(Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {}, shared_places: {foo}, leased_places: {} }, Foo)}, shared_places: {foo}, leased_places: {} }, access: give, accessed_place: foo . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, bar: shared {foo} Foo, foo: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                         the rule "terms" failed at step #1 (src/file.rs:LL:CC) because
                                                                           judgment `shared_place_permits_access { shared_place: foo, access: give, accessed_place: foo . i }` failed at the following rule(s):
                                                                             the rule "share-give" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `place_disjoint_from_or_prefix_of(&accessed_place, &shared_place)`
                                                                                 &accessed_place = foo . i
                                                                                 &shared_place = foo"#]],
    )
}

/// Check sharing a field from a leased value errs.
#[test]
#[allow(non_snake_case)]
fn share_field_of_leased_value_after_explicit_give() {
    check_program(&term(
        "
                class Data { }

                class Foo {
                    i: Data;
                }

                class Main {
                    fn main(my self) {
                        let foo = new Foo(new Data());
                        let bar = foo.lease;
                        bar.give;
                        let i = foo.i.share;
                        ();
                    }
                }
            ",
    ))
    .assert_ok(expect_test::expect!["()"])
}

/// Check that we can permit accessing `foo.i` even though
/// it was leased since `bar` is dead.
#[test]
#[allow(non_snake_case)]
fn share_field_of_leased_value_without_explicit_give() {
    check_program(&term(
        "
                class Data { }

                class Foo {
                    i: Data;
                }

                class Main {
                    fn main(my self) {
                        let foo = new Foo(new Data());
                        let bar = foo.lease;
                        let i = foo.i.share;
                        ();
                    }
                }
            ",
    ))
    .assert_ok(expect_test::expect!["()"])
}

#[test]
fn share_field_of_leased_value_but_lease_variable_is_dead() {
    // Here, the variable `q` is dead, but its restrictions must
    // still be enforced because `r` is live.
    check_program(&term(
        "
                class Data { }

                class Foo {
                    i: Data;
                }

                class Main {
                    fn main(my self) {
                        let p = new Foo(new Data());
                        let q = p.lease;
                        let r = q.share;
                        let i = p.i.share;
                        r.give;
                        ();
                    }
                }
            ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Foo { i : Data ; } class Main { fn main (my self) -> () { let p = new Foo (new Data ()) ; let q = p . lease ; let r = q . share ; let i = p . i . share ; r . give ; () ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `main`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let p = new Foo (new Data ()) ; let q = p . lease ; let r = q . share ; let i = p . i . share ; r . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let p = new Foo (new Data ()) ; let q = p . lease ; let r = q . share ; let i = p . i . share ; r . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let p = new Foo (new Data ()) ; let q = p . lease ; let r = q . share ; let i = p . i . share ; r . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let p = new Foo (new Data ()) ; let q = p . lease ; let r = q . share ; let i = p . i . share ; r . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let p = new Foo (new Data ()) ;, let q = p . lease ;, let r = q . share ;, let i = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let q = p . lease ;, let r = q . share ;, let i = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let r = q . share ;, let i = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                           judgment `type_statements_with_final_ty { statements: [let i = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                             the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `type_statement { statement: let i = p . i . share ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {r} } }` failed at the following rule(s):
                                                 the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                                   judgment `type_expr { expr: p . i . share, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {r} } }` failed at the following rule(s):
                                                     the rule "share|lease place" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `access_permitted { access: share, place: p . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {r} } }` failed at the following rule(s):
                                                         the rule "access_permitted" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `env_permits_access { access: share, place: p . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {r} } }` failed at the following rule(s):
                                                             the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `parameters_permit_access { parameters: [shared {q} leased {p} Foo], access: share, place: p . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                 the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                   judgment `parameter_permits_access { parameter: shared {q} leased {p} Foo, access: share, place: p . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                     the rule "ty" failed at step #1 (src/file.rs:LL:CC) because
                                                                       judgment `terms_permit_access { terms: Terms { unique: false, shared: true, leased: true, vars: {}, named_tys: {(Terms { unique: false, shared: true, leased: true, vars: {}, named_tys: {}, shared_places: {q}, leased_places: {p} }, Foo)}, shared_places: {q}, leased_places: {p} }, access: share, accessed_place: p . i, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: Foo, q: leased {p} Foo, r: shared {q} leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                                         the rule "terms" failed at step #2 (src/file.rs:LL:CC) because
                                                                           judgment `leased_place_permits_access { leased_place: p, access: share, accessed_place: p . i }` failed at the following rule(s):
                                                                             the rule "lease-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `place_disjoint_from(&accessed_place, &leased_place)`
                                                                                 &accessed_place = p . i
                                                                                 &leased_place = p"#]])
}

#[test]
fn share_field_of_leased_value_but_lease_variable_is_dead_explicit_ty() {
    // Here, the variable `q` is dead, but its restrictions must
    // still be enforced because `r` is live.
    check_program(&term(
        "
                class Data { }

                class Foo {
                    i: Data;
                }

                class Main {
                    fn main(my self) {
                        let p: my Foo = new Foo(new Data());
                        let q: leased{p} Foo = p.lease;
                        let r: shared{q} Foo = q.share;
                        let i: shared{p.i} Int = p.i.share;
                        r.give;
                        ();
                    }
                }
            ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Foo { i : Data ; } class Main { fn main (my self) -> () { let p : my Foo = new Foo (new Data ()) ; let q : leased {p} Foo = p . lease ; let r : shared {q} Foo = q . share ; let i : shared {p . i} Int = p . i . share ; r . give ; () ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `main`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let p : my Foo = new Foo (new Data ()) ; let q : leased {p} Foo = p . lease ; let r : shared {q} Foo = q . share ; let i : shared {p . i} Int = p . i . share ; r . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let p : my Foo = new Foo (new Data ()) ; let q : leased {p} Foo = p . lease ; let r : shared {q} Foo = q . share ; let i : shared {p . i} Int = p . i . share ; r . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let p : my Foo = new Foo (new Data ()) ; let q : leased {p} Foo = p . lease ; let r : shared {q} Foo = q . share ; let i : shared {p . i} Int = p . i . share ; r . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let p : my Foo = new Foo (new Data ()) ; let q : leased {p} Foo = p . lease ; let r : shared {q} Foo = q . share ; let i : shared {p . i} Int = p . i . share ; r . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let p : my Foo = new Foo (new Data ()) ;, let q : leased {p} Foo = p . lease ;, let r : shared {q} Foo = q . share ;, let i : shared {p . i} Int = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let q : leased {p} Foo = p . lease ;, let r : shared {q} Foo = q . share ;, let i : shared {p . i} Int = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [let r : shared {q} Foo = q . share ;, let i : shared {p . i} Int = p . i . share ;, r . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: let r : shared {q} Foo = q . share ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {p, r} } }` failed at the following rule(s):
                                             the rule "let" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr_as { expr: q . share, as_ty: shared {q} Foo, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {p} } }` failed at the following rule(s):
                                                 the rule "type_expr_as" failed at step #1 (src/file.rs:LL:CC) because
                                                   judgment `sub { a: shared {q} leased {p} Foo, b: shared {q} Foo, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                     the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `sub_in { terms_a: Terms { unique: true, shared: false, leased: false, vars: {}, named_tys: {}, shared_places: {}, leased_places: {} }, a: shared {q} leased {p} Foo, terms_b: Terms { unique: true, shared: false, leased: false, vars: {}, named_tys: {}, shared_places: {}, leased_places: {} }, b: shared {q} Foo, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                         the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                           judgment `sub_terms { terms_a: Terms { unique: false, shared: true, leased: true, vars: {}, named_tys: {(Terms { unique: false, shared: true, leased: true, vars: {}, named_tys: {}, shared_places: {q}, leased_places: {p} }, Foo)}, shared_places: {q}, leased_places: {p} }, terms_b: Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {(Terms { unique: false, shared: true, leased: false, vars: {}, named_tys: {}, shared_places: {q}, leased_places: {p} }, Foo)}, shared_places: {q}, leased_places: {p} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, p: my Foo, q: leased {p} Foo}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                             the rule "sub_teams" failed at step #1 (src/file.rs:LL:CC) because
                                                               condition evaluted to false: `terms_a.leased <= terms_b.leased`"#]])
}

/// Test where we expect data leased from self and then try to use self.
/// Error.
#[test]
#[allow(non_snake_case)]
fn pair_method__leased_self__use_self() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;

                fn method(my self, data: leased{self} Data) {
                  self.a.lease;
                  data.give;
                  ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Pair { a : Data ; b : Data ; fn method (my self data : leased {self} Data) -> () { self . a . lease ; data . give ; () ; } }`

        Caused by:
            0: check class named `Pair`
            1: check method named `method`
            2: check function body
            3: judgment `can_type_expr_as { expr: { self . a . lease ; data . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { self . a . lease ; data . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { self . a . lease ; data . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { self . a . lease ; data . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [self . a . lease ;, data . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                   judgment `type_statement { statement: self . a . lease ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {data} } }` failed at the following rule(s):
                                     the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                       judgment `type_expr { expr: self . a . lease, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {data} } }` failed at the following rule(s):
                                         the rule "share|lease place" failed at step #0 (src/file.rs:LL:CC) because
                                           judgment `access_permitted { access: lease, place: self . a, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {data} } }` failed at the following rule(s):
                                             the rule "access_permitted" failed at step #1 (src/file.rs:LL:CC) because
                                               judgment `env_permits_access { access: lease, place: self . a, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} }, live_after: LiveVars { vars: {data} } }` failed at the following rule(s):
                                                 the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                   judgment `parameters_permit_access { parameters: [leased {self} Data], access: lease, place: self . a, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                     the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `parameter_permits_access { parameter: leased {self} Data, access: lease, place: self . a, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                         the rule "ty" failed at step #1 (src/file.rs:LL:CC) because
                                                           judgment `terms_permit_access { terms: Terms { unique: true, shared: false, leased: true, vars: {}, named_tys: {(Terms { unique: true, shared: false, leased: true, vars: {}, named_tys: {}, shared_places: {}, leased_places: {self} }, Data)}, shared_places: {}, leased_places: {self} }, access: lease, accessed_place: self . a, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Pair, data: leased {self} Data}, assumptions: {}, fresh: 0 }, flow: Flow { moved_places: {} } }` failed at the following rule(s):
                                                             the rule "terms" failed at step #2 (src/file.rs:LL:CC) because
                                                               judgment `leased_place_permits_access { leased_place: self, access: lease, accessed_place: self . a }` failed at the following rule(s):
                                                                 the rule "lease-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                   condition evaluted to false: `place_disjoint_from(&accessed_place, &leased_place)`
                                                                     &accessed_place = self . a
                                                                     &leased_place = self"#]])
}
