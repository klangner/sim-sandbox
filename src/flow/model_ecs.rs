
pub struct Node {
}

// pub struct Edge {

// }

// pub struct Pipe {
//     pub src_node: usize,
//     pub dest_node: usize,
//     pub is_open: bool,
//     pub scfm: f32,
// }

pub struct Grower {
    pub scfm: f32,
}

pub struct Source {
    pub max_scfm: f32,
}

// pub struct Sink {
//     pub max_scfm: f32,
// }

pub struct Model {
    pub nodes: Vec<Option<Node>>,
    pub growers: Vec<Option<Grower>>,
    pub sources: Vec<Option<Source>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            growers: vec![],
            sources: vec![],
        }
    }

    pub fn new_node(&mut self, node: Option<Node>, grower: Option<Grower>, source: Option<Source>) -> usize {
        self.nodes.push(node);
        self.growers.push(grower);
        self.sources.push(source);
        self.nodes.len() -1
    }

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

// impl Pipe {
//     pub fn new(from: usize, to: usize) -> Self {
//         Self { src_node: from, dest_node: to, is_open: true, scfm: 0.0 }
//     }
// }

impl Node {
    pub fn new() -> Self {
        Self {}
    }
}

impl Grower {
    pub fn new() -> Self {
        Self {scfm: 0.0 }
    }
}

impl Source {
    pub fn new(max_scfm: f32) -> Self {
        Self {max_scfm}
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::flow::model_ecs::Source;

    use super::{Model, Node, Grower};


    #[test]
    fn test_new_model() {
        let mut model = Model::new();
        let g1 = model.new_node(Some(Node::new()), Some(Grower::new()), None);

        assert_eq!(g1, 0);
    }

    #[test]
    fn test_select() {
        let mut model = Model::new();
        let s1 = model.new_node(Some(Node::new()), None, Some(Source::new(100.0)));
        let g1 = model.new_node(Some(Node::new()), Some(Grower::new()), None);

        let growers: Vec<(&Node, &Grower)> = model
            .nodes
            .iter()
            .zip(model.growers.iter())
            .filter_map(|(node, grower)| {
                Some((node.as_ref()?, grower.as_ref()?))
            })
            .collect();

        assert_eq!(s1, 0);
        assert_eq!(g1, 1);
        assert_eq!(growers.len(), 1);
    }
}