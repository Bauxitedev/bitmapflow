use gdnative::{
    api::Node,
    prelude::{user_data::LocalCellError, *},
};

use crate::image_holder::ImageHolder;

//NOTE you can generalize this to any singleton (do_with_singleton<ImageHolder>("ImageHolder", ...))
pub fn do_with_image_holder<T, F: FnOnce(&ImageHolder, TRef<'_, Node>) -> T>(
    f: F,
) -> Result<T, LocalCellError> {
    let imageholder = unsafe { autoload::<Node>("ImageHolder") }
        .unwrap()
        .cast_instance::<ImageHolder>()
        .unwrap();
    imageholder.map(f)
}

pub fn array_to_pair<T>(array: &[T]) -> (&T, &T) {
    match array {
        [a, b] => (a, b),
        _ => panic!("Array size wasn't 2"),
    }
}
