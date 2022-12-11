
use nom::{
    IResult,
    Finish,
    branch::alt,
    sequence::{preceded, separated_pair},
    combinator::{map, rest, all_consuming},
    bytes::complete::tag,
    character::complete,
};

const TOT_SPACE: u64 = 70000000;
const REQ_SPACE: u64 = 30000000;

fn main() {
    let input = include_str!("input");
    let filedir = parse_input(input);

    println!("P1: {}", p1(&filedir));
    println!("P2: {}", p2(&filedir));
}

fn p1(filedir: &[Node]) -> u64 {
    get_directory_sizes(filedir).iter()
        .filter(|&&val| val <= 100000)
        .sum()
}

fn p2(filedir: &[Node]) -> u64 {
    let dir_sizes = get_directory_sizes(filedir);

    let used_space = *dir_sizes.iter().max().unwrap();
    let min_space_to_del = REQ_SPACE - (TOT_SPACE - used_space);

    *dir_sizes.iter()
        .filter(|&&val| val > min_space_to_del)
        .min()
        .unwrap()
}

#[derive(Debug)]
enum LineType {
    Command(CommandType),
    Output(FileType)
}

#[derive(Debug)]
enum CommandType {
    Ls,
    Cd(String),
}

#[derive(Debug)]
enum FileType {
    Directory(String),
    File(u64, String),
}


#[derive(Debug)]
struct Node {
    file_type: FileType,
    parent: Option<usize>,
    children: Vec<usize>,
}


fn get_directory_sizes(filedir: &[Node]) -> Vec<u64> {
    filedir.iter()
        .enumerate()
        .filter_map(|(idx, node)| {
            if matches!(node.file_type, FileType::Directory(_)) {
                Some(idx)
            } else {
                None
            }
        })
        .map(|dir_idx| calc_dir_size(filedir, dir_idx))
        .collect()
}


fn calc_dir_size(filedir: &[Node], dir_idx: usize) -> u64 {
    filedir[dir_idx].children.iter()
        .map(|&child_idx| {
            match filedir[child_idx].file_type {
                FileType::Directory(_) => calc_dir_size(filedir, child_idx),
                FileType::File(size, _) => size,
            }
        })
        .sum()
}


fn parse_input(input: &str) -> Vec<Node> {
    let parsed_lines: Vec<LineType> = input.lines()
        .map(|line| {
            all_consuming(parse_line)(line).finish().unwrap().1
        })
        .collect();
    
    let mut file_dir = Vec::from([
        Node {
            file_type: FileType::Directory("/".to_string()),
            parent: None,
            children: vec![],
        }
    ]);
    let mut parent_idx = 0;

    for line in parsed_lines {
        match line {
            LineType::Command(CommandType::Ls) => (),
            LineType::Command(CommandType::Cd(name)) => {
                if name == ".." {
                    if let Some(idx) = file_dir[parent_idx].parent {
                        parent_idx = idx;
                    }
                }
                else {
                    let found_dir = file_dir[parent_idx].children.iter()
                        .find(|&&idx| {
                            let node = &file_dir[idx];
                            matches!(&node.file_type, FileType::Directory(dir_name) if dir_name == &name)
                        });
                    if let Some(&idx) = found_dir {
                        parent_idx = idx;
                    } else {
                        add_item(&mut file_dir, parent_idx, FileType::Directory(name));
                        parent_idx = file_dir.len() - 1;
                    }
                }
            },
            LineType::Output(FileType::Directory(name)) => {
                if !file_dir.iter().any(|node| matches!(&node.file_type, FileType::Directory(dir_name) if dir_name == &name)) {
                    add_item(&mut file_dir, parent_idx, FileType::Directory(name));
                }
            },
            LineType::Output(FileType::File(size, name)) => {
                add_item(&mut file_dir, parent_idx, FileType::File(size, name));
            }
        }
    }

    file_dir
}


fn add_item(file_dir: &mut Vec<Node>, parent_idx: usize, item: FileType) {
    let child_idx = file_dir.len();
    file_dir.push(Node {
        file_type: item,
        parent: Some(parent_idx),
        children: vec![],
    });
    file_dir[parent_idx].children.push(child_idx);
}


fn parse_line(input: &str) -> IResult<&str, LineType> {
    alt((
        parse_command,
        parse_output
    ))(input)
}


fn parse_command(input: &str) -> IResult<&str, LineType> {
    map(
        preceded(tag("$ "), alt((parse_cd, parse_ls))),
        LineType::Command
    )(input)
}


fn parse_cd(input: &str) -> IResult<&str, CommandType> {
    map(preceded(tag("cd "), rest), |s: &str| CommandType::Cd(s.to_string()))(input)
}

fn parse_ls(input: &str) -> IResult<&str, CommandType> {
    map(tag("ls"), |_| CommandType::Ls)(input)
}


fn parse_output(input: &str) -> IResult<&str, LineType> {
    map(
        alt((parse_directory, parse_file)),
        LineType::Output
    )(input)
}


fn parse_directory(input: &str) -> IResult<&str, FileType> {
    map(
        preceded(tag("dir "), rest),
        |s: &str| FileType::Directory(s.to_string())
    )(input)
}


fn parse_file(input: &str) -> IResult<&str, FileType> {
    map(
        separated_pair(complete::u64, tag(" "), rest),
        |(size, name): (u64, &str)| FileType::File(size, name.to_string())
    )(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "$ cd /\n\
    $ ls\n\
    dir a\n\
    14848514 b.txt\n\
    8504156 c.dat\n\
    dir d\n\
    $ cd a\n\
    $ ls\n\
    dir e\n\
    29116 f\n\
    2557 g\n\
    62596 h.lst\n\
    $ cd e\n\
    $ ls\n\
    584 i\n\
    $ cd ..\n\
    $ cd ..\n\
    $ cd d\n\
    $ ls\n\
    4060174 j\n\
    8033020 d.log\n\
    5626152 d.ext\n\
    7214296 k";

    #[test]
    fn test1() {
        let filedir = parse_input(INPUT);
        assert_eq!(
            95437,
            p1(&filedir)
        );
        assert_eq!(
            24933642,
            p2(&filedir)
        );
    }

}