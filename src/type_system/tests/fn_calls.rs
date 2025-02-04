use crate::{dada_lang::term, type_system::check_program};
use formality_core::test;
use formality_core::test_util::ResultTestExt;

/// Check giving different messages in two fn calls works.
#[test]
#[allow(non_snake_case)]
fn send_two_different_messages() {
    check_program(&term(
        "
            class Bar {}

            class Channel[ty M] {
                fn send[perm P](P self, msg: M)
                where
                    leased(P),
                {
                }
            }

            class TheClass {
                fn empty_method(my self) {
                    let channel = new Channel[Bar]();

                    let bar1 = new Bar();
                    channel.lease.send[leased[channel]](bar1.give);

                    let bar2 = new Bar();
                    channel.lease.send[leased[channel]](bar2.give);

                    ();
                }
            }
        ",
    ))
    .assert_ok(expect_test::expect!["()"])
}

/// Check that giving same message twice in fn calls errors.
#[test]
#[allow(non_snake_case)]
fn send_same_message_twice() {
    check_program(&term(
        "
            class Bar {}

            class Channel[ty M] {
                fn send[perm P](P self, msg: M)
                where
                    leased(P),
                {
                }
            }

            class TheClass {
                fn empty_method(my self) {
                    let channel = new Channel[Bar]();
                    let bar = new Bar();
                    channel.lease.send[leased[channel]](bar.give);
                    channel.lease.send[leased[channel]](bar.give);
                    ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Bar { } class Channel [ty] { fn send [perm] (^perm0_0 self msg : ^ty1_0) -> () where leased(^perm0_0) { } } class TheClass { fn empty_method (my self) -> () { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . lease . send [leased [channel]] (bar . give) ; channel . lease . send [leased [channel]] (bar . give) ; () ; } }`

        Caused by:
            0: check class named `TheClass`
            1: check method named `empty_method`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . lease . send [leased [channel]] (bar . give) ; channel . lease . send [leased [channel]] (bar . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . lease . send [leased [channel]] (bar . give) ; channel . lease . send [leased [channel]] (bar . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . lease . send [leased [channel]] (bar . give) ; channel . lease . send [leased [channel]] (bar . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . lease . send [leased [channel]] (bar . give) ; channel . lease . send [leased [channel]] (bar . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let channel = new Channel [Bar] () ;, let bar = new Bar () ;, channel . lease . send [leased [channel]] (bar . give) ;, channel . lease . send [leased [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let bar = new Bar () ;, channel . lease . send [leased [channel]] (bar . give) ;, channel . lease . send [leased [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [channel . lease . send [leased [channel]] (bar . give) ;, channel . lease . send [leased [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: channel . lease . send [leased [channel]] (bar . give) ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {bar, channel}, traversed: {} } }` failed at the following rule(s):
                                             the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr { expr: channel . lease . send [leased [channel]] (bar . give), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {bar, channel}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "call" failed at step #8 (src/file.rs:LL:CC) because
                                                   judgment `type_method_arguments_as { exprs: [bar . give], input_names: [msg], input_tys: [Bar], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): leased [channel] Channel[Bar], bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 1 }, live_after: LivePlaces { accessed: {bar, channel}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                                       judgment `type_expr { expr: bar . give, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): leased [channel] Channel[Bar], bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 1 }, live_after: LivePlaces { accessed: {bar, channel}, traversed: {} } }` failed at the following rule(s):
                                                         the rule "give place" failed at step #2 (src/file.rs:LL:CC) because
                                                           judgment `give_place { place: bar, ty: Bar, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): leased [channel] Channel[Bar], bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 1 }, live_after: LivePlaces { accessed: {bar, channel}, traversed: {} } }` failed at the following rule(s):
                                                             the rule "copy" failed at step #1 (src/file.rs:LL:CC) because
                                                               judgment `reduces_to_copy { a: Bar, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): leased [channel] Channel[Bar], bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 1 } }` failed at the following rule(s):
                                                                 the rule "my" failed at step #1 (src/file.rs:LL:CC) because
                                                                   condition evaluted to false: `perms.is_copy(&env)`
                                                                     perms = RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }
                                                                     &env = Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): leased [channel] Channel[Bar], bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 1 }
                                                             the rule "move" failed at step #0 (src/file.rs:LL:CC) because
                                                               condition evaluted to false: `!live_after.is_live(&place)`
                                                                 live_after = LivePlaces { accessed: {bar, channel}, traversed: {} }
                                                                 &place = bar"#]])
}

/// Check that calling channel with a shared(self) when leased(self) is declared errors.
#[test]
#[allow(non_snake_case)]
fn needs_leased_got_shared_self() {
    check_program(&term(
        "
            class Bar {}

            class Channel[ty M] {
                fn send[perm P](P self, msg: M)
                where
                    leased(P),
                {
                }
            }

            class TheClass {
                fn empty_method(my self) {
                    let channel = new Channel[Bar]();
                    let bar = new Bar();
                    channel.share.send[shared[channel]](bar.give);
                    ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Bar { } class Channel [ty] { fn send [perm] (^perm0_0 self msg : ^ty1_0) -> () where leased(^perm0_0) { } } class TheClass { fn empty_method (my self) -> () { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . share . send [shared [channel]] (bar . give) ; () ; } }`

        Caused by:
            0: check class named `TheClass`
            1: check method named `empty_method`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . share . send [shared [channel]] (bar . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . share . send [shared [channel]] (bar . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . share . send [shared [channel]] (bar . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let channel = new Channel [Bar] () ; let bar = new Bar () ; channel . share . send [shared [channel]] (bar . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let channel = new Channel [Bar] () ;, let bar = new Bar () ;, channel . share . send [shared [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let bar = new Bar () ;, channel . share . send [shared [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [channel . share . send [shared [channel]] (bar . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: channel . share . send [shared [channel]] (bar . give) ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr { expr: channel . share . send [shared [channel]] (bar . give), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "call" failed at step #9 (src/file.rs:LL:CC) because
                                                   judgment `prove_predicates { predicate: [leased(shared [channel])], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): shared [channel] Channel[Bar], @ fresh(1): Bar, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                     the rule "prove_predicates" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `prove_predicate { predicate: leased(shared [channel]), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): shared [channel] Channel[Bar], @ fresh(1): Bar, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                         the rule "leased" failed at step #0 (src/file.rs:LL:CC) because
                                                           judgment `reduces_to_leased { a: shared [channel], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): shared [channel] Channel[Bar], @ fresh(1): Bar, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                             the rule "my" failed at step #1 (src/file.rs:LL:CC) because
                                                               condition evaluted to false: `perms.is_leased(&env)`
                                                                 perms = RedPerms { copied: true, shared_from: {channel}, leased_from: {}, variables: {} }
                                                                 &env = Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): shared [channel] Channel[Bar], @ fresh(1): Bar, bar: Bar, channel: Channel[Bar]}, assumptions: {}, fresh: 2 }"#]])
}

/// Test where function expects a `Pair` and data borrowed from `pair`.
/// We `give` data shared from `pair.a`.
/// OK.
#[test]
#[allow(non_snake_case)]
fn take_pair_and_data__give_pair_give_data_ok() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;
            }

            class TheClass {
                fn take_pair_and_data[perm P](P self, pair: my Pair, data: shared[pair] Data) {

                }

                fn empty_method(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.a.share;
                    self.give.take_pair_and_data[my](pair.give, data.give);
                    ();
                }
            }
        ",
    ))
    .assert_ok(expect_test::expect![["()"]])
}

/// Test where function expects a `Pair` and data borrowed from `pair`.
/// We `share` data shared from `pair.a`.
/// OK.
#[test]
#[allow(non_snake_case)]
fn take_pair_and_data__give_pair_share_data_ok() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;
            }

            class TheClass {
                fn take_pair_and_data[perm P](P self, pair: my Pair, data: shared[pair] Data) {

                }

                fn empty_method(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.a.share;
                    self.give.take_pair_and_data[my](pair.give, data.share);
                    ();
                }
            }
        ",
    ))
    .assert_ok(expect_test::expect![["()"]])
}

/// Test where function expects a `Pair` and data borrowed from `pair`.
/// We `share` data shared from `pair.a` but use it later.
/// Should error because `pair` has been moved.
#[test]
#[allow(non_snake_case)]
fn take_pair_and_data__give_pair_share_data_share_later() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;
            }

            class TheClass {
                fn take_pair_and_data[perm P](P self, pair: my Pair, data: shared[pair] Data) {

                }

                fn empty_method(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.a.share;
                    self.give.take_pair_and_data[my](pair.give, data.share);
                    data.share;
                    ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Pair { a : Data ; b : Data ; } class TheClass { fn take_pair_and_data [perm] (^perm0_0 self pair : my Pair, data : shared [pair] Data) -> () { } fn empty_method (my self) -> () { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . share) ; data . share ; () ; } }`

        Caused by:
            0: check class named `TheClass`
            1: check method named `empty_method`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . share) ; data . share ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . share) ; data . share ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . share) ; data . share ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . share) ; data . share ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let pair = new Pair (new Data (), new Data ()) ;, let data = pair . a . share ;, self . give . take_pair_and_data [my] (pair . give, data . share) ;, data . share ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let data = pair . a . share ;, self . give . take_pair_and_data [my] (pair . give, data . share) ;, data . share ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [self . give . take_pair_and_data [my] (pair . give, data . share) ;, data . share ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: self . give . take_pair_and_data [my] (pair . give, data . share) ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                             the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr { expr: self . give . take_pair_and_data [my] (pair . give, data . share), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "call" failed at step #10 (src/file.rs:LL:CC) because
                                                   judgment `accesses_permitted { access: drop, places: [@ fresh(0), @ fresh(1), @ fresh(2)], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "accesses_permitted" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `"flat_map"` failed at the following rule(s):
                                                         failed at (src/file.rs:LL:CC) because
                                                           judgment `access_permitted { access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                             the rule "access_permitted" failed at step #0 (src/file.rs:LL:CC) because
                                                               judgment `env_permits_access { access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                                 the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                                   judgment `parameters_permit_access { parameters: [shared [@ fresh(1) . a] Data], access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                     the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                       judgment `parameter_permits_access { parameter: shared [@ fresh(1) . a] Data, access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                         the rule "parameter" failed at step #1 (src/file.rs:LL:CC) because
                                                                           judgment `lien_permit_access { lien: shared(@ fresh(1) . a), access: drop, accessed_place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [data] shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                             the rule "shared" failed at step #0 (src/file.rs:LL:CC) because
                                                                               judgment `shared_place_permits_access { shared_place: @ fresh(1) . a, access: drop, accessed_place: @ fresh(1) }` failed at the following rule(s):
                                                                                 the rule "share-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                                   condition evaluted to false: `place_disjoint_from(&accessed_place, &shared_place)`
                                                                                     &accessed_place = @ fresh(1)
                                                                                     &shared_place = @ fresh(1) . a"#]])
}

/// Test where function expects a `Pair` and data borrowed from `pair`.
/// We `give` data shared from `pair.a` but use it later.
/// Should error because `pair` has been moved.
#[test]
#[allow(non_snake_case)]
fn take_pair_and_data__give_pair_give_data_give_later() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;
            }

            class TheClass {
                fn take_pair_and_data[perm P](P self, pair: my Pair, data: shared[pair] Data) {

                }

                fn empty_method(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.a.share;
                    self.give.take_pair_and_data[my](pair.give, data.give);
                    data.give;
                    ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Pair { a : Data ; b : Data ; } class TheClass { fn take_pair_and_data [perm] (^perm0_0 self pair : my Pair, data : shared [pair] Data) -> () { } fn empty_method (my self) -> () { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . give) ; data . give ; () ; } }`

        Caused by:
            0: check class named `TheClass`
            1: check method named `empty_method`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . give) ; data . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . give) ; data . give ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . give) ; data . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . a . share ; self . give . take_pair_and_data [my] (pair . give, data . give) ; data . give ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let pair = new Pair (new Data (), new Data ()) ;, let data = pair . a . share ;, self . give . take_pair_and_data [my] (pair . give, data . give) ;, data . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let data = pair . a . share ;, self . give . take_pair_and_data [my] (pair . give, data . give) ;, data . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [self . give . take_pair_and_data [my] (pair . give, data . give) ;, data . give ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: self . give . take_pair_and_data [my] (pair . give, data . give) ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                             the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr { expr: self . give . take_pair_and_data [my] (pair . give, data . give), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, data: shared [pair . a] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "call" failed at step #10 (src/file.rs:LL:CC) because
                                                   judgment `accesses_permitted { access: drop, places: [@ fresh(0), @ fresh(1), @ fresh(2)], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "accesses_permitted" failed at step #0 (src/file.rs:LL:CC) because
                                                       judgment `"flat_map"` failed at the following rule(s):
                                                         failed at (src/file.rs:LL:CC) because
                                                           judgment `access_permitted { access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                             the rule "access_permitted" failed at step #0 (src/file.rs:LL:CC) because
                                                               judgment `env_permits_access { access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 }, live_after: LivePlaces { accessed: {data}, traversed: {} } }` failed at the following rule(s):
                                                                 the rule "env_permits_access" failed at step #1 (src/file.rs:LL:CC) because
                                                                   judgment `parameters_permit_access { parameters: [shared [@ fresh(1) . a] Data], access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                     the rule "cons" failed at step #0 (src/file.rs:LL:CC) because
                                                                       judgment `parameter_permits_access { parameter: shared [@ fresh(1) . a] Data, access: drop, place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                         the rule "parameter" failed at step #1 (src/file.rs:LL:CC) because
                                                                           judgment `lien_permit_access { lien: shared(@ fresh(1) . a), access: drop, accessed_place: @ fresh(1), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my TheClass, @ fresh(0): my TheClass, @ fresh(1): Pair, @ fresh(2): shared [@ fresh(1) . a] Data, data: shared [@ fresh(1) . a] Data, pair: Pair}, assumptions: {}, fresh: 3 } }` failed at the following rule(s):
                                                                             the rule "shared" failed at step #0 (src/file.rs:LL:CC) because
                                                                               judgment `shared_place_permits_access { shared_place: @ fresh(1) . a, access: drop, accessed_place: @ fresh(1) }` failed at the following rule(s):
                                                                                 the rule "share-mutation" failed at step #0 (src/file.rs:LL:CC) because
                                                                                   condition evaluted to false: `place_disjoint_from(&accessed_place, &shared_place)`
                                                                                     &accessed_place = @ fresh(1)
                                                                                     &shared_place = @ fresh(1) . a"#]])
}

/// Test where we expect data leased from self (but do nothing with it).
/// OK.
#[test]
#[allow(non_snake_case)]
fn pair_method__leased_self_ok() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;

                fn method(my self, data: leased[self] Data) {
                  ();
                }
            }

            class Main {
                fn main(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.a.lease;
                    pair.give.method(data.give);
                    ();
                }
            }
        ",
    ))
    .assert_ok(expect_test::expect![["()"]])
}

/// Test where we expect data leased from self.a but get data from self.b.
/// Error.
#[test]
#[allow(non_snake_case)]
fn pair_method__expect_leased_self_a__got_leased_self_b() {
    check_program(&term(
        "
            class Data {}

            class Pair {
                a: Data;
                b: Data;

                fn method(my self, data: leased[self.a] Data) {
                  ();
                }
            }

            class Main {
                fn main(my self) {
                    let pair = new Pair(new Data(), new Data());
                    let data = pair.b.lease;
                    pair.give.method(data.give);
                    ();
                }
            }
        ",
    ))
    .assert_err(expect_test::expect![[r#"
        check program `class Data { } class Pair { a : Data ; b : Data ; fn method (my self data : leased [self . a] Data) -> () { () ; } } class Main { fn main (my self) -> () { let pair = new Pair (new Data (), new Data ()) ; let data = pair . b . lease ; pair . give . method (data . give) ; () ; } }`

        Caused by:
            0: check class named `Main`
            1: check method named `main`
            2: check function body
            3: judgment `can_type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . b . lease ; pair . give . method (data . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                 the rule "can_type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                   judgment `type_expr_as { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . b . lease ; pair . give . method (data . give) ; () ; }, as_ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                     the rule "type_expr_as" failed at step #0 (src/file.rs:LL:CC) because
                       judgment `type_expr { expr: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . b . lease ; pair . give . method (data . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                         the rule "block" failed at step #0 (src/file.rs:LL:CC) because
                           judgment `type_block { block: { let pair = new Pair (new Data (), new Data ()) ; let data = pair . b . lease ; pair . give . method (data . give) ; () ; }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                             the rule "place" failed at step #0 (src/file.rs:LL:CC) because
                               judgment `type_statements_with_final_ty { statements: [let pair = new Pair (new Data (), new Data ()) ;, let data = pair . b . lease ;, pair . give . method (data . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                 the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                   judgment `type_statements_with_final_ty { statements: [let data = pair . b . lease ;, pair . give . method (data . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                     the rule "cons" failed at step #2 (src/file.rs:LL:CC) because
                                       judgment `type_statements_with_final_ty { statements: [pair . give . method (data . give) ;, () ;], ty: (), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, data: leased [pair . b] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                         the rule "cons" failed at step #1 (src/file.rs:LL:CC) because
                                           judgment `type_statement { statement: pair . give . method (data . give) ;, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, data: leased [pair . b] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                             the rule "expr" failed at step #0 (src/file.rs:LL:CC) because
                                               judgment `type_expr { expr: pair . give . method (data . give), env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, data: leased [pair . b] Data, pair: Pair}, assumptions: {}, fresh: 0 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                 the rule "call" failed at step #8 (src/file.rs:LL:CC) because
                                                   judgment `type_method_arguments_as { exprs: [data . give], input_names: [data], input_tys: [leased [@ fresh(0) . a] Data], env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 1 }, live_after: LivePlaces { accessed: {}, traversed: {} } }` failed at the following rule(s):
                                                     the rule "cons" failed at step #5 (src/file.rs:LL:CC) because
                                                       judgment `sub { a: leased [@ fresh(0) . b] Data, b: leased [@ fresh(0) . a] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                         the rule "sub" failed at step #0 (src/file.rs:LL:CC) because
                                                           judgment `sub_under_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, a: leased [@ fresh(0) . b] Data, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }, b: leased [@ fresh(0) . a] Data, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                             the rule "sub" failed at step #2 (src/file.rs:LL:CC) because
                                                               judgment `sub_some { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . b}, variables: {} }, ty: NamedTy(Data) }, lien_datas_b: {RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . a}, variables: {} }, ty: NamedTy(Data) }}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                                 the rule "sub-some" failed at step #1 (src/file.rs:LL:CC) because
                                                                   judgment `sub_lien_data { lien_data_a: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . b}, variables: {} }, ty: NamedTy(Data) }, lien_data_b: RedTerm { perms: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . a}, variables: {} }, ty: NamedTy(Data) }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                                     the rule "sub-named" failed at step #3 (src/file.rs:LL:CC) because
                                                                       judgment `sub_perms { perms_a: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . b}, variables: {} }, perms_b: RedPerms { copied: false, shared_from: {}, leased_from: {@ fresh(0) . a}, variables: {} }, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                                         the rule "sub-some" failed at step #4 (src/file.rs:LL:CC) because
                                                                           judgment `covered { place_a: @ fresh(0) . b, places_b: {@ fresh(0) . a}, live_after: LivePlaces { accessed: {}, traversed: {} }, env: Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 } }` failed at the following rule(s):
                                                                             the rule "dead" failed at step #3 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `perms_place.is_lent(&env)`
                                                                                 perms_place = RedPerms { copied: false, shared_from: {}, leased_from: {}, variables: {} }
                                                                                 &env = Env { program: "...", universe: universe(0), in_scope_vars: [], local_variables: {self: my Main, @ fresh(0): Pair, @ fresh(1): leased [@ fresh(0) . b] Data, data: leased [@ fresh(0) . b] Data, pair: Pair}, assumptions: {}, fresh: 2 }
                                                                             the rule "prefix" failed at step #0 (src/file.rs:LL:CC) because
                                                                               condition evaluted to false: `places_b.iter().any(|place_b| place_b.is_prefix_of(&place_a))`"#]])
}
