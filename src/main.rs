enum Kind {
    Kind1,
    Kind2,
}

trait CostEstimator {
    fn estimate(&self, inputs: Vec<usize>) -> f32;
}

struct LinearCost {
    weight_by_input_size: Vec<f32>,
}

impl CostEstimator for LinearCost {
    fn estimate(&self, inputs: Vec<usize>) -> f32 {
        inputs
            .iter()
            .zip(self.weight_by_input_size.iter())
            .map(|(i, w)| *i as f32 * w)
            .sum()
    }
}

type NodeId = i32;
type ClusteredNodeId = i32;

struct Node {
    id: NodeId,
    kind: Kind,
    size: usize,
    weight: Box<dyn CostEstimator>,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

struct Graph {
    nodes: Vec<Node>,
    switch_cost_0_to_1: f32,
    switch_cost_1_to_0: f32,
}

struct ClusteredNode {
    id: ClusteredNodeId,
    nodes: Vec<NodeId>,
    input_nodes: Vec<NodeId>,
    output_nodes: Vec<NodeId>,
    inputs: Vec<ClusteredNodeId>,
    outputs: Vec<ClusteredNodeId>,
}

struct DefaultSwitchCostEstimator {}

impl DefaultSwitchCostEstimator {}

struct ClusteredGraph {
    nodes: Vec<ClusteredNode>,
    switch_cost_0_to_1: f32,
    switch_cost_1_to_0: f32,
}

impl Graph {
    fn has_cycle(&self) -> bool {
        false
    }
}

impl ClusteredGraph {
    fn has_cycle(&self) -> bool {
        false
    }
}

struct Cluster {}

impl Cluster {
    fn do_cluster(dag: &Graph) -> ClusteredGraph {
        todo!()
    }
}

struct Scheduler {}

impl Scheduler {
    fn do_schedule(cluster: ClusteredGraph) -> Vec<(Kind, Vec<Node>)> {
        todo!()
    }
}

// 1 -> 2
// 1 -> 3
// 2 -> 3
// 3 -> 4

fn main() {
    println!("Hello, world!");
}
