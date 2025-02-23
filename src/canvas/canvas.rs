use crate::entities::color::Color;

trait CanvasContainer {
    fn create(width: usize, height: usize) -> Self; 
    fn pixel_at(&self, x: usize, y: usize) -> &Color; 
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
}

struct VectorCanvas {
    width: usize, 
    height: usize, 
    canvas: Vec<Vec<Color>>,
    default_color: Color 
}

impl CanvasContainer for VectorCanvas {
    fn create(width: usize, height: usize) -> VectorCanvas {
        let row: Vec<Color> = vec![Color::create(0_f64, 0_f64, 0_f64); width];

        VectorCanvas {
            width: width, 
            height: height, 
            canvas: vec![row.clone(); height],
            default_color: Color::create(0_f64, 0_f64, 0_f64)
        }
    }

    // TODO: 
    //  * Properly handle x, y outside of width, height. 
    //  * Look into using Result<Color,Error?>
    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.canvas[x][y]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[x][y] = c; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c: VectorCanvas = VectorCanvas::create(10, 20);
        
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        
        let row_iter = c.canvas.iter(); 
        for row in row_iter {
            for elem in row.iter() {
                assert_eq!(elem.clone(), Color::create(0.0, 0.0, 0.));
            }
        }
        // assert_eq!(c.canvas[0][0], Color::create(0.0, 0.0, 0.0)); 
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut c: VectorCanvas = VectorCanvas::create(10, 20);
        let red: Color = Color::create(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3).clone(), Color::create(1.0, 0.0, 0.0));
    }
}