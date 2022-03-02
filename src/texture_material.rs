use nalgebra::Vector3;

pub trait TextureMaterial: {
    fn get_texture(&self) -> (f32, f32, f32, f32, Vector3<f32>);
}

pub struct UniformTexture {
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub ns: f32,
    pub color: Vector3<f32>
}

impl UniformTexture {
    pub fn new(ka_arg: f32, kd_arg: f32, ks_arg: f32, ns_arg: f32, color_arg: Vector3<f32>) -> Self {
        Self {
            ka: ka_arg,
            kd: kd_arg,
            ks: ks_arg,
            ns: ns_arg,
            color: color_arg
        }
    }
}

impl TextureMaterial for UniformTexture {
    fn get_texture(&self) -> (f32, f32, f32, f32, Vector3<f32>) {
        return (self.ka, self.kd, self.ks, self.ns, self.color);
    }
}