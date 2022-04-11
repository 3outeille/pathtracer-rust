use nalgebra::{Rotation3, Vector3};
use serde::Deserialize;
use std::{
    f64::consts::PI,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{
    engine::Engine,
    objects::{Mesh, Triangle},
    texture_material::TextureMaterial,
};

#[derive(Clone, Debug, Deserialize)]
pub struct MeshConfig {
    pub path: String,
    pub origin: Vector3<f64>,
    pub scale: f64,
    pub rotation: Vector3<f64>,
    pub textmat: TextureMaterial,
}

impl MeshConfig {
    pub fn convert_to_triangles(&self, engine: &mut Engine) -> () {
        let faces = match self.parse_obj_file() {
            Ok(res) => res,
            Err(error) => panic!("Problem in parsing obj file '{}': {:?}'", self.path, error),
        };

        self.triangularization(engine, &faces);
    }

    pub fn parse_obj_file(&self) -> Result<Vec<[Vector3<f64>; 3]>, io::Error> {
        let f = File::open(&self.path)?;
        let f = BufReader::new(f);

        let mut vertices: Vec<Vector3<f64>> = Vec::new();
        let mut faces = Vec::new();

        let euler_angle = self.rotation / 180. * PI;
        let rotation_matrix =
            Rotation3::from_euler_angles(euler_angle.x, euler_angle.y, euler_angle.z);

        for line in f.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace();

            match tokens.next().unwrap() {
                "v" => {
                    let xyz = tokens
                        .map(|val| val.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>();

                    vertices.push(
                        (rotation_matrix * Vector3::new(xyz[0], xyz[1], xyz[2])) * self.scale
                            + self.origin,
                    );
                }
                "f" => {
                    let v0v1v2 = tokens
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    // Obj model starts index at 1 instead of 0.
                    let face = [
                        vertices[v0v1v2[0] - 1],
                        vertices[v0v1v2[1] - 1],
                        vertices[v0v1v2[2] - 1],
                    ];
                    faces.push(face);
                }
                _ => {} // TODO: parse vt and vn
            }
        }

        return Ok(faces);
    }

    pub fn triangularization(&self, engine: &mut Engine, faces: &Vec<[Vector3<f64>; 3]>) -> () {
        let mut triangles = vec![];

        for [v0, v1, v2] in faces {
            
            triangles.push(Triangle {
                v0: *v0,
                v1: *v1,
                v2: *v2,
                textmat: self.textmat,
            });
        }

        let mut bounds = [Vector3::zeros(); 2];
        
        for triangle in faces {
            for vertex in triangle {
                bounds[0].x = vertex.x.min(bounds[0].x);
                bounds[0].y = vertex.y.min(bounds[0].y);
                bounds[0].z = vertex.z.min(bounds[0].z);
                bounds[1].x = vertex.x.max(bounds[1].x);
                bounds[1].y = vertex.y.max(bounds[1].y);
                bounds[1].z = vertex.z.max(bounds[1].z);
            }
        }

        engine.add_object(Box::new(Mesh {
            triangles,
            bounds,
            textmat: self.textmat,
        }));
    }
}
