#[test]
fn build() {
    let t = trybuild::TestCases::new();

    t.pass("fixtures/01-monster.rs");
    t.pass("fixtures/02-default.rs");
    t.pass("fixtures/03-battle.rs");
    t.pass("fixtures/04-tame.rs");
    t.pass("fixtures/05-debug.rs");
    t.pass("fixtures/06-construction.rs");
    t.pass("fixtures/07-conversion.rs");
    t.pass("fixtures/08-compare.rs");
    t.pass("fixtures/09-clone.rs");
}
