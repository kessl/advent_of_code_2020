mod ferry;
use ferry::Ferry;

fn main() {
    let mut ferry = Ferry::from_file("./layout.txt").unwrap();
    ferry.stabilize();
}

#[test]
fn test() {
    let mut ferry = Ferry::from_file("./test_layout.txt").unwrap();
    let occupied = ferry.stabilize();
    assert_eq!(26, occupied);
}
