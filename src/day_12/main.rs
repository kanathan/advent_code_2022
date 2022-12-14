use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let (tiles, start, stop) = parse(include_str!("input"));
    println!("P1: {}", p1(&tiles, start, stop));
    println!("P2: {}", p2(&tiles, start, stop));
}


fn p1(tiles: &[Vec<Tile>], start: Position, stop: Position) -> u32 {
    let mut reached = HashSet::new();
    let mut frontier = VecDeque::new();

    frontier.push_back((start, 0));
    reached.insert(start);

    while let Some((cur_pos, steps)) = frontier.pop_front() {
        if cur_pos == stop {
            return steps
        }

        for new_pos in get_valid_destinations(tiles, cur_pos) {
            if reached.contains(&new_pos) { continue }
            frontier.push_back((new_pos, steps+1));
            reached.insert(new_pos);
        }
    }

    unreachable!()
}


fn p2(tiles: &[Vec<Tile>], _start: Position, stop: Position) -> u32 {
    let mut reached = HashSet::new();
    let mut frontier = VecDeque::new();

    frontier.push_back((stop, 0));
    reached.insert(stop);

    while let Some((cur_pos, steps)) = frontier.pop_front() {
        if tiles[cur_pos.y][cur_pos.x].height == 0 {
            return steps
        }

        for new_pos in get_valid_tiles_from(tiles, cur_pos) {
            if reached.contains(&new_pos) { continue }
            frontier.push_back((new_pos, steps+1));
            reached.insert(new_pos);
        }
    }

    unreachable!()
}


fn get_valid_destinations(tiles: &[Vec<Tile>], cur_pos: Position) -> Vec<Position> {
    let mut valid_destinations = vec![];
    let cur_height = tiles[cur_pos.y][cur_pos.x].height;

    if cur_pos.y > 0 && tiles[cur_pos.y-1][cur_pos.x].height <= cur_height + 1 {
        valid_destinations.push(Position::new(cur_pos.x, cur_pos.y-1));
    }

    if cur_pos.x > 0 && tiles[cur_pos.y][cur_pos.x-1].height <= cur_height + 1 {
        valid_destinations.push(Position::new(cur_pos.x-1, cur_pos.y));
    }

    if cur_pos.y < tiles.len()-1 && tiles[cur_pos.y+1][cur_pos.x].height <= cur_height + 1 {
        valid_destinations.push(Position::new(cur_pos.x, cur_pos.y+1));
    }

    if cur_pos.x < tiles[0].len()-1 && tiles[cur_pos.y][cur_pos.x+1].height <= cur_height + 1 {
        valid_destinations.push(Position::new(cur_pos.x+1, cur_pos.y));
    }

    valid_destinations
}


fn get_valid_tiles_from(tiles: &[Vec<Tile>], cur_pos: Position) -> Vec<Position> {
    let mut valid_positions = vec![];
    let cur_height = tiles[cur_pos.y][cur_pos.x].height;

    if cur_pos.y > 0 && tiles[cur_pos.y-1][cur_pos.x].height + 1 >= cur_height {
        valid_positions.push(Position::new(cur_pos.x, cur_pos.y-1));
    }

    if cur_pos.x > 0 && tiles[cur_pos.y][cur_pos.x-1].height + 1 >= cur_height {
        valid_positions.push(Position::new(cur_pos.x-1, cur_pos.y));
    }

    if cur_pos.y < tiles.len()-1 && tiles[cur_pos.y+1][cur_pos.x].height + 1 >= cur_height {
        valid_positions.push(Position::new(cur_pos.x, cur_pos.y+1));
    }

    if cur_pos.x < tiles[0].len()-1 && tiles[cur_pos.y][cur_pos.x+1].height + 1 >= cur_height {
        valid_positions.push(Position::new(cur_pos.x+1, cur_pos.y));
    }

    valid_positions
}


struct Tile {
    height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}


fn parse(input: &str) -> (Vec<Vec<Tile>>, Position, Position) {
    let mut start = None;
    let mut dest = None;
    let tiles = input.lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
            .enumerate()
            .map(|(x, c)| {
                match c {
                    'a'..='z' => Tile { height: (c as u32) - ('a' as u32) },
                    'S' => {
                        start = Some(Position::new(x, y));
                        Tile { height: 0 }
                    },
                    'E' => {
                        dest = Some(Position::new(x, y));
                        Tile { height: 25 }
                    },
                    _ => unreachable!("Invalid char {c}")
                }
            })
            .collect()
        })
        .collect();
    
    (tiles, start.unwrap(), dest.unwrap())
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "Sabqponm\n\
    abcryxxl\n\
    accszExk\n\
    acctuvwj\n\
    abdefghi";

    #[test]
    fn test1() {
        let (tiles, start, stop) = parse(INPUT);
        assert_eq!(31, p1(&tiles, start, stop));
    }

    #[test]
    fn test2() {
        let (tiles, start, stop) = parse(INPUT);
        assert_eq!(29, p2(&tiles, start, stop));
    }

}