use std::{collections::HashMap, cmp::{min,max}};
use smallstr::SmallString;
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value},
    bytes::complete::{tag, take_while1, is_a},
    multi::separated_list1, sequence::{separated_pair, preceded, tuple, delimited}
};

#[derive(Debug)]
struct MachinePart{
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl MachinePart {
    fn xmas_sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone,Copy,Debug)]
struct MachinePartRange {
    /* [a; b) */
    x: (i64,i64),
    m: (i64,i64),
    a: (i64,i64),
    s: (i64,i64),
}

impl MachinePartRange {
    fn count_possible(&self) -> i64 {
        (self.x.1 - self.x.0)
        * (self.m.1 - self.m.0)
        * (self.a.1 - self.a.0)
        * (self.s.1 - self.s.0)
    }

    fn any_ranges_empty(&self) -> bool {
        (self.x.1 - self.x.0 == 0)
        || (self.m.1 - self.m.0 == 0)
        || (self.a.1 - self.a.0 == 0)
        || (self.s.1 - self.s.0 == 0)
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        println!("{} * {} * {} * {}",
            self.x.1 - self.x.0,
            self.m.1 - self.m.0,
            self.a.1 - self.a.0,
            self.s.1 - self.s.0,
            );
    }
}

#[derive(Clone,Copy,Debug)]
enum PartProperty {
    X,
    M,
    A,
    S
}

#[derive(Debug,PartialEq,Eq)]
enum TestResult {
    Jump(SmallString<[u8;4]>),
    Continue,
}

#[derive(Debug,PartialEq,Eq)]
enum WorkflowTest {
    LessThan(i64),
    GreaterThan(i64),
}

#[allow(dead_code)]
#[derive(Debug)]
struct WorkflowRule {
    property: PartProperty,
    test: WorkflowTest,
    jmp: SmallString<[u8;4]>,
}

impl WorkflowRule {
    fn apply(&self, input: &MachinePart) -> TestResult {
        let passes_test = match self.property {
            PartProperty::X => self.apply_rule(input.x),
            PartProperty::M => self.apply_rule(input.m),
            PartProperty::A => self.apply_rule(input.a),
            PartProperty::S => self.apply_rule(input.s),
        };
        if passes_test {
            return TestResult::Jump(self.jmp.clone().clone());
        }
        TestResult::Continue
    }

    fn apply_rule(&self, prop: i64) -> bool {
        match self.test {
            WorkflowTest::LessThan(val) => {prop < val},
            WorkflowTest::GreaterThan(val) => {prop > val},
        }
    }

    fn split_range(&self, input: &mut MachinePartRange, stack: &mut Vec<(SmallString<[u8;4]>,MachinePartRange)>)
    {
        match self.property
        {
            PartProperty::X => self.split_range_by_x(input, stack),
            PartProperty::M => self.split_range_by_m(input, stack),
            PartProperty::A => self.split_range_by_a(input, stack),
            PartProperty::S => self.split_range_by_s(input, stack),
        }
    }

