use std::collections::VecDeque;
use fnv::{FnvHashMap, FnvHashSet};
use nom::{
    Parser,
    IResult,
    combinator::all_consuming,
    bytes::complete::{tag, take},
    multi::separated_list1,
    sequence::separated_pair,
};

#[derive(Debug)]
pub struct MyGraph {
    adj_matrix: FnvHashMap<u32,Vec<u32>>,
}

#[allow(dead_code)]
impl MyGraph {
    fn nodes_to_edge_id(&self, node1: u32, node2: u32) -> u64 {
        if node1 > node2 {
            return ((node2 as u64) << 21) + node1 as u64;
        }
        ((node1 as u64) << 21) + node2 as u64
    }

    fn bfs(&self, start: u32) -> FnvHashMap<u32,u32> {
        let mut queue = VecDeque::<u32>::from([start]);
        /* (node, parent) */
        let mut explored = FnvHashMap::<u32,u32>::default();
        loop {
            let curr_node = match queue.pop_front() {
                Some(val) => val,
                None => break,
            };
            unsafe {// Should never fail by construction
                for w in self.adj_matrix.get(&curr_node).unwrap_unchecked().iter() {
                    if !explored.contains_key(&w) {
                        explored.insert(*w, curr_node);
                        queue.push_back(*w);
                    }
                }
            }
        }
        explored
    }

    fn bfs_furthest_node_from(&self, start: u32) -> u32 {
        let mut queue = VecDeque::<u32>::from([start]);
        /* (node, dist) */
        let mut explored = FnvHashMap::<u32,u32>::default();
        explored.insert(start, 0);
        loop {
            let curr_node = match queue.pop_front() {
                Some(val) => val,
                None => break,
            };
            unsafe {// Should never fail by construction
                for w in self.adj_matrix.get(&curr_node).unwrap_unchecked().iter() {
                    if !explored.contains_key(&w) {
                        explored.insert(*w, *explored.get(&curr_node).unwrap_unchecked() + 1);
                        queue.push_back(*w);
                    }
                }
            }
        }
        explored.into_iter()
            .max_by_key(|x| x.1)
            .expect("No elements found")
            .0
    }

    fn bfs_excluding_edges(&self, start: u32, excluded_edges: &FnvHashSet<u64>)
        -> FnvHashMap<u32,u32>
    {
        let mut queue = VecDeque::<u32>::from([start]);
        /* (node, parent) */
        let mut explored = FnvHashMap::<u32,u32>::default();
        loop {
            let curr_node = match queue.pop_front() {
                Some(val) => val,
                None => break,
            };
            unsafe {// Should never fail by construction
                for w in self.adj_matrix.get(&curr_node).unwrap_unchecked().iter() {
                    let is_excluded = excluded_edges
                        .contains(&self.nodes_to_edge_id(*w, curr_node));
                    if !explored.contains_key(&w) && !is_excluded {
                        explored.insert(*w, curr_node);
                        queue.push_back(*w);
                    }
                }
            }
        }
        explored
    }

    fn bfs_shortest_path_excluding_edges(&self, start: u32, goal: u32, excluded_edges: &FnvHashSet<u64>)
        -> Vec<u64>
    {
        let mut queue = VecDeque::<u32>::from([start]);
        /* (node, parent) */
        let mut explored = FnvHashMap::<u32,u32>::default();
        loop {
            let curr_node = match queue.pop_front() {
                Some(val) => val,
                None => break,
            };
            if curr_node == goal {
                break;
            }
            for w in self.adj_matrix.get(&curr_node).unwrap().iter() {
                let is_excluded = excluded_edges
                    .contains(&self.nodes_to_edge_id(*w, curr_node));
                if !explored.contains_key(&w) && !is_excluded {
                    explored.insert(*w, curr_node);
                    queue.push_back(*w);
                }
            }
        }
        let mut consumed_edges = Vec::<u64>::new();
        let mut node = goal;
        loop {
            let new_node = match explored.get_key_value(&node) {
                Some((_,val)) => *val,
                None => break,
            };
            let explored_edge = self.nodes_to_edge_id(node, new_node);
            consumed_edges.push(explored_edge);
            if node == start || new_node == start {
                break;
            }
            node = new_node;
        }
        consumed_edges
    }
}


#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> MyGraph {
    let nodes_from_input = match all_consuming(separated_list1(tag("\n"), parse_one_line))
    .parse(input) {
        Ok((_,val)) => val,
        Err(e) => panic!("While parsing : {}",e),
    };
    let nodes_and_edges = nodes_from_input
        .into_iter()
        .map(|(start,vec_nodes)| {
            let pairs = vec_nodes
                .iter()
                .map(|x: &u32| (start,*x));
            let pairs_rev = vec_nodes
                .iter()
                .map(|x: &u32| (*x,start));
            pairs
                .chain(pairs_rev)
                .collect::<Vec<(u32,u32)>>()
        })
    .flatten()
        .collect::<Vec<(u32,u32)>>();
    let mut out_graph = FnvHashMap::<u32,Vec<u32>>::default();
    nodes_and_edges.iter()
        .for_each(|(x,y)| {
            if !out_graph.contains_key(&x) {
                out_graph.insert(*x, vec![y.clone()]);
            } else {
                out_graph.get_mut(&x).unwrap()
                    .push(y.clone());
            }
        });
    MyGraph { adj_matrix: out_graph }
}

