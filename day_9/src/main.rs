mod xmas;
use xmas::Xmas;

fn main() {
    let xmas = Xmas::from_file("./data.txt", 25).unwrap();
    let error = xmas.first_invalid().expect("All numbers are valid");
    println!("First invalid number: {:?}", error);

    let contiguous = xmas
        .contiguous_sum(error)
        .expect("Could not find contiguous numbers that sum up to the provided value");
    println!(
        "Contiguous numbers that sum up to first invalid number: {:?}",
        contiguous
    );

    println!(
        "Encryption weakness: {}",
        contiguous.iter().min().unwrap() + contiguous.iter().max().unwrap()
    );
}

#[test]
fn test_data() {
    let xmas = Xmas::from_file("./test_data.txt", 5).unwrap();
    let error = xmas.first_invalid();
    assert_eq!(Some(127), error);

    let contiguous = xmas.contiguous_sum(error.unwrap());
    assert_eq!(Some(vec![15, 25, 47, 40]), contiguous);
}
