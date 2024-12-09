advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (list1, list2) = parseLists(input);

    let mut sum: u64 = 0;
    let mut list1_iter = list1.into_iter();
    let mut list2_iter = list2.into_iter();
    'outer: loop {
        match list1_iter.next() {
            Some(a) => match list2_iter.next() {
                Some(b) => sum += a.abs_diff(b),
                None => break 'outer,
            },
            None => break 'outer,
        }
    }
    Some(sum as u32)
}

fn parseLists(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut list1: Vec<u64> = input
        .lines()
        .map(|s| s.split_ascii_whitespace().next().unwrap())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut list2: Vec<u64> = input
        .lines()
        .map(|s| {
            let mut s = s.split_ascii_whitespace();
            s.next();
            s.next().unwrap()
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    list1.sort();
    list2.sort();
    (list1, list2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parseLists(input);
    println!("list1: {list1:?}\nlist2: {list2:?}");

    let mut sum = 0;
    let mut list1_iter = list1.into_iter();

    loop {
        let a = match list1_iter.next() {
            Some(i) => i,
            None => break,
        };
        let tmp: Vec<&u64> = list2.iter().filter(|b| a.eq(b)).collect();
        let len = tmp.len() as u64;
        sum += a * len;
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
