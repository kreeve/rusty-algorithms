use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Eq;
#[macro_use] extern crate itertools;
use itertools::Itertools;
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
struct Node<T: Hash + Eq + Copy> {
    id: T
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
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
    parent: Option<Node<T>>,
    postorder: i32
}


fn explore<T: Hash+Eq+Copy>(graph: &Graph<T>, start: Node<T>, visited: &mut HashSet<Node<T>>, clock: &mut Clock, parent: Option<Node<T>>) -> HashSet<NodeInfo<T>> {
    let mut result = HashSet::new();
    visited.insert(start);
    clock.0 += 1;
    for e in &graph.edges {
        if e.source == start {
            if (!visited.contains(&e.dest)) {
                let next_layer = explore(graph, e.dest, visited, clock, Some(start));
                for ni in next_layer {
                    result.insert(ni);
                }
            }
        }        
    }
    result.insert(NodeInfo {node: start, postorder: clock.0, parent: parent});
    clock.0 += 1;
    result
}

fn dfs_traverse<T: Hash+Eq+Copy>(graph: &Graph<T>) -> HashSet<NodeInfo<T>> {
    let mut visited = HashSet::new();
    let mut clock = Clock(1);
    let mut result = HashSet::new();
    for node in graph.nodes() {
        if !visited.contains(&node) {
            let dfs_res = explore(&graph, node, &mut visited, &mut clock, None);
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

impl<T: Hash+Eq+Copy> Graph<T> {
    
    fn nodes(&self) -> HashSet<Node<T>> {
        let mut all_nodes = HashSet::new();
        for e in &self.edges {
            all_nodes.insert(e.source);
            all_nodes.insert(e.dest);
        }
        all_nodes
    }
    
    fn has_cycles(&self) -> bool {
        let dfs = dfs_traverse(self);
        
        back_edges(self, dfs).len() > 0
    }

    fn is_dag(&self) -> bool {
        !self.has_cycles()
    }

    fn connected(&self) -> bool {
        let mut clock = Clock(1);
        let num_nodes = self.nodes().len();
        for node in self.nodes() {
            let mut visited = HashSet::new();

            let res = explore(self, node, &mut visited, &mut clock, None);

            if visited.len() < num_nodes {
                return false;
            }
        }
        true
    }
}

fn top_sort<T: Hash+Eq+Copy>(graph: &Graph<T>) ->  Vec<NodeInfo<T>> {
    let dfs = dfs_traverse(graph);
    dfs
        .into_iter()
        .sorted_by(|ref a,ref b| b.postorder.cmp(&a.postorder))
}

fn mst_kruskall<T: Hash+Eq+Copy>(graph: &Graph<T>) -> HashSet<Edge<T>> {
    let mut mst = HashSet::new();

    let sorted_edges = graph
        .edges
        .clone()
        .into_iter()
        .sorted_by(|a, b| a.weight.cmp(&b.weight));

    for edge in sorted_edges {
        let mut scratch_work = mst.clone();
        scratch_work
            .insert(edge.clone());
        let gr = Graph {edges: scratch_work};
        if !gr.has_cycles() {
            mst.insert(edge);
        }
    }
    mst
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

    edges.insert(Edge {source: D, dest: B, weight: 5});
    
    let gr = Graph {edges: edges};
    let nodeset = top_sort(&gr);
    println!("{:?}", nodeset);
    println!("{:?}", gr.is_dag());
    println!("{:?}", mst_kruskall(&gr));
    println!("{:?}", gr.connected());
}
