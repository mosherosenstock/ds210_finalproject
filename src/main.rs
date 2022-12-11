// import crates
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::collections::VecDeque;
use std::sync::Arc;
use std::cmp::Reverse;
use std::collections::BinaryHeap;


fn main() {
    let mut n=1000;
    let mut list_edges = read_file("edges_huawei.txt");
    list_edges.sort();

    let page_rank = page_rank(&mut list_edges, n);
    // println!("{:?}",page_rank);
    // page_rank.sort();
    // println!("{:?}",list_edges.len());

    // With this code we are filtering our list_edges to only contain the connections between the top 50 edges
    list_edges.retain(|&(a, _)| page_rank.iter().any(|&(b, _)| a == b));
    list_edges.retain(|&(_, a)| page_rank.iter().any(|&(b, _)| a == b));
    list_edges.sort();

    // println!("{:?}",list_edges);
    // println!("{:?}",list_edges.len());
    
    //  HERE WE CREATE OUR GRAPH FOR THE CONNECTIONS BETWEEN THE TOP 50 NODES
    let mut adj_list = grouped_vertex_tuples(&list_edges);
    n = adj_list.len();
    let mut graph = Graph::create_directed(n,&list_edges);
    graph.outedges.retain(|edges| !edges.is_empty());

    graph = graph.sort_ascending_order();
    // println!("{:?}",graph);

    println!("These are the top 50 nodes and their connections between them: ");
    for (i, edges) in graph.outedges.iter().enumerate() {
        // Skip vertices that don't have any outgoing edges
        if edges.is_empty() {
            continue;
        }
        // Print the vertex number and its outgoing edges
        println!("Node: {} - Edges: {:?}", page_rank[i].0, edges);
    }

    // IMPLEMENTING BFS ALGORITHM - AI BASED CODE
    // -------------------------------------------------------------------------
    // I AM HAVING THE PROBLEM THAT THE RETURN STATEMENT 
    // IS NOT RETURNING A 50 LEN VECTOR AS EXPECTED
    // -------------------------------------------------------------------------  
    let visited = bfs(&graph, list_edges[0].0);
    println!("Visited vertices BFS: {:?}", visited);  
    println!("");

    // IMPLEMENTING DIJKSTRA ALGORITHM - AI BASED CODE
    // -------------------------------------------------------------------------
    let distances = dijkstra(&graph, 0);
    println!("Distances from initial vertex: {:?}", distances);



    // //  THIS IS DFS , WE NEED BFS 
    // // -------------------------------------------------------------------------
    // let mut visited = vec![false;graph.n];
    // let mut distance = vec![0;graph.n];

    // // Calculate DFS for top 50 nodes (rank based on page rank)
    // for i in 0..50 {
    //     println!{"For vertex {}", page_rank[i].0};
    //     visited[page_rank[i].0] = true;
    //     distance[page_rank[i].0] = 0;
    
    //     dfs(page_rank[i].0, &graph, 1, &mut visited, &mut distance);
    //     println!("vertex:distance");
    //     for v in 0..graph.n {
    //         print!("   {}:{}",v,distance[v]);
    //     }
    //     println!();
    // }
    // // -------------------------------------------------------------------------





}

// IMPLEMENTING DIJKSTRA ALGORITHM 
// -------------------------------------------------------------------------
// Define a function that performs the Dijkstra algorithm on a graph
fn dijkstra(g: &Graph, source: Vertex) -> Vec<usize> {
    // Create a priority queue of vertices, where the priority is determined by the distance from the source vertex
    let mut queue = BinaryHeap::new();

    // Create a vector to hold the distances from the source vertex to all other vertices in the graph
    let mut distances = vec![std::usize::MAX; g.n];

    // Set the distance from the source vertex to itself to 0
    distances[source] = 0;

    // Add the source vertex to the priority queue
    queue.push(Reverse((0, source)));

    // Continue processing vertices until the priority queue is empty
    while let Some(Reverse((d, v))) = queue.pop() {
        // Skip this vertex if it has already been processed
        if d != distances[v] {
            continue;
        }
        // Iterate over the neighbors of the current vertex
        for &w in &g.outedges[v] {
            // Calculate the distance from the source vertex to this neighbor
            let distance = d + 1;

            // Update the distance of this neighbor if it is smaller than the current distance
            if distance < distances[w] {
                distances[w] = distance;
                queue.push(Reverse((distance, w)));
            }
        }
    }
    // Return the vector of distances from the source vertex to all other vertices
    distances
}
// -------------------------------------------------------------------------


