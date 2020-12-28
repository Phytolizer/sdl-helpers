use lazy_static::lazy_static;
use nalgebra::Matrix3;
use parking_lot::RwLock;

lazy_static! {
    pub(crate) static ref MATRIX_STACK: RwLock<Vec<Matrix3<f64>>> =
        RwLock::new(vec![Matrix3::identity()]);
}
