use std::{ffi::CString, os::raw};

use violin_rs::{
    image::Image,
    item::{component::ItemDisplayNameComponent, Item},
    pack::Pack,
    vio::{Buildable, Identifier, SemVer},
};

use crate::utils::RawString;

fn pack_ref<'a>(pointer: *mut Pack) -> &'a mut Pack {
    assert!(!pointer.is_null(), "Received a null pointer");
    unsafe { &mut *pointer }
}

fn raw_to_string(raw: RawString) -> String {
    unsafe { CString::from_raw(raw).to_str().unwrap().to_string() }
}

#[no_mangle]
pub fn create_pack(
    name: RawString,
    id: RawString,
    author: RawString,
    description: RawString,
    dev_bp_folder: RawString,
    dev_rp_folder: RawString,
    icon_path: RawString,
) -> *mut Pack {
    let pack = Pack::new(
        raw_to_string(name),
        raw_to_string(id),
        raw_to_string(author),
        SemVer::new(0, 0, 1),
        raw_to_string(description),
        raw_to_string(dev_bp_folder),
        raw_to_string(dev_rp_folder),
        Image::new(raw_to_string(icon_path)),
        None,
    );

    Box::into_raw(Box::new(pack))
}

#[no_mangle]
pub fn add_item(pointer: *mut Pack) {
    let pack = pack_ref(pointer);

    pack.register_item(
        Item::new(Identifier::new("violin", "js")).using_components(vec![
            ItemDisplayNameComponent::new("Hello, Violin.js!").build(),
        ]),
    );
}

#[no_mangle]
pub fn gen_pack(pointer: *mut Pack) {
    let pack = pack_ref(pointer);

    pack.generate();
}
