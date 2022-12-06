use std::collections::HashSet;


fn main() {
    let input = include_str!("input");
    println!("P1: {}", get_first_unique_set(input, 4));
    println!("P2: {}", get_first_unique_set(input, 14));
}


fn get_first_unique_set(input: &str, size: usize) -> usize {
    input.chars()
        .collect::<Vec<char>>()
        .windows(size)
        .enumerate()
        .find_map(|(idx, slice)| {
            if all_unique(slice) {
                Some(idx + size)
            } else {
                None
            }
        })
        .expect("Unable to find unique set")
}


fn all_unique(input: &[char]) -> bool {
    let mut unique_set = HashSet::new();
    input.iter().all(|c| unique_set.insert(*c))
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(7, get_first_unique_set("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, get_first_unique_set("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, get_first_unique_set("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, get_first_unique_set("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, get_first_unique_set("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test2() {
        assert_eq!(19, get_first_unique_set("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, get_first_unique_set("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, get_first_unique_set("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, get_first_unique_set("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, get_first_unique_set("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }

}