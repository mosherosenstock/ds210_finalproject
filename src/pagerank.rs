// PREVIOUS FUNCTIONS USED IN MAIN THAT I AM NOT CURRENTLY USING


// BFS LECTURE BASED CODE

// Discovering vertices of a connected component via BFS
// fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
//     component[vertex] = Some(component_no);
    
//     let mut queue = std::collections::VecDeque::new();
//     queue.push_back(vertex);
    
//     while let Some(v) = queue.pop_front() {
//         for w in graph.outedges[v].iter() {
//             if let None = component[*w] {
//                 component[*w] = Some(component_no);
//                 queue.push_back(*w);
//             }
//         }
//     }
// }



// --------------------------------------------------------------------------------
// --------------------------------------------------------------------------------
// ALL THIS BELOW GOES IN MAIN
// --------------------------------------------------------------------------------
    // // IMPLEMENTING BFS ALGORITHM LECTURE BASED CODE
    // // -------------------------------------------------------------------------
    // // Marking all connected components (BFS)
    // let mut component: Vec<Option<Component>> = vec![None;n];

    // let mut component_count = 0;

    // for v in 0..n {
    //     if let None = component[v] {
    //         component_count += 1;
    //         mark_component_bfs(v, &graph, &mut component, component_count);
    //     }
    // };

    // // Let's verify the assignment!
    // print!("{} components:\n[  ",component_count);
    // for v in 0..n {
    //     print!("{}:{}  ",v,component[v].unwrap());
    // }
    // println!("]\n");
    // // -------------------------------------------------------------------------
// --------------------------------------------------------------------------------
// --------------------------------------------------------------------------------













