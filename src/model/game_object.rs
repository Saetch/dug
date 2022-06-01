use crate::drawable_object::drawable_object::DrawableObject;
pub mod debug_object;
pub trait LogicObject {
    fn multiply_object(&self, x: u32) -> Vec<Box<dyn LogicObject>>{

        println!("Called multiplay_object on Object that does not provide a custom implementation, skipping!");
        return Vec::new();

    }

}

pub trait GameObject: LogicObject+DrawableObject {}