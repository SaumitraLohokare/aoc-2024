mod input;
use input::*;

fn main() {
    let records: Vec<_> = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("Part 1: {}", part_1(&records));
    println!("Part 2: {}", part_2(&records));
}

fn part_1(records: &Vec<Vec<i32>>) -> usize {
    records
        .iter()
        .map(|r| {
            r.iter().is_sorted_by(|a, b| a < b && **b - **a <= 3)
                || r.iter().is_sorted_by(|a, b| a > b && **a - **b <= 3)
        })
        .filter(|r| *r)
        .count()
}

fn part_2(records: &Vec<Vec<i32>>) -> usize {
    records
        .iter()
        .map(|r| is_record_safe(r))
        .filter(|r| *r)
        .count()
}

fn is_record_safe(record: &Vec<i32>) -> bool {
    let mut dampned = false;
    let mut item = record.first().unwrap();

    let mut check_1 = true;
    let mut check_2 = true;

    for level in record.iter().skip(1) {
        if !(level < item && item - level <= 3) {
            if dampned {
                check_1 = false;
                break;
            }

            dampned = true;
        } else {
            item = level
        }
    }

    dampned = false;
    item = record.first().unwrap();

    for level in record.iter().skip(1) {
        if !(level > item && level - item <= 3) {
            if dampned {
                check_2 = false;
                break;
            }

            dampned = true;
        } else {
            item = level
        }
    }

    check_1
        || check_2
        || (record.iter().skip(1).is_sorted_by(|a, b| a < b && **b - **a <= 3)
            || record.iter().skip(1).is_sorted_by(|a, b| a > b && **a - **b <= 3))
}
