use std::collections::HashSet;
use itertools::Itertools;


fn main() {
    let input = parse(include_str!("input"));
    println!("P1: {}", p1(input.clone()));
    println!("P2: {}", p2(input));
}

fn parse(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut output = Vec::new();

    for line in input.lines() {
        let (l_str, r_str) = line.split_at(line.len()/2);
        output.push((
            HashSet::from_iter(l_str.chars()),
            HashSet::from_iter(r_str.chars()),  
        ));
    }

    output
}


fn p1(input: Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    input.into_iter()
        .map(|(set_1, set_2)| {
            let matching_ch = *set_1.intersection(&set_2).next().unwrap();
            item_value(matching_ch)
        })
        .sum()
}


fn p2(input: Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    input.into_iter()
        .tuples()
        .map(|((e1_l, e1_r), (e2_l, e2_r), (e3_l, e3_r))| {
            let e1: HashSet<char> = e1_l.union(&e1_r).copied().collect();
            let e2: HashSet<char> = e2_l.union(&e2_r).copied().collect();
            let e3: HashSet<char> = e3_l.union(&e3_r).copied().collect();

            let mut badge: HashSet<char> = e1.intersection(&e2).copied().collect();
            badge = badge.intersection(&e3).copied().collect();

            item_value(badge.into_iter().next().unwrap())
        })
        .sum()
}


fn item_value(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else {
        (item as u32) - ('A' as u32) + 27
    }
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test1() {
        assert_eq!(157, p1(parse(INPUT)));
    }

    #[test]
    fn test2() {
        assert_eq!(70, p2(parse(INPUT)));
    }

}