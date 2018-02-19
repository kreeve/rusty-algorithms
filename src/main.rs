use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
#[macro_use] extern crate itertools;
use itertools::Itertools;
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
struct Node<T: Hash + Eq + Copy> {
    id: T
}

#[derive(Debug,PartialEq,Eq,Hash)]
struct Edge<T: Hash+Eq+Copy> {
    source: Node<T>,
    dest: Node<T>,
    weight: i32
}

#[derive(Debug)]
struct Graph<T: Hash+Eq+Copy> {
    edges: HashSet<Edge<T>>
}

#[derive(Debug)]
struct Clock(pub i32);

#[derive(Debug,PartialEq,Eq,Hash)]
struct NodeInfo<T: Hash+Eq+Copy> {
    node: Node<T>,
    postorder: i32
}

fn nodes<T: Hash+Eq+Copy>(graph: &Graph<T>) -> HashSet<Node<T>> {
    let mut all_nodes = HashSet::new();
    for e in &graph.edges {
        all_nodes.insert(e.source);
        all_nodes.insert(e.dest);
    }
    all_nodes
}

fn explore<T: Hash+Eq+Copy>(graph: &Graph<T>, start: Node<T>, visited: &mut HashSet<Node<T>>, clock: &mut Clock) -> HashSet<NodeInfo<T>> {
    let mut result = HashSet::new();
    visited.insert(start);
    clock.0 += 1;
    for e in &graph.edges {
        if e.source == start {
            if (!visited.contains(&e.dest)) {
                let next_layer = explore(graph, e.dest, visited, clock);
                for ni in next_layer {
                    result.insert(ni);
                }
            }
        }        
    }
    result.insert(NodeInfo {node: start, postorder: clock.0});
    clock.0 += 1;
    result
}

fn dfs_traverse<T: Hash+Eq+Copy>(graph: &Graph<T>) -> HashSet<NodeInfo<T>> {
    let mut visited = HashSet::new();
    let mut clock = Clock(1);
    let mut result = HashSet::new();
    for node in nodes(graph) {
        if !visited.contains(&node) {
            let dfs_res = explore(&graph, node, &mut visited, &mut clock);
            for ni in dfs_res {
                result.insert(ni);
            }
        }

    }
    result
}

fn postorder<T: Hash+Eq+Copy>(node: &Node<T>, dfs: &HashSet<NodeInfo<T>>) -> i32 {
    let mut res: i32 = 0;
    for m in dfs {
        if &m.node == node {
            res = m.postorder;
        }
    }
    res
}

fn back_edges<'a, T: Hash+Eq+Copy>(graph: &'a Graph<T>, dfs: HashSet<NodeInfo<T>>) -> HashSet<&'a Edge<T>> {
    let mut edges = HashSet::new();

    for edge in &graph.edges {
        if postorder(&edge.source, &dfs) < postorder(&edge.dest, &dfs) {
            edges.insert(edge);
        }
    }
    edges
}

fn has_cycles<T: Hash+Eq+Copy>(graph: &Graph<T>) -> bool {
    let dfs = dfs_traverse(graph);

    back_edges(graph, dfs).len() > 0
}

fn is_dag<T: Hash+Eq+Copy>(graph: &Graph<T>) -> bool {
    !has_cycles(graph)
}

fn top_sort<T: Hash+Eq+Copy>(graph: &Graph<T>) ->  Vec<NodeInfo<T>> {
    let dfs = dfs_traverse(graph);
    dfs
        .into_iter()
        .sorted_by(|ref a,ref b| b.postorder.cmp(&a.postorder))
}

fn main() {
    let A = Node{id: 'A'};
    let B = Node{id: 'B'};
    let C = Node{id: 'C'};
    let D = Node{id: 'D'};
    let E = Node{id: 'E'};
    let F = Node{id: 'F'};
    let G = Node{id: 'G'};
    let H = Node{id: 'H'};
    
    let mut edges = HashSet::new();
    edges.insert(Edge {source: A,dest: B,weight: 0});
    
    edges.insert(Edge {source: B,dest: C,weight: 0});

    edges.insert(Edge {source: C, dest: D, weight: 0});

    
    let gr = Graph {edges: edges};
    let nodeset = top_sort(&gr);
    println!("{:?}", nodeset);
    println!("{:?}", is_dag(&gr));
}
