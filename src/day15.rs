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

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum Instruction {
    Assign(usize),
    Remove
}

struct LensInstruction {
    instruction: Instruction,
    lens_label: String,
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> u64 {
    input.split(",")
        .map(|sub_str| {
            sub_str.bytes()
                .fold(0u64, |acc,el| ((acc + el as u64) * 17) % 256)
        })
    .sum()
}

fn parse_hash_instruction(input: &str) -> Vec<LensInstruction> {
    match all_consuming(separated_list1(tag(","), parse_lens_instruction))
        .parse(input) {
            Ok((_, val)) => val,
            Err(e) => panic!("{}", e),
        }
}

fn parse_lens_instruction(input: &str) -> IResult<&str, LensInstruction> {
    tuple((parse_label,parse_operation))
        .map(|(p_label, p_op)| LensInstruction{instruction: p_op, lens_label: p_label})
        .parse(input)
}

fn parse_label(input: &str) -> IResult<&str, String> {
    take_while1(char::is_alphabetic)
        .map(|str: &str| str.to_string())
        .parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Instruction> {
    alt((
            preceded(tag("="), parse_lens_assignment),
            value(Instruction::Remove, tag("-")),
            ))
        .parse(input)
}

fn parse_lens_assignment(input: &str) -> IResult<&str, Instruction> {
    parse_num_to_usize
        .map(|num| Instruction::Assign(num))
        .parse(input)
}

fn parse_num_to_usize(input: &str) -> IResult<&str, usize> {
    take_while1(char::is_numeric)
        .map(|num: &str| num.parse::<usize>().unwrap())
        .parse(input)
}

fn label_to_register_num(label: &str) -> usize {
    label.bytes()
        .fold(0usize, |acc,el| ((acc + el as usize) * 17) % 256)
}

/* Box number from part 1 */
fn hashmap_protocoll_for_p2<'a>(instructions: &'a [LensInstruction]) -> Vec<Vec<(&'a str,usize)>> {
    let label_locs = instructions.iter()
        .map(|el| (el.lens_label.as_str(), label_to_register_num(&el.lens_label)))
        .collect::<HashMap<&str, usize>>();
    let mut registers: Vec<Vec<(&str,usize)>> = vec![vec![]; 256];
    for instr in instructions.iter() {
        match instr.instruction {
            Instruction::Assign(focal_length) => {
                let reg_num = label_locs.get(instr.lens_label.as_str()).unwrap();
                let curr_register = &mut registers[*reg_num];
                if curr_register.iter().any(|(el,_)| *el == instr.lens_label.as_str()) {
                    /* search and replace */
                    let idx = curr_register
                        .iter()
                        .enumerate()
                        .filter(|(_,(label,_))| *label == instr.lens_label.as_str())
                        .map(|(num,_)| num)
                        .next().unwrap();
                    curr_register[idx] = (instr.lens_label.as_str(), focal_length);
                } else {
                    registers[*reg_num].push((instr.lens_label.as_str(), focal_length));
                }
            },
            Instruction::Remove => {
                /* remove if in box */
                let reg_num = label_locs.get(instr.lens_label.as_str()).unwrap();
                let curr_register = &mut registers[*reg_num];
                if curr_register.iter().any(|(label,_)| *label == instr.lens_label) {
                    let idx = curr_register
                        .iter()
                        .enumerate()
                        .filter(|(_, (label,_))| *label == instr.lens_label)
                        .map(|(num,_)| num)
                        .next().unwrap();
                    curr_register.remove(idx);
                }
            },
        }
    }
    registers
}

fn calc_lens_power(registers: Vec<Vec<(&str,usize)>>) -> u64 {
    registers.into_iter()
        .enumerate()
        .map(|(box_num,vec)| (box_num + 1, vec))
        .map(|(box_num,vec)| {
            let num = box_num * vec.into_iter().enumerate().map(|(i,(_,num))| num * (i + 1)).sum::<usize>();
            num
        })
    .sum::<usize>() as u64
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let instrs = parse_hash_instruction(input);
    let registers = hashmap_protocoll_for_p2(&instrs);
    calc_lens_power(registers)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_solve_day15_p1() {
        let ans = solve_part1(TEST_INPUT);
        assert_eq!(ans, 1320);
    }

    #[test]
    fn test_hashmap_protocoll() {
        let input = parse_hash_instruction(TEST_INPUT);
        let ans = hashmap_protocoll_for_p2(&input);
        assert_eq!(ans.len(), 256);
        assert_eq!(ans[0].len(), 2);
        assert_eq!(ans[3].len(), 3);
        assert_eq!(ans[0][0], ("rn", 1));
        assert_eq!(ans[3][1], ("ab", 5));
    }
    
    #[test]
    fn test_solve_day15_p2() {
        let ans = solve_part2(TEST_INPUT);
        assert_eq!(ans, 145);
    }
}
