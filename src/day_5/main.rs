use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let (stacks, instrs) = parse(include_str!("input"));
    println!("P1: {}", p1(stacks.clone(), &instrs));
    println!("P2: {}", p2(stacks, &instrs));
}


lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
}


#[derive(Clone, Copy, Debug)]
struct Instr {
    qty: usize,
    from: usize,
    to: usize,
}


fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Instr>) {
    let mut stacks = vec![vec![]; 10];
    let mut instrs = Vec::new();

    let (stack_strs, instr_strs) = input.split_once("\n\n").unwrap();

    for line in stack_strs.lines().rev().skip(1) {
        let mut chars = line.chars();
        chars.next();

        for (idx, next_char) in chars.step_by(4).enumerate() {
            if !next_char.is_whitespace() {
                stacks[idx].push(next_char);
            }
        }
    }

    for line in instr_strs.lines() {
        if let Some(caps) = RE.captures(line) {
            instrs.push(Instr {
                qty: caps.get(1).unwrap().as_str().parse().unwrap(),
                from: caps.get(2).unwrap().as_str().parse().unwrap(),
                to: caps.get(3).unwrap().as_str().parse().unwrap(),
            });
        } else {
            panic!("Invalid str: {}", line)
        }
    }


    (stacks, instrs)
}


fn p1(mut stacks: Vec<Vec<char>>, instrs: &[Instr]) -> String {
    for instr in instrs {
        for _ in 0..instr.qty {
            let c = stacks[instr.from-1].pop().unwrap();
            stacks[instr.to-1].push(c);
        }
    }

    stacks.into_iter()
        .filter_map(|mut stack| {
            stack.pop()
        })
        .collect()
}


fn p2(mut stacks: Vec<Vec<char>>, instrs: &[Instr]) -> String {
    for instr in instrs {
        let temp_stack: Vec<char> = (0..instr.qty)
                .map(|_| stacks[instr.from-1].pop().unwrap())
                .collect();

        temp_stack.iter().rev().for_each(|c| stacks[instr.to-1].push(*c));
    }

    stacks.into_iter()
        .filter_map(|mut stack| {
            stack.pop()
        })
        .collect()
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "    [D]    \n\
    [N] [C]    \n\
    [Z] [M] [P]\n\
     1   2   3 \n\
    \n\
    move 1 from 2 to 1\n\
    move 3 from 1 to 3\n\
    move 2 from 2 to 1\n\
    move 1 from 1 to 2";

    #[test]
    fn test1() {
        let (stacks, instrs) = parse(INPUT);
        assert_eq!("CMZ", p1(stacks, &instrs));        
    }

    #[test]
    fn test2() {
        let (stacks, instrs) = parse(INPUT);
        assert_eq!("MCD", p2(stacks, &instrs));        
    }

}