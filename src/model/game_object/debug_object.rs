
use crate::{drawable_object::drawable_object::DrawableObject};

use super::{GameObject, LogicObject};

#[derive(Debug, Clone, Copy)]
pub struct DebugObject{
    pub(crate) position: (f64, f64),
    tex_i: u32,
    pub size: f32,
}

impl DebugObject{
    #[allow(dead_code)]
    pub fn new(position: (f64, f64), _tex_i : u32) -> Self{
        DebugObject { position, tex_i: 0, size: 0.4 }
    }    
    #[allow(dead_code)]
    pub fn new_with_size(position: (f64, f64), tex_i : u32, size: f32) -> Self{
        DebugObject { position, tex_i, size }
    }
    #[allow(dead_code)]
    fn copy(&self)-> Box<dyn DrawableObject> {
        Box::new(DebugObject{
            ..(*self)
        })
    }
}

impl GameObject for DebugObject {}


impl LogicObject for DebugObject {
    
}

unsafe impl Send for DebugObject {
    
}

unsafe impl Sync for DebugObject{
    
}

impl DrawableObject for DebugObject {
    //this can be overridden if needed, to go for custom behavior. 
    //fn construct_vertices(&self, camera_position: (f64, f64), window_dimensions_ingame: (f64,f64)) -> [crate::view::renderer::Vertex; 6] 
    #[inline(always)]
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    #[inline(always)]
    fn get_size(&self) -> f32 {
        self.size
    }
    #[inline(always)]
    fn get_tex_i(&self) -> u32 {
        self.tex_i
    }
    #[inline(always)]
    fn process_animation(&mut self, delta_time: f64) {
        println!("If I had an animation, this would cycle to the next image!, dt: {}", delta_time);
    }




}

