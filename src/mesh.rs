use std::rc::Rc;

use nalgebra::Vector3;

use crate::{texture_material::TextureMaterial, objects::Triangle};

pub struct Mesh {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub space_cube: Vector3<f32>,
    pub offset: f32,
    pub threshold: f32,
    pub textmat: Rc<dyn TextureMaterial>
}

impl Mesh {

    pub fn new(&self, width: usize, height: usize, depth: usize, space_cube: Vector3<f32>, offset: f32, threshold:f32, textmat: Rc<dyn TextureMaterial>) -> Mesh {

        space_cube_orig = ;
        space_cube_dst = ;

        Mesh {
            
        }
    }

    pub fn marching_cube(&self, threshold: f32) -> Vec<Rc<Triangle>> {
        let triangles: Vec<Rc<Triangle>> = Vec::new();

        let curr_cub = self.space_cube.clone();

        for _ in (0, space_cube_dst.z()){
            for _ in (0, space_cube_dst.y() { 
                for _ in (0, space_cube_dst.x() {
                    
                    curr_cub.z += offset;
                    self.generate_triangles(&curr_cub, &triangles);
                }
            
                curr_cub.y += offset;
            }

            curr_cub.x += offset;
        }

        return triangles;
    }

    pub fn generate_triangles(&self, curr_cub: &Vec3<f32>, triangles: &Vec<Rc<Triangle>>) -> () {
        let index = self.vertices_to_index(curr_cub);

        let edges = lookup_table[index];
        
        // Connect edges to get triangles
        self.connect_edges(edges, &triangles);
    }

    pub fn vertices_to_index(&self, curr_cub: &Vec3<f32>) -> u8 {
        let index = 0 as u8;

        if get_potential_at(0, curr_cub) < self.threshold { index |= 1;  }
        if get_potential_at(1, curr_cub) < self.threshold { index |= 2;  }
        if get_potential_at(2, curr_cub) < self.threshold { index |= 4;  }
        if get_potential_at(3, curr_cub) < self.threshold { index |= 8;  }
        if get_potential_at(4, curr_cub) < self.threshold { index |= 16; }
        if get_potential_at(5, curr_cub) < self.threshold { index |= 32; }
        if get_potential_at(6, curr_cub) < self.threshold { index |= 64; }
        if get_potential_at(7, curr_cub) < self.threshold { index |= 128;}

        return index;
    }

    pub fn get_potential_at(&self, vertex: u8, curr_cub: &Vec3<f32>) -> f32 {
        todo!()
    }

    pub fn connect_edges(&self, edges: Vec<u8>, triangles: &Vec<Rc<Triangle>>) -> () {
        todo!()
    }
}