use crate::view::renderer::Vertex;

pub trait DrawableObject {
    /**
     * Default implementation for a somewhat rectangle-shaped object
     */
    #[inline(always)]
    fn construct_vertices(&self, camera_position: (f64, f64), window_dimensions_ingame: (f64, f64)) -> [Vertex; 6]{
        let x = ( self.get_position().0 - camera_position.0 ) as f32;
        let y = ( self.get_position().1 - camera_position.1 ) as f32;
        let size_x = self.get_size() / (window_dimensions_ingame.0 as f32) * 2.0;
        let size_y = self.get_size() / (window_dimensions_ingame.1 as f32) * 2.0;
        let tex_i = self.get_tex_i();
        [
            Vertex{
                position: [x+size_x, y-size_y],
                tex_i,
                coords: self.top_right_coords(),
            },
            Vertex{
                position: [x-size_x, y-size_y],
                tex_i,
                coords: self.top_left_coords(),
            },
            Vertex{
                position: [x-size_x, y+size_y],
                tex_i,
                coords: self.bottom_left_coords(),
            },
            Vertex{
                position: [x+size_x, y-size_y],
                tex_i,
                coords: self.top_right_coords(),
            },
            Vertex{
                position: [x-size_x, y+size_y],
                tex_i,
                coords: self.bottom_left_coords(),
            },
            Vertex{
                position: [x+size_x, y+size_y],
                tex_i,
                coords: self.bottom_right_coords(),
            }
            
        ]
    }


    fn get_position(&self) -> (f64, f64);
    fn get_size(&self) -> f32;
    fn get_tex_i(&self) -> u32;

    //Consider making this interior mutable, in order to speed up access to these 
    fn process_animation(&mut self, delta_time: f64);


    /**
     * the relative position of the top right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_right_coords(&self) -> [f32; 2]{
        //const expressions are evaluated at compile time and thus can be used to explicitly tell the compiler to optimize this. Would most likely happen anyway.
        const RET: [f32; 2] = [1.0, 0.0];
        return RET;
    }
    
    /**
     * the relative position of the top left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn top_left_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [0.0, 0.0];
        return RET;
    }
    
    /**
     * the relative position of the bottom right vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_right_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [1.0, 1.0];
        return RET;
    }
    
    /**
     * the relative position of the bottom left vertex if overlayed on top of the image that is to be drawn
     */
    #[inline(always)]
    fn bottom_left_coords(&self) -> [f32; 2]{
        const RET: [f32; 2] = [0.0, 1.0];
        return RET;
    }


}