    fn split_range_by_x<'a>(&'a self, input: &'a mut MachinePartRange, stack: &'a mut Vec<(SmallString<[u8;4]>,MachinePartRange)>) {
        let new_x_range = match self.test {
            WorkflowTest::LessThan(val) => {
                let below_range = self.calc_below_range(input.x.0, input.x.1, val);
                let above_range = self.calc_above_range(input.x.0, input.x.1, val);
                if below_range.1 - below_range.0 > 0 {
                        let foo = MachinePartRange {x: below_range, m: input.m.clone(),  a: input.a.clone(), s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                above_range
            },
            WorkflowTest::GreaterThan(val) => {
                let below_range = self.calc_below_range(input.x.0, input.x.1, val+1);
                let above_range = self.calc_above_range(input.x.0, input.x.1, val+1);
                if above_range.1 - above_range.0 > 0 {
                        let foo = MachinePartRange {x: above_range, m: input.m.clone(),  a: input.a.clone(), s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                below_range
            }
        };
        input.x = new_x_range;
    }

    fn split_range_by_m<'a>(&'a self, input: &'a mut MachinePartRange, stack: &'a mut Vec<(SmallString<[u8;4]>,MachinePartRange)>) {
        let new_m_range = match self.test {
            WorkflowTest::LessThan(val) => {
                let below_range = self.calc_below_range(input.m.0, input.m.1, val);
                let above_range = self.calc_above_range(input.m.0, input.m.1, val);
                if below_range.1 - below_range.0 > 0 {
                        let foo = MachinePartRange {x: input.x.clone(), m: below_range,  a: input.a.clone(), s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                above_range
            },
            WorkflowTest::GreaterThan(val) => {
                let below_range = self.calc_below_range(input.m.0, input.m.1, val+1);
                let above_range = self.calc_above_range(input.m.0, input.m.1, val+1);
                if above_range.1 - above_range.0 > 0 {
                        let foo = MachinePartRange {x: input.x.clone(), m: above_range,  a: input.a.clone(), s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                below_range
            }
        };
        input.m = new_m_range;
    }

    fn split_range_by_a<'a>(&'a self, input: &'a mut MachinePartRange, stack: &'a mut Vec<(SmallString<[u8;4]>,MachinePartRange)>) {
        let new_a_range = match self.test {
            WorkflowTest::LessThan(val) => {
                let below_range = self.calc_below_range(input.a.0, input.a.1, val);
                let above_range = self.calc_above_range(input.a.0, input.a.1, val);
                if below_range.1 - below_range.0 > 0 {
                        let foo = MachinePartRange {x: input.x.clone(), m: input.m.clone(),  a: below_range, s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                above_range
            },
            WorkflowTest::GreaterThan(val) => {
                let below_range = self.calc_below_range(input.a.0, input.a.1, val+1);
                let above_range = self.calc_above_range(input.a.0, input.a.1, val+1);
                if above_range.1 - above_range.0 > 0 {
                        let foo = MachinePartRange {x: input.x.clone(), m: input.m.clone(),  a: above_range, s: input.s.clone()};
                        stack.push((self.jmp.clone(),foo));
                };
                below_range
            }
        };
        input.a = new_a_range;
    }

    fn split_range_by_s<'a>(&'a self, input: &'a mut MachinePartRange, stack: &'a mut Vec<(SmallString<[u8;4]>,MachinePartRange)>) {
        let new_s_range = match self.test {
            WorkflowTest::LessThan(val) => {
                let below_range = self.calc_below_range(input.s.0, input.s.1, val);
                let above_range = self.calc_above_range(input.s.0, input.s.1, val);
                if below_range.1 - below_range.0 > 0 {
                        let foo = MachinePartRange {x: input.x.clone(), m: input.m.clone(),  a: input.a.clone(), s: below_range};
                        stack.push((self.jmp.clone(),foo));
                };
                above_range
            },
            WorkflowTest::GreaterThan(val) => {
                let below_range = self.calc_below_range(input.s.0, input.s.1, val+1);
                let above_range = self.calc_above_range(input.s.0, input.s.1, val+1);
                if above_range.1 - above_range.0 > 0 {
                    let foo = MachinePartRange {x: input.x.clone(), m: input.m.clone(),  a: input.a.clone(), s: above_range};
                        stack.push((self.jmp.clone(),foo));
                };
                below_range
            }
        };
        input.s = new_s_range;
    }

    #[inline]
    fn calc_below_range(&self, a: i64, b: i64, d: i64) -> (i64,i64) {
        (min(a,d), min(b,d))
    }

    #[inline]
    fn calc_above_range(&self, a: i64, b: i64, d: i64) -> (i64,i64) {
        (max(a,d), max(b,d))
    }

}

#[allow(dead_code)]
#[derive(Debug)]
struct Workflow {
    rules: Vec<WorkflowRule>,
    default: SmallString<[u8;4]>
}

#[allow(dead_code)]
impl Workflow {
    fn apply(&self, input: &MachinePart) -> SmallString<[u8;4]> {
        let mut next_workflow = self.default.clone();
        for rule in self.rules.iter() {
            match rule.apply(input) {
                TestResult::Jump(val) => {
                    next_workflow = val;
                    break;
                },
                TestResult::Continue => {},
            }
        }
        next_workflow
    }

    fn map_range<'a>(&'a self,
        input: &'a mut MachinePartRange,
        stack: &'a mut Vec<(SmallString<[u8;4]>,MachinePartRange)>) {
        for rule in self.rules.iter() {
            rule.split_range(input,stack);
            if input.any_ranges_empty() {
                return;
            }
        }
        stack.push((self.default.clone(),*input))
    }
}

#[derive(Debug)]
pub struct PartsAndWorkflows {
    workflows: HashMap<SmallString<[u8;4]>,Workflow>,
    parts: Vec<MachinePart>,
}

impl PartsAndWorkflows {
    fn count_accepted_parts_p1(&self) -> i64 {
        self.parts
            .iter()
            .filter(|x| self.is_acceptable_p1(x))
            .map(|x| x.xmas_sum())
            .sum()
    }

    fn is_acceptable_p1(&self, part: &MachinePart) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            let next_w = workflow.apply(part);
            if next_w == "A".to_string() {
                return true;
            }
            if next_w == "R".to_string() {
                return false;
            }
            workflow = match self.workflows.get(&next_w)  {
                Some(v) => v,
                None => panic!("Workflows reached dead end"),
            };
        }
    }

    fn count_possibilites(&self) -> i64 {
        let mut stack = vec![
            (SmallString::<[u8;4]>::from("in"), MachinePartRange {x: (1,4001), m: (1,4001), a: (1,4001), s: (1,4001)})
        ];
        stack.reserve(1000);
        let mut accepted_count: i64 = 0;
        loop {
            let (next_workflow, mut curr_range) = match stack.pop() {
                Some(val) => val,
                None => break,
            };
            if next_workflow == "A" {
                let inc = curr_range.count_possible();
                accepted_count += inc;
                continue;
            }
            if next_workflow == "R" {
                continue;
            }
            let workflow = self.workflows.get(&next_workflow).unwrap();
            workflow.map_range(&mut curr_range, &mut stack);
        }
        accepted_count
    }
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

fn parse_workflows(input: &str) -> IResult<&str,Vec<(SmallString<[u8;4]>,Workflow)>> {
    separated_list1(tag("\n"), parse_workflow_line).parse(input)
}

fn parse_workflow_line(input: &str) -> IResult<&str,(SmallString<[u8;4]>,Workflow)> {
    tuple((
            take_while1(char::is_alphabetic),
            parse_workflow_rules_and_default,
            ))
        .map(|(the_name,(the_rules,def))| (SmallString::<[u8;4]>::from(the_name.to_string()), Workflow{rules: the_rules, default: def}))
        .parse(input)
}

fn parse_workflow_rules_and_default(input: &str) -> IResult<&str,(Vec<WorkflowRule>,SmallString<[u8;4]>)> {
    delimited(
        tag("{"),
        separated_list1(tag(","), take_while1(|c: char| c.is_ascii_digit() ||
                c.is_ascii_alphabetic() || c == ':' || c == '>' || c == '<')),
        tag("}"))
        .map(|v: Vec<&str>| {
            let default = SmallString::<[u8;4]>::from(v.last().unwrap().to_string());
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
                .map(|el| WorkflowRule {property: el.0, test: el.1, jmp: el.3.into()})
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
            preceded(tag("<"), parse_num_to_i64).map(|x| WorkflowTest::LessThan(x)),
            preceded(tag(">"), parse_num_to_i64).map(|x| WorkflowTest::GreaterThan(x)),
            ))
        .parse(input)
}

fn parse_num_to_i64(input: &str) -> IResult<&str, i64> {
    take_while1(|c: char| c.is_ascii_digit()).map(|x: &str| x.parse::<i64>().unwrap())
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
            preceded(preceded(is_a("xmas"), tag("=")), parse_num_to_i64)),
        tag("}"))
        .map(|el| {
            let mut it = el.into_iter();
            MachinePart {x: it.next().unwrap(), m: it.next().unwrap(), a: it.next().unwrap(),
            s: it.next().unwrap()}
        })
    .parse(input)
}


#[aoc(day19, part1)]
pub fn solve_part1(input: &PartsAndWorkflows) -> i64 {
    input.count_accepted_parts_p1()
}


#[aoc(day19, part2)]
pub fn solve_part2(input: &PartsAndWorkflows) -> i64 {
    input.count_possibilites()
}

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
/*
 * in -> qqz -> hdj -> pv -> A
 * (s >= 1351,s <= 2770, m < 1801, m <= 838,a <= 1716)
 * 4000 * 838 * 1716 * (2771 - 1351)
 *
 * in -> qqz -> hdj -> A 
 * (s >= 1351,s <= 2770, m < 1801, m > 838)
 * 4000 * (1801 - 839) * 4000 * (2771 - 1351)
 *
 * in -> qqz -> qs -> lnx -> A
 * (s >= 1351,s > 2770,s <= 3448,m <= 1548)
 * ???
 *
 * in -> qqz -> qs -> lnx -> A
 * (s >= 1351,s > 2770,s <= 3448,m > 1548)
 * ???
 *
 * in -> qqz -> qs -> A
 * (s >= 1351,s > 2770,s > 3448)
 * 4000 * 4000 * 4000 * (4001 - 3449)
 *
 * in -> px -> rfg -> A
 * (s < 1351, a >= 2006, m <= 2090, s >= 537, x <= 2440)
 * 2440 * 2090 * (4001 - 2006) * (1351 - 537)
 *
 * in -> px -> A
 * (s < 1351,a >= 2006, m > 2090)
 * 4000 * (4001 - 2091) * (4001 - 2006) * 1350
 *
 * in -> px -> qkq -> crn -> A
 * (s < 1351,a < 2006,x >= 1416,x > 2662)
 * (4001 - 2663) * 4000 * 2005 * 1350
 *
 * in -> px -> qkq -> A 
 * (s < 1351,a < 2006,x < 1416) 
 * 1415 * 4000 * 2005 * 1350
 *
 * */

    #[test]
    fn day19_input() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.parts.len(), 5);
        assert_eq!(input.workflows["px"].rules[0].test, WorkflowTest::LessThan(2006));
    }

    #[test]
    fn day19_solve_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 19114);
    }

    #[test]
    fn day19_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part2(&input);
        assert_eq!(ans, 167409079868000);
    }

}
