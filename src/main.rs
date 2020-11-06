use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

/// Contains the heuristic cost to the goal for the given node
/// Used for the binary heap in the frontier vector to hold
/// all possible nodes which might be visited next.
///
/// Selection is based on the cost field (with a min heap)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    /// Heuristic cost from the node to the goal
    cost: usize,
    /// The node associated with the cost above
    node: usize,
}

// Got this idea from the example in std::collections::binary_heap
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for NodeCost {
    fn cmp(&self, other: &NodeCost) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare the node (index) - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &NodeCost) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // let path: &str = "input.cav";
    let path: &str = "generated5000-1.cav";

    // This extra binding to non mutable content is needed as expect() consumes the value and split()
    // has a lifetime specifier which would point to a temporary object (to
    // the x value of Some(x)) by expect().
    // We can use the object in that statement, but after the
    // statement is finished, it will be discarded. Hence, we need to bind
    // the result of expect to a variable which then can be used further down
    // the road.
    //
    // So this would not work
    // let mut content = fs::read_to_string(&path)
    //     .expect("Could not read file")
    //     .split(',');
    //
    // Instead we use the intermediate let binding to hold the Some() value
    // which is returned by expect().
    let content = fs::read_to_string(&path).expect("Could not read file");
    let mut content = content.split(',');

    // The number of nodes
    let amount: usize = content
        .next()
        .unwrap_or_else(|| panic!("File is not properly formatted. Expecting something here!"))
        .parse::<usize>()
        .unwrap_or_else(|count| panic!("Could not parse {} to usize", count));

    // The goal node is the last one (goal referes to the index)
    let goal: usize = amount - 1;

    // Parse the coordinates
    // With using unwrap() here, we are assuming that the input file is
    // correctly structered.
    let mut nodes: Vec<(i32, i32)> = Vec::with_capacity((amount / 2) as usize);
    for _ in 0..amount {
        nodes.push((
            content.next().unwrap().parse().unwrap(),
            content.next().unwrap().parse().unwrap(),
        ));
    }

    // Read [amount X amount] matrix
    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(amount);
    for _ in 0..amount {
        matrix.push(
            content
                .by_ref()
                .take(amount)
                .map(|arg| arg.parse::<u8>().unwrap())
                .collect(),
        );
    }

    // If we found a path to goal node
    let mut found = false;

    // This is a binary heap of many NodeCost structs ordered by the
    // cost field.
    // The containing nodes are considered to be explored next.
    let mut frontier = BinaryHeap::new();
    // Add the startig node with cost 0 and index 0
    frontier.push(NodeCost { cost: 0, node: 0 });

    // Contains the actual (travelled) cost (value) to a node (key)
    let mut travelled_cost: HashMap<usize, f64> = HashMap::new();
    // Insert the start node with node 0 as key and cost 0 as value
    travelled_cost.insert(0, 0_f64);

    // prev_node contains the previous node of a node.
    // So the value of a key is the previous node of the node
    // used for the key.
    // Used to reconstruct the path through the graph in the end.
    let mut prev_node: HashMap<usize, usize> = HashMap::new();

    while let Some(current) = frontier.pop() {
        // goal test
        if current.node == goal {
            found = true;
            break;
        }

        let neighbours = adjacent_nodes(&matrix, current.node);

        for neighbour in neighbours {
            // calculate cost from current node to selected neighbour node
            let temp_cost: f64 = travelled_cost.get(&current.node).unwrap()
                + distance(&nodes, current.node, neighbour);

            // check if neighbour was already visited.
            // If yes, there is a travelled cost in the hashmap travelled_cost
            match travelled_cost.get(&neighbour) {
                Some(cost) => {
                    // If true, we found a cheaper path to neighbour
                    if temp_cost < *cost {
                        // Set the current node as previous node for the neighbour
                        prev_node.insert(neighbour, current.node);

                        // Set the travelled cost to neighbour over current
                        travelled_cost.insert(neighbour, temp_cost);

                        // Update the cost of neighbour with heuristic to goal
                        frontier.push(NodeCost {
                            // TODO: Needs further examination. Casting f64 to usize to not be
                            // bothered with PartialEq and PartialOrd for the type f64.
                            // Though, might be loosing precision now.
                            cost: (temp_cost + distance(&nodes, neighbour, goal)) as usize,
                            node: neighbour,
                        });
                    }
                }
                None => {
                    // Set the current node as previous node for the neighbour
                    prev_node.insert(neighbour, current.node);
                    // Set the travelled cost to neighbour over current
                    travelled_cost.insert(neighbour, temp_cost);

                    // Add travelled distance and heuristic
                    frontier.push(NodeCost {
                        // TODO: Needs further examination. Casting f64 to usize to not be
                        // bothered with PartialEq and PartialOrd for the type f64.
                        // Though, might be loosing precision now.
                        cost: (temp_cost + distance(&nodes, neighbour, goal)) as usize,
                        node: neighbour,
                    });
                }
            }
        }
    } // end while

    if found {
        println!("Found the following path:");
        // println!("{:?}", reconstruct_path(&prev_node, goal));
        // Have it printed the way my python script does
        println!(
            "{}",
            reconstruct_path(&prev_node, goal)
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!("Length: {}", travelled_cost.get(&goal).unwrap());
    } else {
        println!("Did not find a path");
    }

    // println!("Goal node: {}", goal);
    // println!("Number of nodes: {}", amount);
    // println!("Nodes: {:?}", nodes);
    // println!("Matrix: {:?}", matrix);

    // for i in 0..amount {
    //     println!(
    //         "distance between 0 and {} = {}",
    //         i,
    //         distance(&nodes, 0, i as usize)
    //     );
    // }

    // println!("Node 0 has connection to {:?}", adjacent_nodes(&matrix, 0));
}

/// Returns the straight line distance between two nodes
///
/// # Arguments
///
/// * `nodes` - A reference to the vector which contains the
/// coordinates of the nodes as tuple
/// * `node1` - The index of the first node
/// * `node2` - The index of the second node
fn distance(nodes: &Vec<(i32, i32)>, node1: usize, node2: usize) -> f64 {
    // ((x1 - x2)^2 + (y1 - y2)^2).sqrt()
    (((nodes[node1].0 - nodes[node2].0) as f64).powi(2)
        + ((nodes[node1].1 - nodes[node2].1) as f64).powi(2))
    .sqrt()
}

/// Returns a vector of indeces which represent the list of
/// adjacent nodes to the given node
///
/// # Arguments
///
/// * `matrix` - A reference to the connection matrix
/// * `node` - The node for which the adjacent nodes are returned
fn adjacent_nodes(matrix: &Vec<Vec<u8>>, node: usize) -> Vec<usize> {
    let mut adj_nodes: Vec<usize> = Vec::new();
    for index in 0..matrix.len() {
        if matrix[index][node] == 1 {
            adj_nodes.push(index);
        }
    }
    adj_nodes
}

/// Returns a vector with the path from the first node to the goal node.
///
/// # Arguments
///
/// * `prev_node` - A reference to the prev_node hashmap to find the
/// previous node of a node
/// * `current` - The index of the goal node
fn reconstruct_path(prev_node: &HashMap<usize, usize>, mut current: usize) -> Vec<usize> {
    // While pushing into `path` we need to add 1 because we want to
    // see the nodes numbers and not the indeces.
    let mut path = vec![current + 1];

    while let Some(&node) = prev_node.get(&current) {
        // println!("Adding {}", node);
        path.insert(0, node + 1);
        current = node;
    }
    path
}
