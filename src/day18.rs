use std::hint::unreachable_unchecked;
#[allow(unused_imports)]
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value, map_res},
    bytes::complete::{tag, take_while1, take_while_m_n},
    multi::separated_list1,
    sequence::{tuple, preceded, terminated, delimited},
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone,Copy,Debug)]
#[allow(dead_code)]
pub struct TrenchInstruction {
    direction_p1: Direction,
    length_p1: i32,
    direction_p2: Direction,
    length_p2: i64
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<TrenchInstruction> {
    parse_list_of_trench_instructions(input)
}

fn parse_list_of_trench_instructions(input: &str) -> Vec<TrenchInstruction> {
    match all_consuming(separated_list1(tag("\n"), parse_trench_instruction))
    .parse(input){
        Ok((_,val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_trench_instruction(input: &str) -> IResult<&str, TrenchInstruction> {
    tuple((
            parse_direction, tag(" "), parse_num_to_i32, tag(" "), parse_rgb_hex
            ))
        .map(|el| TrenchInstruction {direction_p1: el.0, length_p1: el.2, direction_p2: el.4.1, length_p2: el.4.0})
        .parse(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
            ))
        .parse(input)
}

fn parse_num_to_i32(input: &str) -> IResult<&str, i32> {
    take_while1(char::is_numeric)
        .map(|el: &str| el.parse::<i32>().unwrap())
        .parse(input)
}

fn parse_rgb_hex(input: &str) -> IResult<&str, (i64,Direction)> {
    delimited(tag("(#"), tuple((parse_hex_length,parse_hex_direction)), tag(")"))
        .parse(input)
}

fn parse_hex_length(input: &str) -> IResult<&str, i64> {
    take_while_m_n(5, 5, char::is_alphanumeric)
        .map(|el| i64::from_str_radix(el, 16).unwrap())
        .parse(input)
}

fn parse_hex_direction(input: &str) -> IResult<&str, Direction> {
    alt((
            value(Direction::Right, tag("0")),
            value(Direction::Down, tag("1")),
            value(Direction::Left, tag("2")),
            value(Direction::Up, tag("3"))
            ))
        .parse(input)
}

#[inline]
fn trench_instructions_to_nodes_p1(input: &[TrenchInstruction]) -> Vec<(i32,i32)> {
    let mut node = (0,0);//is always in trench shape
    let mut out = Vec::<(i32,i32)>::from([node]);
    for instr in input.iter() {
        match instr.direction_p1 {
            Direction::Up => {
                node = (node.0, node.1 - instr.length_p1);
            },
            Direction::Down => {
                node = (node.0, node.1 + instr.length_p1);
            },
            Direction::Left => {
                node = (node.0 - instr.length_p1, node.1);
            },
            Direction::Right => {
                node = (node.0 + instr.length_p1, node.1);
            },
        }
        out.push(node);
    }
    out
}

/* Uses shoelace formula for finding the area of a polygon from its vertex coordinates
 * More info at : https://en.wikipedia.org/wiki/Shoelace_formula */
fn apply_shoelace_formula<T>
(input: &[(T,T)]) -> T
where 
    T : std::ops::Mul
    + std::ops::Sub
    + std::ops::Div<T>
    + std::ops::AddAssign
    + num::Signed
    + std::marker::Copy
    + std::convert::From<i32>
    + std::iter::Sum<<<T as std::ops::Mul>::Output as std::ops::Sub>::Output>,
{
    let mut ans = T::from(0);
    unsafe {
        let inc: T = input.windows(2)
            .map(|win| {
                let [p1,p2] = win else {unreachable_unchecked()};
                p1.0 * p2.1 - p1.1 * p2.0 //2x2 Matrix determinant
            })
        .sum();
        ans += inc;
    }
    ans = ans.abs() / T::from(2);
    ans
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[TrenchInstruction]) -> i32 {
    let trench_nodes = trench_instructions_to_nodes_p1(input);
    let mut ans = apply_shoelace_formula(&trench_nodes);
    let inc = input.iter()
        .filter(|el| (el.direction_p1 == Direction::Down) || (el.direction_p1 == Direction::Right))
        .map(|el| el.length_p1)
        .sum::<i32>();
    ans += inc + 1;
    ans
}

#[inline]
fn trench_instructions_to_nodes_p2(input: &[TrenchInstruction]) -> Vec<(i64,i64)> {
    let mut node = (0,0);//is always in trench shape
    let mut out = Vec::<(i64,i64)>::from([node]);
    for instr in input.iter() {
        match instr.direction_p2 {
            Direction::Up => {
                node = (node.0, node.1 - instr.length_p2);
            },
            Direction::Down => {
                node = (node.0, node.1 + instr.length_p2);
            },
            Direction::Left => {
                node = (node.0 - instr.length_p2, node.1);
            },
            Direction::Right => {
                node = (node.0 + instr.length_p2, node.1);
            },
        }
        out.push(node);
    }
    out
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[TrenchInstruction]) -> i64 {
    let trench_nodes = trench_instructions_to_nodes_p2(input);
    let mut ans = apply_shoelace_formula(&trench_nodes);
    let inc = input.iter()
        .filter(|el| (el.direction_p2 == Direction::Down) || (el.direction_p2 == Direction::Right))
        .map(|el| el.length_p2)
        .sum::<i64>();
    ans += inc + 1;
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn day18_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 14);
        assert_eq!(input[0].direction_p1, Direction::Right);
    }

    #[test]
    fn day18_shoelace_formula_1() {
        let input = vec![(1,6), (3,1), (7,2), (4,4), (8,5),(1,6)];
        let ans = apply_shoelace_formula(&input);
        assert_eq!(ans, 16);
    }

    #[test]
    fn day18_shoelace_formula_2() {
        let input = vec![(0,0),(0,1),(1,1),(1,0)];
        let ans = apply_shoelace_formula(&input);
        assert_eq!(ans, 1);
    }

    #[test]
    fn day18_shoelace_formula_3() {
        let input = vec![(0,0), (7,0), (7,10),(0,10),(0,0)];
        let ans = apply_shoelace_formula(&input);
        assert_eq!(ans, 70);
    }

    #[test]
    fn day18_instrs_to_coords() {
        let input = input_generator(TEST_INPUT);
        let ans = trench_instructions_to_nodes_p1(&input);
        assert_eq!(ans,
            vec![(0,0), (6,0), (6,5), (4,5), (4,7), (6,7), (6,9), (1,9), (1,7),
            (0,7), (0,5), (2,5), (2,2), (0,2), (0,0)]
        )
    }

    #[test]
    fn day18_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 62);
    }
}
