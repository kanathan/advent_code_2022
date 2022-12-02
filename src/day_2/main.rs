
fn main() {
    let plays = parse(include_str!("input"));
    println!("{}", p1(plays.clone()));
    println!("{}", p2(plays));
}


fn p1(input: Vec<(Elf, Player)>) -> u64 {
    input.into_iter()
        .map(|(elf, player)| play_round(elf, player))
        .sum()
}


fn p2(input: Vec<(Elf, Player)>) -> u64 {
    input.into_iter()
        .map(|(elf, player)| play_round_p2(elf, player))
        .sum()
}


fn parse(input: &str) -> Vec<(Elf, Player)> {
    let mut output = Vec::new();
    for line in input.lines() {
        let elf_chr = line.chars().next().unwrap();
        let player_chr = line.chars().nth(2).unwrap();

        let elf = match elf_chr {
            'A' => Elf::A_Rock,
            'B' => Elf::B_Paper,
            'C' => Elf::C_Scissors,
            _ => unreachable!("Unknown: {}", elf_chr)
        };
        let player = match player_chr {
            'X' => Player::X_Rock,
            'Y' => Player::Y_Paper,
            'Z' => Player::Z_Scissors,
            _ => unreachable!("Unknown: {}", player_chr)
        };

        output.push((elf, player));
    }
    output
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Elf {
    A_Rock,
    B_Paper,
    C_Scissors,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Player {
    X_Rock,
    Y_Paper,
    Z_Scissors,
}

fn play_round(elf: Elf, player: Player) -> u64 {
    match (elf, player) {
        (Elf::A_Rock, Player::X_Rock)           => 1 + 3,
        (Elf::A_Rock, Player::Y_Paper)          => 2 + 6,
        (Elf::A_Rock, Player::Z_Scissors)       => 3 + 0,
        (Elf::B_Paper, Player::X_Rock)          => 1 + 0,
        (Elf::B_Paper, Player::Y_Paper)         => 2 + 3,
        (Elf::B_Paper, Player::Z_Scissors)      => 3 + 6,
        (Elf::C_Scissors, Player::X_Rock)       => 1 + 6,
        (Elf::C_Scissors, Player::Y_Paper)      => 2 + 0,
        (Elf::C_Scissors, Player::Z_Scissors)   => 3 + 3,
    }
}


fn play_round_p2(elf: Elf, player: Player) -> u64 {
    match (elf, player) {
        (Elf::A_Rock, Player::X_Rock)           => 3 + 0,
        (Elf::A_Rock, Player::Y_Paper)          => 1 + 3,
        (Elf::A_Rock, Player::Z_Scissors)       => 2 + 6,
        (Elf::B_Paper, Player::X_Rock)          => 1 + 0,
        (Elf::B_Paper, Player::Y_Paper)         => 2 + 3,
        (Elf::B_Paper, Player::Z_Scissors)      => 3 + 6,
        (Elf::C_Scissors, Player::X_Rock)       => 2 + 0,
        (Elf::C_Scissors, Player::Y_Paper)      => 3 + 3,
        (Elf::C_Scissors, Player::Z_Scissors)   => 1 + 6,
    }
}




#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "A Y\n\
    B X\n\
    C Z";

    #[test]
    fn test1() {
        assert_eq!(15, p1(parse(INPUT)));
    }

    #[test]
    fn test2() {
        assert_eq!(12, p2(parse(INPUT)));
    }

}