# DS 210 Final Project
 ds210_finalproject_moshe_rosenstock_
 
In this project, we analyze a Twitter communication network dataset provided by Huawei. The dataset contains information on 1000 nodes (i.e., individuals) and 50153 edges (i.e., connections) between them. The graph is directed and labeled, with each node representing a unique user.
To begin, we imported and cleaned the data by converting the original Excel file into a text file and replacing each user's name with a unique node label. We then used this text file to create a vector of tuples in Rust, which we used to create a graph using a custom Graph type and its associated methods.
Next, we applied the PageRank algorithm to the graph in order to identify the top 50 nodes based on their PageRank scores. We then created a new graph containing only the connections between these top 50 nodes. This new graph allowed us to perform a breadth-first search (BFS) in order to find the shortest path between any two nodes. Finally, we calculated the distance between each pair of top 50 nodes, providing insight into the structure of the graph.
Overall, this analysis allowed us to effectively reduce the complexity of the original dataset and gain a better understanding of the connections within the Twitter network. This information can be useful for customer analysis in Huawei's marketing strategy.

