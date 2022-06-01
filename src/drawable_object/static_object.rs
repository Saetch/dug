

//these objects are meant to be used as the background or anything static, that does not need any logic to run on it
pub struct StaticObject{
    pub texture_id: u16,
    pub position: (f64, f64),
}

impl StaticObject {
    pub fn construct_vertices(&self, _camera_position: (f64, f64)) -> [crate::view::renderer::Vertex; 6]{
        todo!();
    }
}