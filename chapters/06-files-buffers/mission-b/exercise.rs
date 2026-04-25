// Edit-build-fix loop — fix three issues across iterations.
// Run :sp | terminal below, then `cargo check` (no Cargo project required —
// `rustc --edition 2021 exercise.rs` works for quick checks).

// ROUND 1: type error — fix `let x: i32 = "hello";`
pub fn round_one() -> i32 {
    let x: i32 = "hello";
    x
}

// ROUND 2: unused variable — remove `_unused`
pub fn round_two() {
    let _unused = 42;
    println!("round two");
}

// ROUND 3: missing return type — add `-> i32`
pub fn add(a: i32, b: i32) {
    a + b
}
