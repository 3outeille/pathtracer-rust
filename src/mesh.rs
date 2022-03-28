use nalgebra::Vector3;
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    rc::Rc,
};

use crate::{objects::Triangle, engine::Engine, texture_material::TextureMaterial};

fn default_path() -> String {
    return "".to_string();
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mesh {
    #[serde(default = "default_path")]
    pub path: String,
    pub textmat: TextureMaterial,
}

impl Mesh {
    pub fn convert_to_triangles(&self, engine: &mut Engine) -> () {
        let faces = match self.parse_obj_file() {
            Ok(res) => res,
            Err(error) => panic!("Problem in parsing obj file '{}': {:?}'", self.path, error),
        };

        self.triangularization(engine, &faces);
    }

    pub fn parse_obj_file(
        &self,
    ) -> Result<Vec<(Vector3<f32>, Vector3<f32>, Vector3<f32>)>, io::Error> {
        let f = File::open(&self.path)?;
        let f = BufReader::new(f);

        let mut vertices: Vec<Vector3<f32>> = Vec::new();
        let mut faces: Vec<(Vector3<f32>, Vector3<f32>, Vector3<f32>)> = Vec::new();

        for line in f.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace();

            match tokens.next().unwrap() {
                "v" => {
                    let xyz = tokens
                        .map(|val| val.parse::<f32>().unwrap())
                        .collect::<Vec<f32>>();

                    vertices.push(Vector3::new(xyz[0], xyz[1], xyz[2]));
                }
                "f" => {
                    let v0v1v2 = tokens
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    // Obj model starts index at 1 instead of 0.
                    let face = (
                        vertices[v0v1v2[0] - 1],
                        vertices[v0v1v2[1] - 1],
                        vertices[v0v1v2[2] - 1],
                    );
                    faces.push(face);
                }
                _ => {} // TODO: parse vt and vn
            }
        }

        return Ok(faces);
    }

    pub fn triangularization(
        &self,
        engine: &mut Engine,
        faces: &Vec<(Vector3<f32>, Vector3<f32>, Vector3<f32>)>,
    ) -> () {
        for (v0, v1, v2) in faces {
            engine.add_object(Rc::new(Triangle {
                v0: *v0,
                v1: *v1,
                v2: *v2,
                textmat: self.textmat,
            }));
        }
    }
}
