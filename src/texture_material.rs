use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct TextureMaterial {
    pub color: Vector3<f32>,
    #[serde(default)]
    pub surface: Surface,
}

impl Default for TextureMaterial {
    fn default() -> Self {
        TextureMaterial {
            color: Vector3::new(0.3, 0.1, 0.1), // red
            surface: Surface::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Surface {
    pub ambient: Ambient,
    pub emittance: Option<Emittance>,
    pub diffuse: Diffuse,
    pub specular: Specular,
    pub reflection: Reflection,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            ambient: Ambient::new(1.0),
            emittance: None,
            diffuse: Diffuse::new(1.0),
            specular: Specular::new(1.0, 15.0),
            reflection: Reflection::new(0.5),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Emittance {
    pub ke: f32,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Ambient {
    pub ka: f32,
}

impl Ambient {
    pub fn new(ka: f32) -> Self {
        Self { ka }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Diffuse {
    pub kd: f32,
}

impl Diffuse {
    pub fn new(kd: f32) -> Self {
        Self { kd }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Specular {
    pub ks: f32,
    pub ns: f32,
}

impl Specular {
    pub fn new(ks: f32, ns: f32) -> Self {
        Self { ks, ns }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Reflection {
    pub kr: f32,
}

impl Reflection {
    pub fn new(kr: f32) -> Self {
        Self { kr }
    }
}
