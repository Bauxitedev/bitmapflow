use gdnative::{
    api::{PopupDialog, RichTextLabel, OS},
    prelude::*,
};

type Base = PopupDialog;
#[derive(NativeClass, Default)]
#[inherit(Base)]
pub struct AboutPopup {}

impl AboutPopup {
    fn new(_owner: &Base) -> Self {
        AboutPopup::default()
    }
}

#[methods]
impl AboutPopup {
    #[export]
    fn _ready(&mut self, owner: &Base) {
        let extra_info = unsafe { owner.get_node_as::<RichTextLabel>("ExtraInfo").unwrap() };

        let text = format!(
            "{} v{}\n{} ({})\nBuild date: {} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("VERGEN_CARGO_TARGET_TRIPLE"),
            env!("VERGEN_CARGO_PROFILE"),
            env!("VERGEN_BUILD_DATE"),
            env!("VERGEN_BUILD_TIME"),
        );

        //This is needed to avoid escaping BBCode
        extra_info.clear();
        extra_info.push_align(RichTextLabel::ALIGN_RIGHT);
        extra_info.add_text(text);
        extra_info.pop();
    }
    #[export]
    fn _on_followme_meta_clicked(&mut self, _owner: &Base, meta: String) {
        if let Err(err) = OS::godot_singleton().shell_open(meta) {
            godot_warn!("Failed to open meta: {}", err);
        }
    }
}
