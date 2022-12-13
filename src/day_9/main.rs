use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    let dirs = parse(input);

    println!("P1: {}", p1(&dirs));
    println!("P2: {}", p2(&dirs));
}


fn p1(dirs: &[Movement]) -> usize {
    HashSet::<Position>::from_iter(run(dirs, 2).into_iter()).len()
}

fn p2(dirs: &[Movement]) -> usize {
    HashSet::<Position>::from_iter(run(dirs, 10).into_iter()).len()
}


fn run(dirs: &[Movement], segments: usize) -> Vec<Position> {
    let mut cur_state = vec![Position{x: 0, y: 0}; segments];
    let mut visited = vec![*cur_state.last().unwrap()];

    for dir in dirs {
        for _ in 0..dir.amount {
            match dir.direction {
                Direction::Up => cur_state[0].y += 1,
                Direction::Down => cur_state[0].y -= 1,
                Direction::Left => cur_state[0].x -= 1,
                Direction::Right => cur_state[0].x += 1,
            }
            for idx in 1..segments {
                cur_state[idx] = tail_movement(cur_state[idx-1], cur_state[idx]);
            }
            visited.push(*cur_state.last().unwrap())
        }
    }

    visited
}


fn tail_movement(head_pos: Position, tail_pos: Position) -> Position {
    let x_diff = head_pos.x - tail_pos.x;
    let y_diff = head_pos.y - tail_pos.y;
    let mut new_pos = tail_pos;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            // Don't move
            return new_pos
    }

    if y_diff != 0 {
        if y_diff > 0 { new_pos.y += 1 }
        else { new_pos.y -= 1 }
    }

    if x_diff != 0 {
        if x_diff > 0 { new_pos.x += 1 }
        else { new_pos.x -= 1 }
    }

    new_pos
}


#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Debug)]
struct Movement {
    direction: Direction,
    amount: i32,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Vec<Movement> {
    input.lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(i: &str) -> nom::IResult<&str, Movement> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::character::complete::one_of("UDLR"),
            nom::bytes::complete::tag(" "),
            nom::character::complete::i32
        ),
        |(c_dir, amount)| {
            let direction = match c_dir {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("Invalid input {c_dir}")
            };
            Movement {
                direction,
                amount
            }
        }
    )(i)
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "R 4\n\
    U 4\n\
    L 3\n\
    D 1\n\
    R 4\n\
    D 1\n\
    L 5\n\
    R 2";

    const INPUT_2: &str =
    "R 5\n\
    U 8\n\
    L 8\n\
    D 3\n\
    R 17\n\
    D 10\n\
    L 25\n\
    U 20";

    #[test]
    fn test1() {
        assert_eq!(13, p1(&parse(INPUT)));
    }

    #[test]
    fn test2() {
        assert_eq!(1, p2(&parse(INPUT)));
    }

    #[test]
    fn test3() {
        assert_eq!(36, p2(&parse(INPUT_2)));
    }

}