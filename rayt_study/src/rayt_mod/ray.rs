// 光線．視点と向きを持つ．
use crate::rayt_mod::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    // パラメータtを指定して始点から特定方向を指すベクトルを返す．
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
