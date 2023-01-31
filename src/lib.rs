#![allow(unused)]
mod also_lib;

pub mod nest1 {
    pub mod nest2 {
        pub fn dummy_func() {}
        #[derive(Debug)]
        pub struct DummyStruct {
            pub pubint: i32,
            privstr: String,
        }

        impl DummyStruct {
            pub fn new() -> Self {
                DummyStruct {
                    pubint: 1,
                    privstr: String::from("new"),
                }
            }
        }
    }
}

pub use crate::also_lib::exported_module;

pub fn lets_see() {
    nest1::nest2::dummy_func();
    let dsi = nest1::nest2::DummyStruct::new();
    println!("{:?}", dsi)
}
