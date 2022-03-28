use nalgebra::Vector3;
use serde::{de::Error, Deserialize};
use std::{
    collections::HashMap,
    f32::INFINITY,
    fs::{self, File},
    path::Path,
    rc::Rc,
};
// use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

use crate::{
    camera::Camera,
    light::{self, PointLight},
    mesh::Mesh,
    objects::{Plane, Sphere, Triangle},
};

use {crate::ray::*, crate::engine::*};

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
