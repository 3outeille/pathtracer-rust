use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct TextureMaterial {
    pub color: Vector3<f64>,
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
    pub emittance: Option<Emittance>,
    pub diffuse: Diffuse,
    pub specular: Specular,
    pub reflection: Reflection,
    pub transmission: Transmission,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            emittance: None,
            diffuse: Diffuse::new(1.0),
            specular: Specular::new(1.0, 15.0),
            reflection: Reflection::new(0.5),
            transmission: Transmission::new(0.5),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Emittance {
    pub ke: f64,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Diffuse {
    pub kd: f64,
}

impl Diffuse {
    pub fn new(kd: f64) -> Self {
        Self { kd }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Specular {
    pub ks: f64,
    pub ns: f64,
}

impl Specular {
    pub fn new(ks: f64, ns: f64) -> Self {
        Self { ks, ns }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Reflection {
    pub kr: f64,
}

impl Reflection {
    pub fn new(kr: f64) -> Self {
        Self { kr }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Transmission {
    pub kt: f64,
}

impl Transmission {
    pub fn new(kt: f64) -> Self {
        Self { kt }
    }
}
