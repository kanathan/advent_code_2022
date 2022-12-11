

fn main() {
    let input = include_str!("input");
    let trees = parse(input);

    println!("P1: {}", p1(trees.clone()));
    println!("P2: {}", p2(trees));
}


fn p1(mut trees: Vec<Vec<Tree>>) -> usize {
    calc_visibility(&mut trees);
    trees.iter()
        .flatten()
        .filter(|t| t.visible)
        .count()
}

fn p2(trees: Vec<Vec<Tree>>) -> u64 {
    let mut max_score = 0;

    for cur_row in 0..trees.len() {
        for cur_col in 0..trees[0].len() {
            let mut score = 1; // 1 since we'll be multiplying
            let max_height = trees[cur_row][cur_col].height;
            // Up
            let mut tree_count = 0;
            for row in (0..cur_row).rev() {
                let t = &trees[row][cur_col];
                tree_count += 1;
                if t.height >= max_height {
                    break
                }
            }
            score *= tree_count;

            // Down
            let mut tree_count = 0;
            #[allow(clippy::needless_range_loop)]
            for row in (cur_row+1)..trees.len() {
                let t = &trees[row][cur_col];
                tree_count += 1;
                if t.height >= max_height {
                    break
                }
            }
            score *= tree_count;

            // Left
            let mut tree_count = 0;
            for col in (0..cur_col).rev() {
                let t = &trees[cur_row][col];
                tree_count += 1;
                if t.height >= max_height {
                    break
                }
            }
            score *= tree_count;

            // Right
            let mut tree_count = 0;
            for col in (cur_col+1)..trees[0].len() {
                let t = &trees[cur_row][col];
                tree_count += 1;
                if t.height >= max_height {
                    break
                }
            }
            score *= tree_count;

            max_score = max_score.max(score);
        }
    }

    max_score
}


#[derive(Copy, Clone)]
struct Tree {
    height: i8,
    visible: bool,
}


fn calc_visibility(trees: &mut [Vec<Tree>]) {
    // Process each row
    for row in trees.iter_mut() {
        let mut max_height = -1;
        for t in row.iter_mut() {
            if t.height > max_height {
                t.visible = true;
                max_height = t.height;
            }
        }
        max_height = -1;
        for t in row.iter_mut().rev() {
            if t.height > max_height {
                t.visible = true;
                max_height = t.height;
            }
        }
    }

    // Process each column
    for col_idx in 0..trees[0].len() {
        let mut max_height = -1;
        for row in trees.iter_mut() {
            let t = row.get_mut(col_idx).unwrap();
            if t.height > max_height {
                t.visible = true;
                max_height = t.height;
            }
        }

        let mut max_height = -1;
        for row in trees.iter_mut().rev() {
            let t = row.get_mut(col_idx).unwrap();
            if t.height > max_height {
                t.visible = true;
                max_height = t.height;
            }
        }
    }

}


fn parse(input: &str) -> Vec<Vec<Tree>> {
    input.lines()
        .map(|line| {
            line.chars().map(|c| {
                let height = c.to_digit(10).unwrap() as i8;
                Tree {
                    height,
                    visible: false,
                }
            })
            .collect()
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "30373\n\
    25512\n\
    65332\n\
    33549\n\
    35390";

    #[test]
    fn test1() {
        let trees = parse(INPUT);
        assert_eq!(21, p1(trees))
    }

    #[test]
    fn test2() {
        let trees = parse(INPUT);
        assert_eq!(8, p2(trees))
    }

}