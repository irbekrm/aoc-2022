use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day0();
    day1();
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
                },
                Err(_) => {
                    if current_largest < current {
                        current_largest = current;
                    }
                    current = 0;
                },
            }
    }
      }
      println!("day 0: elf carrying most calories is {current_largest}");
    }

}

fn day1() {
  if let Ok(lines) = read_lines("./calories.txt") {
    let mut current_largest: [u32; 3] = [0, 0, 0];
    let mut current: u32 = 0;
    let mut counter = 0;
    for line in lines {
        if let Ok(l) = line {
        match l.parse::<u32>() {
            Ok(n) => {
                current = current + n;
            },
            Err(_) => {
                if counter < 3 {
                    current_largest[counter] = current;
                    counter += 1;
                } else {
                    if counter == 3 {
                    // sort the items in descending order
                    for i in 0..(current_largest.len() - 1) * 2 {
                        let index = i % (current_largest.len() - 1);
                        if current_largest[index] < current_largest[index + 1] {
                            let temp = current_largest[index];
                            current_largest[index] = current_largest[index + 1];
                            current_largest[index + 1] = temp;
                        }
                      }
                    }
                    // there is definitely a more elegant way to do this
                    if current_largest[0] < current {
                        current_largest[0] = current;
                    } else if current_largest[1] < current {
                        current_largest[1] = current;
                    } else if current_largest[2] < current {
                        current_largest[2] = current;
                    }
              }
              current = 0;
            },
        }
      }
    }
    println!("day 1: top 3 elves are carrying {} calories", current_largest[0] + current_largest[1] + current_largest[2]);
  }
}

// Mostly from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
