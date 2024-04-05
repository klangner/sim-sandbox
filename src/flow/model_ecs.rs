
pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

pub struct World {
    // We'll use `entities_count` to assign each Entity a unique ID.
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(&mut self, entity: usize, component: ComponentType) {

        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(new_component_vec));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub struct Node {
}

pub struct Edge {

}

pub struct Pipe {
    pub src_node: usize,
    pub dest_node: usize,
    pub is_open: bool,
    pub scfm: f32,
}

pub struct Grower {
    pub scfm: f32,
}

pub struct Source {
    pub max_scfm: f32,
}

pub struct Sink {
    pub max_scfm: f32,
}

impl Pipe {
    pub fn new(from: usize, to: usize) -> Self {
        Self { src_node: from, dest_node: to, is_open: true, scfm: 0.0 }
    }
}

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

    use super::{World, Node, Grower};


    #[test]
    fn test_new_model() {
        let mut world = World::new();
        let g1 = world.new_entity();
        world.add_component_to_entity(g1, Grower::new());

        assert_eq!(g1, 0);
    }

    #[test]
    fn test_growers() {
        let mut world = World::new();
        let g1 = world.new_entity();
        world.add_component_to_entity(g1, Grower::new());
        let s1 = world.new_entity();
        world.add_component_to_entity(s1, Source::new(100.));

        let data = world.borrow_component_vec::<Grower>().unwrap();
        let growers: Vec<&Grower> = data.iter()
            .filter_map(|f| f.as_ref())
            .collect();

        assert_eq!(g1, 0);
        assert_eq!(s1, 1);
        assert_eq!(growers.len(), 1);
    }

    #[test]
    fn test_nodes_growers() {
        let mut world = World::new();
        let g1 = world.new_entity();
        world.add_component_to_entity(g1, Grower::new());
        world.add_component_to_entity(g1, Node::new());
        let s1 = world.new_entity();
        world.add_component_to_entity(s1, Source::new(100.));
        world.add_component_to_entity(s1, Node::new());

        let data: Vec<(&Grower, &Node)> = world.borrow_component_vec::<Grower>()
            .unwrap()
            .iter()
            .zip(world.borrow_component_vec::<Node>().unwrap().iter())
            .filter_map(|(grower, node)| Some((grower.as_ref()?, node.as_ref()?)))
            .collect();

        assert_eq!(g1, 0);
        assert_eq!(s1, 1);
        assert_eq!(data.len(), 1);
    }
}