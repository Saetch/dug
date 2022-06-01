use crate::view::renderer::Vertex;

pub trait DrawableObject {
    fn construct_vertices(&self, camera_position: (f64, f64) ) -> [Vertex; 6];

    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);


    /**
     * the relative position of the top right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_right_coords(&self) -> [f32; 2]{
        //const expressions are evaluated at compile time and thus can be used to explicitly tell the compiler to optimize this. Would most likely happen anyway.
        const ret: [f32; 2] = [1.0, 0.0];
        return ret;
    }
    
    /**
     * the relative position of the top left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_left_coords(&self) -> [f32; 2]{
        const ret: [f32; 2] = [0.0, 0.0];
        return ret;
    }
    
    /**
     * the relative position of the bottom right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_right_coords(&self) -> [f32; 2]{
        const ret: [f32; 2] = [1.0, 1.0];
        return ret;
    }
    
    /**
     * the relative position of the bottom left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_left_coords(&self) -> [f32; 2]{
        const ret: [f32; 2] = [0.0, 1.0];
        return ret;
    }


}
