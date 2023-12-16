use anyhow::Result;
use std::io::{self, Read};

struct Input(Vec<String>);

struct Task {
    input: Input,
}

impl Task {
    fn part1(&self) -> usize {
        self.input
            .0
            .iter()
            .map(|line| {
                let v = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
                let v = format!("{}{}", v.first().unwrap(), v.last().unwrap());
                v.parse::<usize>().unwrap()
            })
            .sum()
    }
}

fn parse(i: &str) -> Result<Task> {
    let input = Input(
        i.lines()
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<_>>(),
    );
    Ok(Task { input })
}

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let task = parse(&s)?;

    println!("day 1, part 1: {}", task.part1());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet\n";

    #[test]
    fn part1() {
        let task = parse(EXAMPLE).unwrap();
        assert_eq!(task.part1(), 142);
    }
}
