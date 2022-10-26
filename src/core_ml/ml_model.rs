use crate::objective_c_runtime::{
    macros::{interface_impl, object},
    traits::PNSObject,
};

object! {
    /// An encapsulation of all the details of your machine learning model.
    unsafe pub struct MLModel;
}

#[interface_impl(NSObject)]
impl MLModel {}
