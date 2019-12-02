use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;

fn calc(f: f32) -> f32 {
    let result: f32 = (f / 3.0).trunc() - 2.0;
    if result <= 0.0 {
        return result;
    }
    if calc(result) > 0.0 {
        return result + calc(result);
    } else {
        return result;
    }
}

fn main() {
    let file = File::open("data/input").unwrap();
    let reader = BufReader::new(file);
    let sum: f32 = Sum::sum(
        reader
            .lines()
            .map(|x| calc(x.unwrap().parse::<f32>().unwrap())),
    );
    println!("{}", sum);
}

#[test]
fn test_calc() {
    assert_eq!(calc(14.0), 2.0);
    assert_eq!(calc(1969.0), 966.0);
    assert_eq!(calc(100756.0), 50346.0);
}
