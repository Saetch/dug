
use crate::{drawable_object::drawable_object::DrawableObject, view::renderer::Vertex};

use super::{GameObject, LogicObject};

#[derive(Debug, Clone, Copy)]
pub struct DebugObject{
    pub(crate) position: (f64, f64),
    tex_i: u32,
    pub size: f32,
}

impl GameObject for DebugObject {}


impl LogicObject for DebugObject {
    
}

unsafe impl Send for DebugObject {
    
}

unsafe impl Sync for DebugObject{
    
}

impl DrawableObject for DebugObject {
    fn construct_vertices(&self, camera_position: (f64, f64) ) -> [crate::view::renderer::Vertex; 6] {
        let x = ( self.position.0 - camera_position.0 ) as f32;
        let y = ( self.position.1 - camera_position.1 ) as f32;
        let size = self.size;
        let tex_i = self.tex_i;
        [
            Vertex{
                position: [x+size, y-size],
                tex_i,
                coords: self.top_right_coords(),
            },
            Vertex{
                position: [x-size, y-size],
                tex_i,
                coords: self.top_left_coords(),
            },
            Vertex{
                position: [x-size, y+size],
                tex_i,
                coords: self.bottom_left_coords(),
            },
            Vertex{
                position: [x+size, y-size],
                tex_i,
                coords: self.top_right_coords(),
            },
            Vertex{
                position: [x-size, y+size],
                tex_i,
                coords: self.bottom_left_coords(),
            },
            Vertex{
                position: [x+size, y+size],
                tex_i,
                coords: self.bottom_right_coords(),
            }
            
        ]
    }

    fn process_animation(&mut self, delta_time: f64) {
        println!("If I had an animation, this would cycle to the next image!, dt: {}", delta_time);
    }


}