use itertools::Itertools;
use std::collections::VecDeque;


fn main() {
    let monkeys = parse(include_str!("input"));
    println!("P1: {}", p1(monkeys.clone()));
    println!("P2: {}", p2(monkeys));
}


fn p1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        run_round(&mut monkeys, None);
    }

    let sorted_monkeys: Vec<usize> = monkeys.iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .collect();
    
    sorted_monkeys[0] * sorted_monkeys[1]        
}

fn p2(mut monkeys: Vec<Monkey>) -> usize {
    let divisor = monkeys.iter()
        .map(|m| m.test_div)
        .product();


    for _ in 0..10000 {
        run_round(&mut monkeys, Some(divisor));
    }

    let sorted_monkeys: Vec<usize> = monkeys.iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .collect();
    
    sorted_monkeys[0] * sorted_monkeys[1]        
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test_div: u64,
    test_true_dest: usize,
    test_false_dest: usize,
    inspection_count: usize,
}

#[derive(Clone, Copy, Debug)]
struct Item(u64);

#[derive(Clone, Copy, Debug)]
enum Operation {
    Mult(u64),
    Add(u64),
    Square,
}


fn run_round(monkeys: &mut Vec<Monkey>, divisor: Option<u64>) {
    for idx in 0..monkeys.len() {
        while let Some(mut item) = monkeys[idx].items.pop_front() {
            match monkeys[idx].operation {
                Operation::Mult(val) => item.0 *= val,
                Operation::Add(val) => item.0 += val,
                Operation::Square => item.0 = item.0 * item.0,
            }

            if let Some(div) = divisor {
                item.0 %= div;
            } else {
                item.0 /= 3;
            }

            let dest =
                if item.0 % monkeys[idx].test_div == 0 {
                    monkeys[idx].test_true_dest
                } else {
                    monkeys[idx].test_false_dest
                };
            
            monkeys[dest].items.push_back(item);

            monkeys[idx].inspection_count += 1;
        }
    }
}


fn parse(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines().peekable();
    let mut output = vec![];

    while lines.peek().is_some() {
        lines.next(); // Skip first line
        let items = parse_items(lines.next().unwrap()).unwrap().1;
        let operation = parse_operation(lines.next().unwrap()).unwrap().1;
        let test_div = parse_test_div(lines.next().unwrap()).unwrap().1;
        let test_true_dest = parse_throw(lines.next().unwrap()).unwrap().1;
        let test_false_dest = parse_throw(lines.next().unwrap()).unwrap().1;
        lines.next(); // Skip blank line or do nothing if empty

        output.push(Monkey {
            items,
            operation,
            test_div,
            test_true_dest,
            test_false_dest,
            inspection_count: 0
        })
    }

    output
}


fn parse_items(i: &str) -> nom::IResult<&str, VecDeque<Item>> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("  Starting items: "),
            nom::multi::separated_list0(
                nom::bytes::complete::tag(", "),
                nom::character::complete::u64
            )
        ),
        |list| {
            list.into_iter().map(Item).collect()
        }
    )(i)
}

fn parse_operation(i: &str) -> nom::IResult<&str, Operation> {
    nom::sequence::preceded(
        nom::bytes::complete::tag("  Operation: "), 
        nom::branch::alt((
            nom::combinator::map(
                nom::sequence::preceded(
                    nom::bytes::complete::tag("new = old * "), 
                    nom::character::complete::u64
                ),
                Operation::Mult
            ),
            nom::combinator::map(
                nom::sequence::preceded(
                    nom::bytes::complete::tag("new = old + "), 
                    nom::character::complete::u64
                ),
                Operation::Add
            ),
            nom::combinator::map(
                nom::bytes::complete::tag("new = old * old"),
                |_| Operation::Square
            )
        ))
    )(i)
}

fn parse_test_div(i: &str) -> nom::IResult<&str, u64> {
    nom::sequence::preceded(
        nom::bytes::complete::tag("  Test: divisible by "),
        nom::character::complete::u64
    )(i)
}

fn parse_throw(i: &str) -> nom::IResult<&str, usize> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::branch::alt((
                nom::bytes::complete::tag("    If true: throw to monkey "),
                nom::bytes::complete::tag("    If false: throw to monkey "), 
            )),
            nom::character::complete::u64
        ), 
        |val| val as usize
    )(i)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("input_test");

    #[test]
    fn test1() {
        let monkeys = parse(INPUT);
        assert_eq!(10605, p1(monkeys));
    }

    #[test]
    fn test2() {
        let monkeys = parse(INPUT);
        assert_eq!(2713310158, p2(monkeys));
    }

}