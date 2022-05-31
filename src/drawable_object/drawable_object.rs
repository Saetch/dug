use crate::view::renderer::Vertex;

pub trait DrawableObject {
    fn construct_vertices(&self, camera_position: (f64, f64) ) -> [Vertex; 6];

    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);
}
