pub struct Context {
    pub context_parameter: Option<Vec<i32>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            context_parameter: None,
        }
    }
}

pub trait Communicator {
    async fn talk(&mut self, message: &str) -> String;
}
