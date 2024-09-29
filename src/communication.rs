pub struct Context {
    context_parameter: Option<Vec<i32>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            context_parameter: None,
        }
    }

    pub fn set_context(&mut self, context_parameter: Vec<i32>) {
        self.context_parameter = Some(context_parameter);
    }

    pub fn get_context(&self) -> Option<Vec<i32>> {
        self.context_parameter.clone()
    }
}

pub trait Communicator {
    async fn talk(&mut self, message: &str) -> String;
}
