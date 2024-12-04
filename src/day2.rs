/**/
// https://adventofcode.com/2024/day/1
/**/

use crate::prelude::*;

pub fn part1() -> Result<usize> {
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);

    let mut count = 0;
    for line in lines {
        let list_nums: Vec<usize> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        if is_safe(&list_nums) {
            count += 1;
        }
    }

    Ok(count)
}

fn is_safe(list: &[usize]) -> bool {
    let is_increasing = list.get(0) < list.get(1);
    for slice in list.windows(2) {
        let (n1, n2) = (slice[0], slice[1]);
        if (is_increasing && n1 > n2) || (!is_increasing && n2 > n1) || (n1 == n2) || (n2.abs_diff(n1) > 3) {
            return false;
        }
    }
    true
}

pub fn part2() -> Result<usize> {
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);

    let mut count = 0;
    for line in lines {
        let list_nums: Vec<usize> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        if is_safe(&list_nums) || is_almost_safe(list_nums) {
            count += 1;
        }
    }

    Ok(count)
}

fn is_almost_safe(mut list: Vec<usize>) -> bool {
    let is_increasing = list.get(0) < list.get(1);
    let mut failed_idx = None;
    for (idx, slice) in list.windows(2).enumerate() {
        let (n1, n2) = (slice[0], slice[1]);
        if (is_increasing && n1 > n2) || (!is_increasing && n2 > n1) || (n1 == n2) || (n2.abs_diff(n1) > 3) {
            failed_idx = Some(idx);
            break;
        }
    }
    // If it failed, try removing each of the potentially problematic entries and test again.
    if let Some(idx) = failed_idx {
        let mut list1 = list.clone();
        list.remove(idx);
        list1.remove(idx+1);
        return is_safe(&list) || is_safe(&list1);
    }
    true
}