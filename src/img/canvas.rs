use crate::img::color::Color;

pub trait Canvas {
    fn create(width: usize, height: usize) -> Self; 
    fn pixel_at(&self, x: usize, y: usize) -> &Color; 
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> usize; 
    fn iter(&mut self) -> impl Iterator<Item = Color>;
}

pub struct VectorCanvas {
    pub width: usize, 
    pub height: usize, 
    pub canvas: Vec<Vec<Color>>,
}

pub struct VectorCanvasIterator<'a> {
    canvas: &'a VectorCanvas,
    current_row: usize,
    current_column: usize
}

#[allow(dead_code)]
struct RowIteratorHolder<I: Iterator<Item = Vec<Color>>> {
    iter: I
}

// pub struct OtherIterator<'a> {
//     canvas: &'a VectorCanvas,
//     row_iter: Box<dyn Iterator<Item = Vec<Color>>>,
//     elem_iter: Box<dyn Iterator<Item = Color>>
//     // row: usize, 
//     // current_iter: Box<dyn Iterator<Item = Color>>
// }

// impl OtherIterator<'a> {
//     fn create(&self, canvas: &VectorCanvas) -> Self {
//         let canvas_row_iter = canvas.canvas.iter();
//         let elem_iter = canvas_row_iter.next().iter(); 

//         OtherIterator {
//             canvas: canvas,
//             row_iter : canvas_row_iter,
//             current_iter: canvas.canvas.iter().iter()
//         }
//     }
// }

impl<'a> Iterator for VectorCanvasIterator<'a> {
    type Item = Color; 

    fn next(&mut self) -> Option<Self::Item> {
        // Go up one row
        if self.current_column == self.canvas.width {
            self.current_row = self.current_row + 1;
            self.current_column = 0;
        }

        // End of the canvas
        if self.current_row == self.canvas.height {
            return None 
        }

        let return_color = Some(self.canvas.pixel_at(self.current_column, self.current_row).clone());

        self.current_column += 1; 

        return_color
    }
}

impl Canvas for VectorCanvas {
    fn create(width: usize, height: usize) -> VectorCanvas {
        let row: Vec<Color> = vec![Color::create(0_f64, 0_f64, 0_f64); width];

        VectorCanvas {
            width: width, 
            height: height, 
            canvas: vec![row.clone(); height],
        }
    }

    // TODO: 
    //  * Properly handle x, y outside of width, height. 
    //  * Look into using Result<Color,Error?>
    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.canvas[y][x]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[y][x] = c; 
    }

    fn iter(&mut self) -> impl Iterator<Item = Color> {
        VectorCanvasIterator {
            canvas: self,
            current_row: 0,
            current_column: 0
        }
    }

    fn size(&self) -> usize {
        self.width * self.height 
    }

    fn width(&self) -> usize {
        return self.width
    }

    fn height(&self) -> usize {
        return self.height
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