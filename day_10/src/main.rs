mod adapters;
use adapters::Adapters;

fn main() {
    let adapters = Adapters::from_file("./adapters.txt").unwrap();
    let (one, _, three) = adapters.chain();
    println!(
        "{} differences of 1 jolt, {} differences of 3 jolts. {} * {} = {}",
        one,
        three,
        one,
        three,
        one * three
    )
}

#[test]
fn test_1() {
    let adapters = Adapters::from_file("./test_adapters_1.txt").unwrap();
    let dist = adapters.chain();
    assert_eq!((dist.0, dist.2), (7, 5));
}

#[test]
fn test_2() {
    let adapters = Adapters::from_file("./test_adapters_2.txt").unwrap();
    let dist = adapters.chain();
    assert_eq!((dist.0, dist.2), (22, 10));
}
