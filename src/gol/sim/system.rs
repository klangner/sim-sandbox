use super::Params;


pub struct System {
    pub params: Params,
}

impl System {
    pub fn init(params: Params) -> Self {
        Self { params }
    }

    pub fn next_step(&mut self) {

    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_map() {
        // 2 ports with 2 routes
    }
}