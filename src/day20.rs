use std::collections::VecDeque;
use fnv::FnvHashMap;
use num::integer::lcm;
use nom::{
IResult,
Parser,
branch::alt,
bytes::complete::tag,
combinator::{all_consuming, value},
multi::separated_list1,
sequence::{tuple, separated_pair},
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum ModuleType {
    Conjunction,
    FlipFlop,
    Broadcast
}

#[allow(dead_code)]
impl Module {

    fn apply_pulse(&mut self,incoming_pulse: Pulse,incoming_id: Option<usize>) -> Option<Pulse> {
        match self.module_type {
            ModuleType::Conjunction => self.apply_pulse_to_conjuction(incoming_pulse, incoming_id),
            ModuleType::FlipFlop => self.apply_pulse_to_flipflop(incoming_pulse),
            ModuleType::Broadcast => Some(incoming_pulse),
        }
    }

    fn apply_pulse_to_conjuction(&mut self,incoming_pulse: Pulse,incoming_id: Option<usize>) -> Option<Pulse> {
        *self.src_ids.get_mut(&incoming_id.expect("Should never fail")).unwrap() = incoming_pulse;
        if self.src_ids.iter().any(|x| *x.1 == Pulse::Low) {
            return Some(Pulse::High);
        }
        /* All stored pulses are high */
        Some(Pulse::Low)
    }

    fn apply_pulse_to_flipflop(&mut self,incoming_pulse: Pulse) -> Option<Pulse> {
        match incoming_pulse {
            Pulse::Low => {
                self.toggle_flip_flop();
                Some(self.state)
            },
            _ => None,
        }
    }

    fn toggle_flip_flop(&mut self) {
        match self.state {
            Pulse::Low => self.state = Pulse::High,
            Pulse::High => self.state = Pulse::Low,
        }
    }
}

#[derive(Clone,Debug)]
struct Module {// &: conjunction, %: flip-flop
    id: usize,
    module_type: ModuleType,
    src_ids: FnvHashMap<usize,Pulse>,
    state: Pulse,
}

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct ModuleNetwork {
    lookup: Vec<usize>,
    modules: Vec<Module>,
    children: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl ModuleNetwork {
    fn solve_p1(&mut self) -> usize {
        let mut low_pulse_counter = 0;
        let mut high_pulse_counter = 0;
        for _ in 0..1_000 {
            self.push_button_p1(&mut low_pulse_counter, &mut high_pulse_counter);
        }
        low_pulse_counter * high_pulse_counter
    }

    /* This solution is thanks to insights from the Reddit mega-thread. The entire network models
     * a mod m counter circuit with 4 bits (at least, my input does), with broadcast as our incoming
     * clock signal. Each bit has a different periodicity which we need to find when the counter
     * resets, i.e. m + 1, because that's exactly when rx will receive a low pulse.
     * This graph illustrates it all quite well (not my input):
     * https://www.reddit.com/media?url=https%3A%2F%2Fi.redd.it%2F69qgom9ylg7c1.png
     *
     * Find the periodicity of each sub-circuit and then compute their LCM. */
    fn solve_p2(&self) -> usize {
        let lsb_nodes = self.lookup_children(0);//nodes after broadcaster
        let mut counter_periods = Vec::<usize>::with_capacity(lsb_nodes.len());
        for start in lsb_nodes.iter() {
            let period = match self.find_mod_m_counter_period(*start) {
                Ok(val) => val,
                Err(_) => panic!("Assumption didn't hold! Graph isn't mod m counter."),
            };
            counter_periods.push(period);
        }
        counter_periods.into_iter()
            .reduce(|acc,el| lcm(acc, el)).unwrap()
    }
    
    fn find_mod_m_counter_period(&self, start: usize) -> Result<usize,()> {
        let mut counter_period: usize = 0;
        let mut bit_number = 0;
        let mut queue = Vec::<usize>::from([start]);
        loop {
            let curr = match queue.pop() {
                Some(val) => val,
                None => break,
            };
            let childr = self.lookup_children(curr).iter()
                .map(|x| self.lookup_module(*x).expect("Shouldn't fail"));
            /* Check if our assumption holds: should never have more than 1 flip-flop as child */
            if childr.clone().filter(|x| x.module_type == ModuleType::FlipFlop)
                .count() > 1 {
                    return Err(());
            }
            if childr.clone().any(|x| x.module_type == ModuleType::Conjunction) {
                    counter_period += 1 << bit_number;
            }
            /* Push next flip-flop to queue */
            let next_node = childr.filter(|x| x.module_type == ModuleType::FlipFlop)
                .map(|x| x.id).next();
            match next_node {
                Some(val) => queue.push(val),
                None => {},
            }
            bit_number += 1;
        }
        Ok(counter_period)
    }

    fn push_button_p1(&mut self, low_pulse_counter: &mut usize, high_pulse_counter: &mut usize) {
        let mut queue = VecDeque::<(Option<usize>,usize,Pulse)>::with_capacity(500);
        queue.push_back((None,0usize,Pulse::Low));//Button -Low-> Broadcaster
        loop {
            let (src_id,curr_id,pulse_t) = match queue.pop_front() {
                Some(val) => val,
                None => break,
            };
            /* Count types of pulses in network */
            match pulse_t {
                Pulse::Low => *low_pulse_counter += 1,
                Pulse::High => *high_pulse_counter += 1,
            };
            /* Apply pulse to current module */
            let curr_mod = self.lookup_module_mut(curr_id);
            /* None only ever occurs if we have a module with no outgoing edges, e.g. the output
             * node "ou" in test example 2*/
            if curr_mod.is_some() {
                match curr_mod.unwrap().apply_pulse(pulse_t, src_id) {
                    Some(out_pulse) => {
                        /* Add any outgoing pulses to queue */
                        for new_dest in self.lookup_children(curr_id).into_iter() {
                            queue.push_back((Some(curr_id),*new_dest,out_pulse));
                        }
                    },
                    None => {}
                }
            }
        }
    }

    #[inline]
    fn lookup_children(&self, id: usize) -> &[usize] {
        let index = self.lookup[id];
        &self.children[index]
    }

    #[inline]
    fn lookup_module(& self, id: usize) -> Option<&Module> {
        let index = self.lookup[id];
        self.modules.get(index)
    }

    #[inline]
    fn lookup_module_mut(&mut self, id: usize) -> Option<&mut Module> {
        let index = self.lookup[id];
        self.modules.get_mut(index)
    }

}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> ModuleNetwork {
    let parsed_modules = match all_consuming(separated_list1(tag("\n"), parse_one_line))
        .parse(input) {
            Ok((_,val)) => val,
            Err(e) => panic!("While parsing {}", e),
        };
    /* Generate graph for module network */
    let mut mlookup = vec![usize::MAX; 26*26 + 1];
    let mut mchildren = Vec::<Vec<usize>>::with_capacity(100);
    let mut mparents = FnvHashMap::<usize,Vec<usize>>::default();
    let mut mods = Vec::with_capacity(100);
    for (i,(module,childr)) in parsed_modules.iter().enumerate() {
        mlookup[module.id] = i;
        mchildren.push(childr.clone());
        mods.push(module.clone());
        for ch in childr.into_iter() {
            if !mparents.contains_key(&ch) {
                mparents.insert(*ch, Vec::with_capacity(10));
            }
            let src = mparents.get_mut(&ch).unwrap();
            src.push(module.id);
        }
    }
    /* Add source modules for conjunction modules */
    for m in mods.iter_mut().filter(|x| x.module_type == ModuleType::Conjunction) {
        let sources = mparents.get(&m.id).unwrap();
        sources.iter()
            .for_each(|x| {
                m.src_ids.insert(*x,Pulse::Low);
            });
    }
    ModuleNetwork {modules: mods, lookup: mlookup, children: mchildren}
}

fn parse_module_id(input: &str) -> IResult<&str, usize> {
    /* Represent two-letter id as base 26 number */
    let out_id = input[..2]
        .bytes()
        .fold(0u16, |acc,b| acc*26 + (b - b'a') as u16) + 1;
    Ok((&input[2..],out_id as usize))
}

fn parse_one_line(input: &str) -> IResult<&str,(Module,Vec<usize>)> {
    separated_pair(parse_src_module, tag(" -> "), parse_dest_modules)
        .parse(input)
}

fn parse_src_module(input: &str) -> IResult<&str,Module> {
    alt((
            parse_conjunction_module,
            parse_flipflop_module,
            parse_broadcaster_module,
            ))
        .parse(input)
}

fn parse_conjunction_module(input: &str) -> IResult<&str,Module> {
    tuple((
            value(ModuleType::Conjunction, tag("&")),
            parse_module_id
            ))
        .map(|(mtype,mid)| Module { id: mid, module_type: mtype,
        src_ids: FnvHashMap::<usize,Pulse>::default(), state: Pulse::High })
        .parse(input)
}

fn parse_flipflop_module(input: &str) -> IResult<&str,Module> {
    tuple((
            value(ModuleType::FlipFlop, tag("%")),
            parse_module_id
            ))
        .map(|(mtype,mid)| Module { id: mid, module_type: mtype,
        src_ids: FnvHashMap::<usize,Pulse>::default(), state: Pulse::Low })
        .parse(input)
}

fn parse_broadcaster_module(input: &str) -> IResult<&str,Module> {
    value(ModuleType::Broadcast, tag("broadcaster"))
        .map(|mtype| Module { id: 0usize, module_type: mtype,
        src_ids: FnvHashMap::<usize,Pulse>::default(), state: Pulse::Low })
        .parse(input)
}

fn parse_dest_modules(input: &str) -> IResult<&str,Vec<usize>> {
    separated_list1(tag(", "), parse_module_id)
        .parse(input)
}

#[aoc(day20,part1)]
pub fn solve_day20_p1(input: &ModuleNetwork) -> usize {
    let mut graph = input.clone();
    graph.solve_p1()
}

#[aoc(day20,part2)]
pub fn solve_day20_p2(input: &ModuleNetwork) -> usize {
    input.solve_p2()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT1: &str =
"broadcaster -> aa, bb, cc
%aa -> bb
%bb -> cc
%cc -> iv
&iv -> aa";

    const TEST_INPUT2: &str =
"broadcaster -> aa
%aa -> iv, co
&iv -> bb
%bb -> co
&co -> ou";

    #[test]
    fn day20_input_generator() {
        let input = input_generator(TEST_INPUT1);
        assert_eq!(input.modules.len(),5);
    }

    #[test]
    fn day20_solve_p1_1() {
        let input = input_generator(TEST_INPUT1);
        let ans = solve_day20_p1(&input);
        assert_eq!(ans, 32_000_000)
    }

    #[test]
    fn day20_solve_p1_2() {
        let input = input_generator(TEST_INPUT2);
        let ans = solve_day20_p1(&input);
        assert_eq!(ans, 11_687_500)
    }

}
