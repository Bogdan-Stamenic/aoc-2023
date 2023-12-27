use std::collections::HashSet;
use petgraph::{
    graphmap::DiGraphMap,
    visit::Bfs,
};

#[derive(Clone,Copy,Debug)]
enum Optics {
    MirrorSlash,      // /
    MirrorBackslash,  // \
    VertSplitter,     // |
    HorzSplitter,     // -
    Open,             // .
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct MirrorNode {
    coords: (usize,usize),
    beam_from: IncidenceDirection,
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum IncidenceDirection {
    Above,
    Below,
    Left,
    Right,
    Source,
}

#[derive(Clone,Debug)]
pub struct Mirrors {
    graph: DiGraphMap<MirrorNode,()>,
    outer_max: usize,
    inner_max: usize,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Mirrors {
    let optics_fixtures = input_to_optics(input);
    let out_graph = build_mirror_graph(&optics_fixtures);
    Mirrors {graph: out_graph, outer_max: optics_fixtures.len() + 1, inner_max: optics_fixtures[0].len() + 1}
}

fn input_to_optics(input: &str) -> Vec<Vec<Optics>> {
    input.lines()
        .map(|line| {
            line
                .bytes()
                .map(|c| match c {
                    b'|' => Optics::VertSplitter,
                    b'/' => Optics::MirrorSlash,
                    b'\\' => Optics::MirrorBackslash,
                    b'-' => Optics::HorzSplitter,
                    b'.' => Optics::Open,
                    _ => unreachable!("Unexpected char")
                })
            .collect()
        })
    .collect()
}

fn build_mirror_graph(optics: &Vec<Vec<Optics>>) -> DiGraphMap<MirrorNode,()> {
    let outer_max = optics.len();
    let inner_max = optics[0].len();
    let mut out_graph = DiGraphMap::<MirrorNode,()>::new();
    for (i,line) in optics.iter().enumerate() {
        for (j,val) in line.iter().enumerate() {
            /* Shift the grid and surround it with source-nodes at i=0, j=0, i=outer_max+1,
            * j=inner_max+1*/
            let i_new = i+1;
            let j_new = j+1;
            match *val {
                Optics::MirrorBackslash => connect_edges_to_mirror_backslash(&mut out_graph, i_new, j_new, outer_max, inner_max),
                Optics::MirrorSlash => connect_edges_to_mirror_slash(&mut out_graph, i_new, j_new, outer_max, inner_max),
                Optics::VertSplitter => connect_edges_to_vert_splitter(&mut out_graph, i_new, j_new, outer_max, inner_max),
                Optics::HorzSplitter => connect_edges_to_horz_splitter(&mut out_graph, i_new, j_new, outer_max, inner_max),
                Optics::Open => connect_edges_to_open(&mut out_graph, i_new, j_new, outer_max, inner_max),
            }
        }
    }
    out_graph
}

#[inline]
fn connect_to_source(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    if i == 1 {
        let src_node = MirrorNode {coords: (i-1, j), beam_from: IncidenceDirection::Source};
        let curr_node = MirrorNode{coords: (i,j), beam_from: IncidenceDirection::Above};
        graph.add_edge(src_node, curr_node, ());
    }
    if j == 1 {
        let src_node = MirrorNode {coords: (i, j-1), beam_from: IncidenceDirection::Source};
        let curr_node = MirrorNode{coords: (i,j), beam_from: IncidenceDirection::Left};
        graph.add_edge(src_node, curr_node, ());
    }
    if i == outer_max {
        let src_node = MirrorNode {coords: (i+1, j), beam_from: IncidenceDirection::Source};
        let curr_node = MirrorNode{coords: (i,j), beam_from: IncidenceDirection::Below};
        graph.add_edge(src_node, curr_node, ());
    }
    if j == inner_max {
        let src_node = MirrorNode {coords: (i,j+1), beam_from: IncidenceDirection::Source};
        let curr_node = MirrorNode{coords: (i,j), beam_from: IncidenceDirection::Right};
        graph.add_edge(src_node, curr_node, ());
    }
}

fn connect_edges_to_mirror_slash(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    connect_to_source(graph, i, j, outer_max, inner_max);
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
}

fn connect_edges_to_mirror_backslash(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    connect_to_source(graph, i, j, outer_max, inner_max);
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
}

fn connect_edges_to_vert_splitter(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    connect_to_source(graph, i, j, outer_max, inner_max);
    /*Split when coming from left */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
    /*Split when coming from right */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
    /* Let it pass straight through when coming from above or below */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
}

fn connect_edges_to_horz_splitter(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    connect_to_source(graph, i, j, outer_max, inner_max);
    /*Split when coming from below */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
    /*Split when coming from above */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
    /* Let it pass straight through when coming from the left or right */
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
}

fn connect_edges_to_open(graph: &mut DiGraphMap<MirrorNode,()>, i: usize, j: usize, outer_max: usize, inner_max: usize) {
    connect_to_source(graph, i, j, outer_max, inner_max);
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Below },
        MirrorNode { coords: (i-1,j), beam_from: IncidenceDirection::Below },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Above },
        MirrorNode { coords: (i+1,j), beam_from: IncidenceDirection::Above },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Left },
        MirrorNode { coords: (i,j+1), beam_from: IncidenceDirection::Left },
        ());
    graph.add_edge(
        MirrorNode { coords: (i,j), beam_from: IncidenceDirection::Right },
        MirrorNode { coords: (i,j-1), beam_from: IncidenceDirection::Right },
        ());
}


fn count_illuminated_tiles(gr: &DiGraphMap<MirrorNode,()>, om: usize, im: usize, start_node: MirrorNode) -> usize {
    let mut bfs = Bfs::new(gr, start_node);
    let mut illuminated = HashSet::<(usize,usize)>::new();
    loop {
        match bfs.next(gr) {
            Some(mnode) => {
                illuminated.insert(mnode.coords);
            },
            None => break,
        }
    }
    illuminated.iter()
        .filter(|(i,j)| (*i != 0) && (*j != 0))
        .filter(|(i,j)| (*i != om) && (*j != im))
        .count()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Mirrors) -> usize {
    let Mirrors {graph: gr, outer_max:om, inner_max:im} = input;
    let start_node = MirrorNode { coords: (1,0), beam_from: IncidenceDirection::Source };
    count_illuminated_tiles(&gr, *om, *im, start_node)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Mirrors) -> usize {
    let Mirrors {graph: gr, outer_max:om, inner_max:im} = input;
    let top_sources = (1..*im-1).map(|x| (0, x));
    let left_sources = (1..*om-1).map(|x| (x,0));
    let right_sources = (1..*om-1).map(|x| (x,*im));
    let bottom_sources = (1..*im-1).map(|x| (*om,x));
    let src_nodes = top_sources.chain(left_sources).chain(right_sources).chain(bottom_sources);
    src_nodes.map(|x|
        count_illuminated_tiles(&gr, *om, *im, MirrorNode { coords: x, beam_from: IncidenceDirection::Source })
        )
        .max().expect("Couldn't count any")
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn day16_solve_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 100);
    }

    //#[test]
    //fn day16_solve_p2() {
    //}
}
