

//these objects are meant to be used as the background or anything static, without any logic attached to it
pub struct StaticObject{
    pub texture_id: u16,
    pub position: (f64, f64),
}

impl StaticObject {
    pub fn construct_vertices(&self, _camera_position: (f64, f64), _win_dimensions: (f64, f64)) -> [crate::view::renderer::Vertex; 6]{
        todo!();
    }
}