// // IMPLEMENTING BFS ALGORITHM 
// // -------------------------------------------------------------------------
fn bfs(g: &Graph,source_node:usize) -> Vec<Vertex> {
    // Create a queue to hold the vertices that are waiting to be processed
    let mut queue = VecDeque::new();

    // Create a vector to hold the vertices in the order they were visited
    let mut visited = Vec::new();

    // Start the BFS algorithm at the specified source node
    queue.push_back(source_node);

    // Continue processing vertices until the queue is empty
    while let Some(v) = queue.pop_front() {
        // Skip this vertex if it has already been visited
        if visited.contains(&v) {
            continue;
        }
        // Add this vertex to the visited list
        visited.push(v);

        // Add all the unvisited neighbors of this vertex to the queue
        if v < g.outedges.len() {
            for &w in &g.outedges[v] {
                if !visited.contains(&w) {
                    queue.push_back(w);
                }
            }
        }
    }
    println!("{:?}",visited.len());
    // Return the list of vertices in the order they were visited
    visited
}


// // -------------------------------------------------------------------------

// MARK COMPONENT BFS
fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    // Check that the index of the vertex is within the bounds of the component vector
    if vertex < component.len() {
        component[vertex] = Some(component_no);
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);

    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                // Check that the index of the vertex is within the bounds of the component vector
                if *w < component.len() {
                    component[*w] = Some(component_no);
                }
                queue.push_back(*w);
            }
        }
    }
}


// FUNCTION DFS 
// TO COMPUTE DISTANCE FROM A GIVEN VERTEX
fn dfs(vertex:Vertex, graph: &Graph, d: usize, visited: &mut Vec<bool>, distance: &mut Vec<usize> ){
    for w in graph.outedges[vertex].iter() {
      if visited[*w] == false {
        distance[*w] = d;
        visited[*w] = true;
        dfs(*w, graph, d+1, visited, distance);
      }
    }
}

// FUNCTION TO CREATE AN ADJACENCY LIST GIVEN A LIST OF EDGES
fn grouped_vertex_tuples(vertex_tuples: &[(Vertex, Vertex)]) -> AdjacencyLists {
    let mut adjacency_lists: AdjacencyLists = Vec::new();

    for &(u, v) in vertex_tuples {
        if adjacency_lists.len() <= u {
            adjacency_lists.resize(u + 1, Vec::new());
        }
        adjacency_lists[u].push(v);
    }
    adjacency_lists
}


// FUNCTION TO READ FILE AND RETURN A LIST OF EDGES
fn read_file(_path: &str) -> Vec<(Vertex, Vertex)>{
    let mut list_edges: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open("edges_huawei.txt").expect("Could not open file");

    let mut buf_reader = std::io::BufReader::new(file).lines();
    buf_reader.next();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let line_str = line_str.replace("[","");
        let line_str = line_str.replace("]","");
        let line_str = line_str.replace(" ","");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<i128>().unwrap();
        let y = v[1].parse::<i128>().unwrap();
        list_edges.push((x as Vertex, y as Vertex));
    }
    return list_edges;
}


// CREATE A GRAPH TYPE
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
type Component = usize;

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

