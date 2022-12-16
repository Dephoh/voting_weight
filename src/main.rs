use std::fs::File;
use std::io::prelude::*;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in &edges[1..] {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
}

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize,usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("error reading line");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        if !line_str.contains(" ") {
            result.push((v[0].parse::<usize>().unwrap(),0));
        } else {
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
    }
    return result;
}

fn condense(verts:&mut Vec<(usize,usize)>) -> Vec<Vec<usize>> {
    let length = verts[0].0;
    verts.remove(0);
    verts.sort();
    let mut condensed: Vec<Vec<usize>> = vec![vec![];length];
    for pair in verts {
        if condensed[pair.0].contains(&pair.1) {
            continue;
        } else {
            condensed[pair.0].push(pair.1);
        }
    }
    condensed
}

fn dfs_collect_stack(v:Vertex, graph:&Graph, stack:&mut Vec<Vertex>, visited:&mut Vec<bool>) {
    if !visited[v] {
        visited[v] = true;
        for w in graph.outedges[v].iter() {
            dfs_collect_stack(*w, graph, stack, visited);
        }
        stack.push(v);
    }
}


fn connected_bottomup(vertex: usize, data: &mut Vec<(usize,usize)>, target: & Vec<usize>)->Vec<usize>{
    let n = data[0].0;

    let graph = Graph::create_directed(n, &reverse_edges(&data));

    let mut stack: Vec<Vertex> = Vec::new();
    let mut iscon: Vec<Vertex> = Vec::new();
    let mut visited = vec![false;graph.n];

    dfs_collect_stack(25,&graph, &mut stack, &mut visited);
    
    for n in target.iter() {
        if stack.contains(n) {
            iscon.push(*n);
        }
    }
    iscon
}

fn compute_influence(starts: Vec<usize>,ends: Vec<usize>, voters: Vec<&Vec<usize>>, mut data:  Vec<(usize,usize)>) -> Vec<Vec<f32>>{
    let mut tally:Vec<Vec<usize>> = vec![vec![0;starts.len()];ends.len()];
    let mut percentages:Vec<Vec<f32>> = vec![vec![0.;starts.len()];ends.len()];

    for i in 0..ends.len() {
        for j in voters[i] {
            let con = connected_bottomup(*j, &mut data, &starts);
            for n in con.iter() {
                tally[i][*n] += 1;
            }
        }
    }

    for i in 0..tally.len() {
        for j in 0..tally[0].len() {
            percentages[i][j] = tally[i][j] as f32 / voters.len() as f32;
        }
    }

    return percentages;
}

fn reverse_data(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![list[0]];
    for (u,v) in &list[1..] {
        new_list.push((*v,*u));
    }
    new_list
}

#[test]
fn test_data() {
    let raw_data = read_file("test_data.txt");


    let starts:Vec<usize> = (0..=4).collect();
    // Calculate influence for these nodes
    let ends: Vec<usize> = (25..=29).collect();
    // based on these nodes


    let mut voters: Vec<&Vec<usize>> = Vec::new();

    //fills voters
    let mut raw_for_con = read_file("test_data.txt");
    raw_for_con = reverse_data(&raw_for_con);
    let condensed = condense(&mut raw_for_con);
    for n in ends.iter() {
        voters.push(&condensed[*n]);
    }

    let percentages = compute_influence(starts,ends,voters,raw_data);

    assert_eq!(percentages.len(),5);
    assert_eq!(percentages[0].len(),5);
}

fn main() {

    let raw_data = read_file("data.txt");

    let starts:Vec<usize> = (0..=20).collect();
    // Calculate influence for these nodes
    let ends: Vec<usize> = (8279..=8298).collect();
    // based on these nodes


    let mut voters: Vec<&Vec<usize>> = Vec::new();

    //fills voters
    let mut raw_for_con = read_file("data.txt");
    raw_for_con = reverse_data(&raw_for_con);
    let condensed = condense(&mut raw_for_con);
    for n in ends.iter() {
        voters.push(&condensed[*n]);
    }

    let percentages = compute_influence(starts,ends,voters,raw_data);

    for i in percentages.iter() {
        println!("{:?}",i);
    }

    //interestingly, there are no connections. This means all the voters were voted in by someone other than the original 20 nodes.

    // let start = 1
    // let mut one_cnx = bfs_connected(start,&c_data);
    // one_cnx.sort();
    // // one_cnx.dedup();
    // println!("{:?}", one_cnx);
    // println!("{:?}", one_cnx.len());


}
