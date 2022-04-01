use std::f32::consts::PI;

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

impl Surface {
    /*
    http://www.codinglabs.net/article_physically_based_rendering_cook_torrance.aspx
    https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf
    */
    pub fn get_bsdf(
        &self,
        normal: Vector3<f32>,
        wi: Vector3<f32>,
        wo: Vector3<f32>,
    ) -> Vector3<f32> {
        // BSDF = BTDF + BRDF

        // BTDF (transparence)
        let btdf = { Vector3::zeros() };

        // BDRF = kd * diffuse + ks * specular
        let brdf = {
            // Diffuse: Lambert
            let diffuse = Vector3::new(1. / PI, 1. / PI, 1. / PI);

            // Specular: Cook-Torrance BRDF = DFG / (4(n \cdot wi)(n \cdot wo))
            let specular = Vector3::zeros();

            // D: microfacet distribution function
            // D = exp(((n \cdot h)^2 - 1) / (m^2 (n \cdot h)^2)) / (pi m^2 (n \cdot h)^4)

            // F: fresnel, schlick's approximation
            // F = F0 + (1 - F0)(1 - wi \cdot h)^5

            // G: geometry function, microfacet shadowing
            // G = min(1, 2(n \cdot h)(n \cdot wo)/(wo \cdot h), 2(n \cdot h)(n \cdot wi)/(wo \cdot h))

            (self.diffuse.kd * diffuse) + (self.specular.ks * specular)
        };

        return btdf + brdf;
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
