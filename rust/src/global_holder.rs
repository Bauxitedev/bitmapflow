use gdnative::prelude::*;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct GlobalHolder {
    #[property(path = "fps")]
    pub fps: f64,
}

impl GlobalHolder {
    fn new(_owner: &Node) -> Self {
        GlobalHolder::default()
    }
}

#[methods]
impl GlobalHolder {}
