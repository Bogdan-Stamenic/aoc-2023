use std::collections::HashMap;
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value},
    bytes::complete::{tag, take_while1, is_a},
    multi::separated_list1, sequence::{separated_pair, preceded, tuple, delimited}
};

#[allow(dead_code)]
#[derive(Debug)]
struct MachinePart{
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Clone,Copy,Debug)]
enum PartProperty {
    X,
    M,
    A,
    S
}

#[derive(Debug,PartialEq,Eq)]
enum WorkflowTest {
    LessThan(i32),
    GreaterThan(i32),
}

#[allow(dead_code)]
#[derive(Debug)]
struct WorkflowRule {
    property: PartProperty,
    test: WorkflowTest,
    jmp: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Workflow {
    rules: Vec<WorkflowRule>,
    default: String
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PartsAndWorkflows {
    workflows: HashMap<String,Workflow>,
    parts: Vec<MachinePart>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> PartsAndWorkflows {
    match all_consuming(separated_pair(parse_workflows, tag("\n\n"), parse_machine_parts))
        .map(|(x,y)| {
            let foo = x.into_iter().collect();
            PartsAndWorkflows {workflows: foo, parts: y}
        })
        .parse(input) {
        Ok((_,val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_workflows(input: &str) -> IResult<&str,Vec<(String,Workflow)>> {
    separated_list1(tag("\n"), parse_workflow_line).parse(input)
}

fn parse_workflow_line(input: &str) -> IResult<&str,(String,Workflow)> {
    tuple((
            take_while1(char::is_alphabetic),
            parse_workflow_rules_and_default,
            ))
        .map(|(the_name,(the_rules,def))| (the_name.to_string(), Workflow{rules: the_rules, default: def}))
        .parse(input)
}

fn parse_workflow_rules_and_default(input: &str) -> IResult<&str,(Vec<WorkflowRule>,String)> {
    delimited(
        tag("{"),
        separated_list1(tag(","), take_while1(|c: char| c.is_ascii_digit() ||
                c.is_ascii_alphabetic() || c == ':' || c == '>' || c == '<')),
        tag("}"))
        .map(|v: Vec<&str>| {
            let default = v.last().unwrap().to_string();
            let rules = parse_workflow_rules(&v[..v.len()-1]);
            (rules, default)
        })
        .parse(input)
}

fn parse_workflow_rules(input: &[&str]) -> Vec<WorkflowRule> {
    input.iter()
        .map(|x| {
            match tuple((
                    parse_property,
                    parse_workflow_test,
                    tag(":"),
                    take_while1(|c: char| c.is_ascii_alphabetic())
                    ))
                .map(|el| WorkflowRule {property: el.0, test: el.1, jmp: el.3.to_string()})
                .parse(x) {
                Ok((_,val)) => val,
                Err(e) => panic!("{}", e)
            }
        })
    .collect()
}

#[inline]
fn parse_property(input: &str) -> IResult<&str,PartProperty> {
    alt((
            value(PartProperty::X, tag("x")),
            value(PartProperty::M, tag("m")),
            value(PartProperty::A, tag("a")),
            value(PartProperty::S, tag("s")),
            ))
        .parse(input)
}

#[inline]
fn parse_workflow_test(input: &str) -> IResult<&str,WorkflowTest> {
    alt((
            preceded(tag("<"), parse_num_to_i32).map(|x| WorkflowTest::LessThan(x)),
            preceded(tag(">"), parse_num_to_i32).map(|x| WorkflowTest::GreaterThan(x)),
            ))
        .parse(input)
}

fn parse_num_to_i32(input: &str) -> IResult<&str, i32> {
    take_while1(|c: char| c.is_ascii_digit()).map(|x: &str| x.parse::<i32>().unwrap())
        .parse(input)
}

fn parse_machine_parts(input: &str) -> IResult<&str,Vec<MachinePart>> {
    separated_list1(tag("\n"), parse_one_machine_part).parse(input)
}

#[inline]
fn parse_one_machine_part(input: &str) -> IResult<&str,MachinePart> {
    delimited(
        tag("{"),
        separated_list1(
            tag(","),
            preceded(preceded(is_a("xmas"), tag("=")), parse_num_to_i32)),
        tag("}"))
        .map(|el| {
            let mut it = el.into_iter();
            MachinePart {x: it.next().unwrap(), m: it.next().unwrap(), a: it.next().unwrap(),
            s: it.next().unwrap()}
        })
    .parse(input)
}


//#[aoc(day19, part1)]
//pub fn solve_part1(input: &Almanac) -> i64 {
//}


//#[aoc(day19, part2)]
//pub fn solve_part2(input: &Almanac) -> i64 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn day19_input() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.parts.len(), 5);
        assert_eq!(input.workflows["px"].rules[0].test, WorkflowTest::LessThan(2006));
    }
}
