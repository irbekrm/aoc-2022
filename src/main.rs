use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day0();
}

fn day0() {
    if let Ok(lines) = read_lines("./calories.txt") {
        let mut current_largest: u32 = 0;
        let mut current: u32 = 0;
        for line in lines {
            if let Ok(l) = line {
            match l.parse::<u32>() {
                Ok(n) => {
                    current = current + n;
                    println!("n is {n}");
                },
                Err(n) => {
                    if current_largest < current {
                        println!("new largest {current}");
                        current_largest = current;
                    }
                    current = 0;
                },
            }
    }
      }
      println!("elf carrying most calories is {current_largest}");
    }

}

// Mostly from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
