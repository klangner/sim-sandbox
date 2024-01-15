
pub trait ModelBuilder {
    fn new() -> Self;
    fn add_node(&mut self, node: Node) -> usize;
    fn add_pipe(&mut self, from_node: usize, to_node: usize) -> Option<usize>;
    fn build(&self) -> &Model;
}

pub struct Node {
    pub max_scfm: f32,
}

pub struct Pipe {
    pub src_node: usize,
    pub dest_node: usize,
    pub is_open: bool,
    pub scfm: f32,
}

pub struct FlowMeter {
    pub pipe_id: usize,
}

pub struct Model {
    pub nodes: Vec<Node>,
    pub pipes: Vec<Pipe>,
    pub flow_meters: Vec<FlowMeter>,
}

impl ModelBuilder for Model {
    fn new() -> Self {
        Self {
            nodes: vec![],
            pipes: vec![],
            flow_meters: vec![],
        }
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len()
    }

    fn add_pipe(&mut self, from_node: usize, to_node: usize) -> Option<usize> {
        self.pipes.push(Pipe::new(from_node, to_node));
        Some(self.pipes.len())
    }

    fn build(&self) -> &Model {
        self
    }
}

impl Pipe {
    pub fn new(from: usize, to: usize) -> Self {
        Self { src_node: from, dest_node: to, is_open: true, scfm: 0.0 }
    }
}

impl Model {
    // pub fn update(&mut self, ts: i32) {
    //     input_pipes = []
    //     for source in sources:
    //         pipe = find_pipe(source)
    //         pipe.flow = source.max_flow
    //         input_pipes.append(pipe)

    //     while len(input_pipes) > 0:
    //         in = input_pipes.pop() // get and remove
    //         node = find_node(in.dest)
    //         flow = min(in.flow, node.max_flow)
    //         in.flow = flow
    //         node.flow = flow
    //         input_pipes.append(find_pipe(node))


    //     update_flow_meters()
    //     update_analyzers()
    // }
}