// IMPLEMENTATION OF GRAPH
impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
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
    
    fn create_undirected(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }

    fn sort_ascending_order(&self) -> Graph {
        // Create a vector of tuples representing the edges in the graph
        let mut edges: ListOfEdges = Vec::new();

        // Iterate over the vertices in the graph
        for (u, neighbors) in self.outedges.iter().enumerate() {
            // Iterate over the neighbors of the current vertex
            for &v in neighbors {
                // Add the edge (u, v) to the list of edges
                edges.push((u, v));
            }
        }

        // Sort the edges in ascending order
        edges.sort();

        // Create a vector of vectors of vertices to hold the sorted graph
        let mut sorted_outedges = Vec::new();

        // Iterate over the sorted edges
        for (u, v) in edges {
            // If the current vertex is not already in the sorted graph, add it
            if sorted_outedges.len() <= u {
                sorted_outedges.push(Vec::new());
            }

            // Add the neighbor of the current vertex to the sorted graph
            sorted_outedges[u].push(v);
        }

        // Create and return a new Graph struct using the sorted graph
        Graph {
            n: self.n,
            outedges: sorted_outedges,
        }
    }
}


// COMPUTE THE PAGE RANK FOR TOP 50 VERTEXES
fn page_rank(data: &mut Vec<(Vertex, Vertex)>, n_vertices:usize) -> Vec<(Vertex, f64)> { 
    // This function creates a page rank calculator for each node
    // select a random node 
    let mut rng = rand::thread_rng();
    let r_node = rng.gen_range(0..=(data.len()));
    // convert r_node to usize
    let r_node = r_node as usize;
    // make a variable node that contains the initial node that is being calculated
    let mut node = data[r_node].0;
    // create a variable page_rank to store the PageRank value for each node
    // the first value should be a random iteration from 0 to 1000, 
    // the second value should be the PageRank value (starting at 0.0)
    let mut page_rank: Vec<(Vertex, Vertex)> = Vec::new();

    for i in 0..n_vertices {
        page_rank.push((i, 0));
    }
    // create a empty vector "out" to store the outgoing edges; and variable "out_edges" to count how many of them
    let mut out = Vec::new();
    let mut out_edges = 0;
    // starting from that random node make 100 random steps
    for _i in 0..100 {
        // check if the node has any outgoing edges that are not himself
        for j in 0..data.len() {
            if data[j].0 == node {
                out_edges += 1;
                out.push(data[j].1);
            }
        }
        // if the node has no outgoing edges, select a new randon node
        while out_edges == 0 {
            let mut rng = rand::thread_rng();
            node = rng.gen_range(0..=(data.len()));
            node = data[node as usize].0;
        }
        // check if the node has any outgoing edges
        for j in 0..data.len() {
            if data[j].0 == node {
                out_edges += 1;
                // add the outgoing edges to the vector "out"
                out.push(data[j].1);
            }
        }
        // if the node has outgoing edges, select a random outgoing edge with 9/10 probability and a random node with 1/10 probability
        let random = rng.gen_range(0..10);
        // probability of 1/10 to select a random node (from entire graph)
        if random == 0 {
            node = rng.gen_range(0..=(data.len()));
            node = data[node as usize].0;
        }
        // probability of 9/10 to select a random node (from conecting edges)
        else {
            // select a random value from vector out
            // select a random node from the conecting edges
            let mut rng = rand::thread_rng();
            node = rng.gen_range(0..out_edges);
            node = out[node as usize];
        }
        // now we should add one point to the PageRank value of the node that we are on
        for i in 0..page_rank.len() {
            if page_rank[i].0 == node {
                page_rank[i].1 += 1;
            }
        }
        // reset the vector "out" and the variable "out_edges"
        out = Vec::new();
        out_edges = 0;
    }
    // END OF THE 100 ITERATIONS
    // sort the page_rank vector by the second element of the tuple
    page_rank.sort_by(|a, b| b.1.cmp(&a.1));
    // create a new vector that contains the page rank of the top 50 nodes
    let mut top_50: Vec<(Vertex, f64)> = Vec::new();
    for i in 0..50 {
        // divide the second value of each tuple by 100 to get the page rank
        top_50.push((page_rank[i].0 as Vertex, page_rank[i].1 as f64 / 100.0));
    }
    return top_50;
}