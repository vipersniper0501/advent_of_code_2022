use std::{fs, io::{self, BufRead}};

struct Day9 {

}

impl Day9 {

    fn new() -> Self {
        Self {
        }
    }

    fn parse(&mut self) {
        let file = fs::File::open("data/input")
            .expect("No File Found");

        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }

        }
    }

    fn part1(&self) {
        println!("Part1: Unsolved");
    }

    fn part2(&self) {
        println!("Part2: Unsolved");
    }
}


fn main() {
    let mut day = Day9::new();
    day.parse();
    day.part1();
    day.part2();
}
