advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    //Some(input.filter(|r| {is_safe_report(r)}).collect().len())
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut out: Vec<Vec<u32>> = vec![];
    input.lines().for_each(|r| {
        out.push(r.split_ascii_whitespace().map(|l| {
            l.parse().unwrap()
        }).collect())
    });
    println!("{out:?}");
    out
}

fn is_safe_report(input: &Vec<u32>) -> bool {
    let mut iter = input.iter();
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let inc = b > a;
    loop {
        b = match iter.next() {
            Some(i) => i,
            None => break
        };
        if !(a.abs_diff(*b) <= 4 && inc == (b > a)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
