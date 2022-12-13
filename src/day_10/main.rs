
fn main() {
    let instrs = parse(include_str!("input"));
    let x_hist = run(&instrs);

    println!("P1: {}", p1(&x_hist));
    p2(&x_hist);
}


fn p1(x_history: &[i32]) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];

    cycles.iter()
        .map(|&cycle| {
            let x = *x_history.get(cycle).unwrap();
            cycle as i32 * x
        })
        .sum()
}


fn p2(x_history: &[i32]) {
    x_history.iter()
        .skip(1) // Skip cycle 0
        .enumerate()
        .for_each(|(cycle, &x)| {
            let horz_pix_pos = (cycle % 40) as i32;
            if horz_pix_pos == 0 { println!() }
            if horz_pix_pos >= (x - 1) && horz_pix_pos <= (x + 1) {
                print!("#")
            } else {
                print!(".")
            }
        });
    println!();
}


fn run(instrs: &[Instr]) -> Vec<i32> {
    let mut cur_x = 1;
    let mut x_history = vec![cur_x];

    for instr in instrs {
        match instr {
            Instr::Noop => x_history.push(cur_x),
            Instr::Addx(val) => {
                x_history.extend(std::iter::repeat(cur_x).take(2));
                cur_x += *val;
            }
        }
    }

    x_history
}


#[derive(Clone, Copy, Debug)]
enum Instr {
    Noop,
    Addx(i32),
}


fn parse(input: &str) -> Vec<Instr> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(i: &str) -> Instr {
    let result: nom::IResult<&str, Instr> = nom::branch::alt((
        nom::combinator::map(
            nom::sequence::preceded(
                nom::bytes::complete::tag("addx " ),
                nom::character::complete::i32
            ),
            Instr::Addx
        ),
        nom::combinator::map(
            nom::bytes::complete::tag("noop"),
            |_| Instr::Noop
        )
    ))(i);

    result.unwrap_or_else(|_| panic!("Unable to parse {i}")).1
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let instrs = parse(include_str!("test_input"));
        let x_hist = run(&instrs);
        assert_eq!(13140, p1(&x_hist));
    }

    #[test]
    fn test2() {
        let instrs = parse(include_str!("test_input"));
        let x_hist = run(&instrs);
        p2(&x_hist);
    }

}