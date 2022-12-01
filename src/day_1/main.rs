use itertools::Itertools;



fn main() {
    let input = parse(include_str!("input"));
    println!("{:?}", p1(input.clone()));
    println!("{}", p2(input))
}


fn p1(input: Vec<Vec<u64>>) -> (usize, u64) {
    input
        .into_iter()
        .map(|cal_list| {
            cal_list.into_iter().sum::<u64>()
        })
        .enumerate()
        .max_by(|(_,v1), (_,v2)| {
            v1.cmp(v2)
        })
        .unwrap()
}


fn p2(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|cal_list| {
            cal_list.into_iter().sum::<u64>()
        })
        .sorted_by(|v1, v2| v2.cmp(v1))
        .take(3)
        .sum()
}


fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n\n")
        .map(|list| {
            list
                .lines()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "1000\n\
    2000\n\
    3000\n\
    \n\
    4000\n\
    \n\
    5000\n\
    6000\n\
    \n\
    7000\n\
    8000\n\
    9000\n\
    \n\
    10000";

    #[test]
    fn test1() {
        assert_eq!((3, 24000), p1(parse(INPUT)))
    }

    #[test]
    fn test2() {
        assert_eq!(45000, p2(parse(INPUT)))
    }

}


