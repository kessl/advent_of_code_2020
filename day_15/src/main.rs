use std::io::Write;
use std::io::stdout;

mod recitation;
use recitation::Recitation;

fn main() {
    let mut recitation = Recitation::start(vec![0, 8, 15, 2, 12, 1, 4]);
    for step in 7..(30000000 - 1) {
        recitation.speak();
        if step % 300000 == 0 {
            print!("\rCalculating... {}%", step / 300000);
            stdout().flush().unwrap();
        }
    }
    let result = recitation.speak();
    println!("\nThe 30000000th spoken number is {}", result);
}

fn main_step1() {
    let mut recitation = Recitation::start(vec![0, 8, 15, 2, 12, 1, 4]);
    for _ in 7..2019 {
        recitation.speak();
    }
    let result = recitation.speak();
    println!("The 2020th spoken number is {}", result);
}

#[test]
fn first_steps() {
    let mut recitation = Recitation::start(vec![0, 3, 6]);
    for result in vec![0, 3, 3, 1, 0, 4, 0] {
        let number = recitation.speak();
        assert_eq!(number, result);
    }
}

#[test]
fn the_2020th() {
    let mut recitation = Recitation::start(vec![0, 3, 6]);
    for _ in 3..2019 {
        recitation.speak();
    }
    let result = recitation.speak();
    assert_eq!(result, 436);
}
