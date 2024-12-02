mod input;
use input::INPUT;

fn main() {
    let x = INPUT.lines().map(|l| {
        l.split("   ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut first = x.clone().map(|v| v[0]).collect::<Vec<_>>();
    let mut second = x.map(|v| v[1]).collect::<Vec<_>>();

    first.sort();
    second.sort();

    println!("Part 1: {}", part1(&first, &second));
    println!("Part 2: {}", part2(&first, &second));
}

fn part1(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    first
        .iter()
        .zip(second.iter())
        .map(|(f, s)| (f - s).abs())
        .sum::<i32>()
}

fn part2(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    first
        .iter()
        .map(|n| *n * second.iter().filter(|m| *m == n).count() as i32)
        .sum::<i32>()
}