fn parse_one_line(input: &str) -> IResult<&str,(u32,Vec<u32>)> {
    separated_pair(parse_letters_to_id, tag(": "), separated_list1(tag(" "), parse_letters_to_id))
        .parse(input)
}

#[inline]
fn parse_letters_to_id(input: &str) -> IResult<&str,u32> {
    take(3usize).map(|x: &str| {
        x.bytes()
            .enumerate()
            .map(|(i,x)| (x as u32) << i*7)
            .sum::<u32>() - ((b'a' as u32) << 2*7)
    })
        .parse(input)
}

#[allow(dead_code)]
fn to_u32_id(input: &str) -> u32 {
    input.bytes()
        .enumerate()
        .map(|(i,x)| (x as u32) << i*7)
        .sum::<u32>() - ((b'a' as u32) << 2*7)
}

#[aoc(day25,part1)]
pub fn solve_p1(input: &MyGraph) -> usize {
    /* Find 2 nodes that are as far apart as possible -> on either side of the 3 edges */
    let arbitrary_node: u32 = *input.adj_matrix.iter().next().unwrap().0;
    let start_node: u32 = input.bfs_furthest_node_from(arbitrary_node);
    let end_node: u32 = input.bfs_furthest_node_from(start_node);
    let mut excluded_edges = FnvHashSet::<u64>::default();
    /* "Saturates" the 3 edges connecting the 2 graph regions */
    for _ in 0..3 {
        let edge_ids_of_shortest_path =
            input.bfs_shortest_path_excluding_edges(start_node, end_node, &excluded_edges);
        edge_ids_of_shortest_path.into_iter()
            .for_each(|x| {excluded_edges.insert(x);});
    }
    /* Excluding edges should now split graph into two disjointed regions -> count nodes with BFS */
    let some_count = input.bfs_excluding_edges(start_node, &excluded_edges).into_iter().count();
    let another_count = input.bfs_excluding_edges(end_node, &excluded_edges).into_iter().count();
    some_count * another_count
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str =
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn day25_letters_to_id() {
        const INPUT1: &str = "abc ";
        const INPUT2: &str = "bbc ";
        const INPUT3: &str = "cba ";
        let ans1 = parse_letters_to_id(INPUT1);
        let ans2 = parse_letters_to_id(INPUT2);
        let ans3 = parse_letters_to_id(INPUT3);
        assert_eq!(ans1, Ok((" ", 45409)));
        assert_eq!(ans2, Ok((" ", 45410)));
        assert_eq!(ans3, Ok((" ", 12643)));
        assert_ne!(ans1,ans2);
        assert_ne!(ans3,ans2);
        assert_ne!(ans1,ans3);
    }

    #[test]
    fn day25_bfs() {
        const TEST: &str = 
"abc: def ghi
def: jkl
ghi: mno
mno: pqr
pqr: stu
def: stu";
        let input = input_generator(TEST);
        let foo = input.bfs(to_u32_id("abc"));
        assert_eq!(foo.len(), 7);
        assert_eq!(*foo.get(&to_u32_id("stu")).unwrap(), to_u32_id("def"))
    }

    #[test]
    fn day25_bfs_furtherst_node() {
        const TEST: &str =
"abc: def ghi
jkl: def ghi";
        let input = input_generator(TEST);
        let foo = input.bfs_furthest_node_from(to_u32_id("abc"));
        assert_eq!(foo, to_u32_id("jkl"));
    }

    #[test]
    fn day25_bfs_excluding_edges() {
        const TEST: &str =
"abc: def ghi
jkl: def ghi";
        let input = input_generator(TEST);
        let mut excluded_edges = FnvHashSet::<u64>::default();
        excluded_edges.insert(input.nodes_to_edge_id(to_u32_id("abc"), to_u32_id("def")));
        let foo = input.bfs_shortest_path_excluding_edges(to_u32_id("abc"), to_u32_id("jkl"), &excluded_edges);
        assert_eq!(foo.len(), 2);
    }

    #[test]
    fn day25_solve_p1() {
        /* 9 comps: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, rzs
         * 6 comps: bvb, hfx, jqt, ntq, rhn, xhk
         * 6 * 9 = 54
         * */
        let input = input_generator(TEST_INPUT);
        let ans = solve_p1(&input);
        assert_eq!(ans, 54)
    }
}

