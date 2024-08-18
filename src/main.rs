
use special_testing::urusura;

mod special_testing;

fn main() {
    doit();
}

fn doit() {
    urusura();
}

#[test] // so that you can run this with `cargo test --release -- my_bench --nocapture`
fn my_bench() {
    doit();
}
