use std::collections::HashMap;
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value},
    bytes::complete::{tag, take_while1},
    multi::separated_list1,
    sequence::{tuple, preceded},
};

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Array2<RockType> {
    let parse_vec = match all_consuming(separated_list1(tag("\n"), parse_platform_line))
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    };
    let shape = (parse_vec.len(), parse_vec[0].dim());
    let flat = parse_vec.iter().flatten().cloned().collect();
    let input = Array2::from_shape_vec(shape, flat).expect("Dimensions didn't line up");
    input
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> u64 {
    input.split(",")
        .map(|sub_str| {
            sub_str.bytes()
                .fold(0u64, |acc,el| ((acc + el as u64) * 17) % 256)
        })
    .sum()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let instrs = parse_hash_instruction(input);
    let registers = hashmap_protocoll_for_p2(&instrs);
    calc_lens_power(registers)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    //#[test]
    //fn test_solve_day17_p1() {
    //}

    //#[test]
    //fn test_solve_day17_p2() {
    //}
}
