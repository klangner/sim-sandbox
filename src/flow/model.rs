
pub trait Node {
    fn max_scfm(&self) -> f32;
    fn scfm(&self) -> f32;
    fn set_scfm(&self, scfm: f32);
}

pub struct Model {
    pub nodes: Vec<Box<dyn Node>>,
    pub pipes: Vec<Pipe>,
}

pub struct Grower {
    pub scfm: f32,
}

pub struct SourceNode {
    pub max_scfm: f32,
}

pub struct Sink {
}

pub struct Pipe {
    pub src_node: u32,
    pub dest_node: u32,
    pub is_open: bool,
    pub scfm: f32,
}

impl Grower {
    pub fn new() -> Self {
        Self { scfm: 0. }
    }
}

impl SourceNode {
    pub fn new() -> Self {
        Self {max_scfm: f32::MAX}
    }
}

impl Sink {
    pub fn new() -> Self {
        Self {}
    }
}

impl Node for Grower {
    fn max_scfm(&self) -> f32 {
        todo!()
    }

    fn scfm(&self) -> f32 {
        todo!()
    }

    fn set_scfm(&self, scfm: f32) {
        todo!()
    }
}
impl Node for SourceNode {
    fn max_scfm(&self) -> f32 {
        todo!()
    }

    fn scfm(&self) -> f32 {
        todo!()
    }

    fn set_scfm(&self, scfm: f32) {
        todo!()
    }
}

impl Node for Sink {
    fn max_scfm(&self) -> f32 {
        todo!()
    }

    fn scfm(&self) -> f32 {
        todo!()
    }

    fn set_scfm(&self, scfm: f32) {
        todo!()
    }
}



impl Pipe {
    pub fn new(from: u32, to: u32) -> Self {
        Self { src_node: from, dest_node: to, is_open: true, scfm: 0.0 }
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            pipes: vec![],
        }
    }

    pub fn update(&mut self, _ts: i32) {
        // let pipes = vec![];
        // for (_id, pipe) in self.query_mut::<&mut Pipe>() {
        //     if let Ok(source) = self.get::<&SourceNode>(pipe.src_node) {
        //         pipe.scfm = source.scfm;
        //         pipes.push(&pipe);
        //     }
        // }
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
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use crate::flow::{model::*, model_ecs::{Edge, World}};

    #[test]
    fn test_model() {
        let mut world = World::new();
        // let a = world.spawn((Node::new(), SourceNode::new(100.)));
        // let b = world.spawn((Node::new(), Grower::new()));
        // let c = world.spawn((Node::new(), Sink::new()));
        
        // let p1 = world.spawn((Edge::new(), Pipe::new(a, b)));
        // let p2 = world.spawn((Edge::new(), Pipe::new(b, c)));

        // world.update(0);

        // // Random access is simple and safe
        // assert_eq!(world.get::<&SourceNode>(a).unwrap().scfm, 100.);
        // assert_eq!(world.get::<&Pipe>(p1).unwrap().src_node, a);
        assert!(true);
    }
}
