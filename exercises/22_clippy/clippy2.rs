fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    #[allow(for_loops_over_fallibles)]
    for x in option {
        res += x;
    }

    println!("{res}");
}
