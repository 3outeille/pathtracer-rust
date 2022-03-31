use serde::Deserialize;

use crate::{
    camera::Camera,
    light::PointLight,
    mesh::Mesh,
    objects::{Plane, Sphere, Triangle},
};

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    #[serde(default = "Vec::new")]
    pub spheres: Vec<Sphere>,
    #[serde(default = "Vec::new")]
    pub triangles: Vec<Triangle>,
    #[serde(default = "Vec::new")]
    pub planes: Vec<Plane>,
    #[serde(default = "Vec::new")]
    pub meshes: Vec<Mesh>,
}
