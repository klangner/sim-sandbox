use hecs::{World, Entity};


pub struct Node {
}

pub struct Edge {

}

pub struct Grower {
    pub scfm: f32,
}

pub struct SourceNode {
    pub scfm: f32,
}

pub struct Sink {
}

pub struct Pipe {
    pub src_node: Entity,
    pub dest_node: Entity,
    pub is_open: bool,
    pub scfm: f32,
}

pub struct FlowMeter {
    pub pipe_id: usize,
}

impl Edge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Node {
    pub fn new() -> Self {
        Self {}
    }
}

impl Grower {
    pub fn new() -> Self {
        Self { scfm: 0. }
    }
}

impl SourceNode {
    pub fn new(scfm: f32) -> Self {
        Self {scfm}
    }
}

impl Sink {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pipe {
    pub fn new(from: Entity, to: Entity) -> Self {
        Self { src_node: from, dest_node: to, is_open: true, scfm: 0.0 }
    }
}

trait Model {
    fn update(&mut self, ts: i32);
}

impl Model for World {
    fn update(&mut self, _ts: i32) {
        let pipes = vec![];
        for (_id, pipe) in self.query_mut::<&mut Pipe>() {
            if let Ok(source) = self.get::<&SourceNode>(pipe.src_node) {
                pipe.scfm = source.scfm;
                pipes.push(&pipe);
            }
        }
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
    use hecs::World;

    use crate::flow::model::*;

    #[test]
    fn test_model() {
        let mut world = World::new();
        let a = world.spawn((Node::new(), SourceNode::new(100.)));
        let b = world.spawn((Node::new(), Grower::new()));
        let c = world.spawn((Node::new(), Sink::new()));
        
        let p1 = world.spawn((Edge::new(), Pipe::new(a, b)));
        let p2 = world.spawn((Edge::new(), Pipe::new(b, c)));

        world.update(0);

        // Random access is simple and safe
        assert_eq!(world.get::<&SourceNode>(a).unwrap().scfm, 100.);
        assert_eq!(world.get::<&Pipe>(p1).unwrap().src_node, a);
    }
}
