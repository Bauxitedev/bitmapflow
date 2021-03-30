use gdnative::prelude::*;

#[derive(NativeClass, Default)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct GlobalHolder {
    pub fps: f64,
}

impl GlobalHolder {
    fn new(_owner: &Node) -> Self {
        GlobalHolder::default()
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property("fps")
            .with_setter(|this, _owner, value: f64| {
                this.fps = if value.is_finite() { value } else { 10. }
            })
            .with_getter(|this, _owner| this.fps)
            .done();
    }
}

#[methods]
impl GlobalHolder {}
