

fn main() {
    let assignments = parse(include_str!("input"));
    println!("P1: {}", p1(&assignments));
    println!("P2: {}", p2(&assignments));
}


#[derive(Clone, Copy)]
struct Sections {
    lower: u64,
    upper: u64
}


fn parse(input: &str) -> Vec<(Sections, Sections)> {
    input.lines().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        
        let (r1_str, r2_str) = s1.split_once('-').unwrap();
        let section1 = Sections {
            lower: r1_str.parse().unwrap(),
            upper: r2_str.parse().unwrap()
        };

        let (r1_str, r2_str) = s2.split_once('-').unwrap();
        let section2 = Sections {
            lower: r1_str.parse().unwrap(),
            upper: r2_str.parse().unwrap()
        };

        (section1, section2)
    })
    .collect()
}


fn p1(assignments: &[(Sections, Sections)]) -> usize {
    assignments.iter()
        .filter(|(s1, s2)| {
            (s1.lower <= s2.lower && s1.upper >= s2.upper)
            ||
            (s2.lower <= s1.lower && s2.upper >= s1.upper)
        })
        .count()
}


fn p2(assignments: &[(Sections, Sections)]) -> usize {
    assignments.iter()
        .filter(|(s1, s2)| {
            (s1.lower <= s2.lower && s1.upper >= s2.upper)
            ||
            (s2.lower <= s1.lower && s2.upper >= s1.upper)
            ||
            (s2.lower <= s1.upper && s2.lower >= s1.lower)
            ||
            (s1.lower <= s2.upper && s1.lower >= s2.lower)
        })
        .count()
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "2-4,6-8\n\
    2-3,4-5\n\
    5-7,7-9\n\
    2-8,3-7\n\
    6-6,4-6\n\
    2-6,4-8";

    #[test]
    fn test1() {
        assert_eq!(2, p1(&parse(INPUT)));
    }

    #[test]
    fn test2() {
        assert_eq!(4, p2(&parse(INPUT)));
    }

}