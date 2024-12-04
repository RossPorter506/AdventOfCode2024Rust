/**/
// https://adventofcode.com/2024/day/1
/**/

use crate::prelude::*;

pub fn part1() -> Result<usize> {
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);

    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    for line in lines {
        let [first, second] = line.split_whitespace().collect::<Vec<&str>>()[..] else {unreachable!()};
        list1.push(first.parse()?);
        list2.push(second.parse()?);
    }

    let mut total = 0;
    while !list1.is_empty() || !list2.is_empty() {
        let (l1_idx, l1_min) = list1.iter().enumerate().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        let (l2_idx, l2_min) = list2.iter().enumerate().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        let diff = l1_min.abs_diff(*l2_min);
        total += diff;
        list1.remove(l1_idx);
        list2.remove(l2_idx);
    }
    Ok(total)
}

pub fn part2() -> Result<usize> {
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);

    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    for line in lines {
        let [first, second] = line.split_whitespace().collect::<Vec<&str>>()[..] else {unreachable!()};
        list1.push(first.parse()?);
        list2.push(second.parse()?);
    }
    
    let mut total = 0;
    for elem in list1 {
        let count = list2.iter().filter(|&&x| x == elem).count();
        total += elem*count;
    }
    Ok(total)
}