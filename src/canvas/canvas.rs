use crate::entities::color::Color;

trait CanvasContainer {
    fn create(width: usize, height: usize) -> Self; 
    fn pixel_at(&self, x: usize, y: usize) -> Color; 
    fn write_pixel(&self, x: usize, y: usize, c: Color);
}

struct VectorCanvas {
    width: usize, 
    height: usize, 
    canvas: Vec<Vec<Color>>, 
}

impl CanvasContainer for VectorCanvas {
    fn create(width: usize, height: usize) -> VectorCanvas {
        let row: Vec<Color> = vec![Color::create(0_f64, 0_f64, 0_f64); width];

        VectorCanvas {
            width: width, 
            height: height, 
            canvas: vec![row.clone(); height]
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        let row = self.canvas.get(y);
        let return_value = Color::create(0_f64,0_f64,0_f64);

        self.canvas.get(y).get(x); 

        match row {
            Some(row) => {
                let col = row.get(x); 
                
                match col {
                    Some(col) => {
                        return col.clone()
                    },
                    None => {}
                }
            },
            None => {}
        }
        
        return_value
    }

    fn write_pixel(&self, x: usize, y: usize, c: &Color) {
        let row: Option<&Vec<Color>> = self.canvas.get(y); 

        match row {
            Some(row) = {

            },
            None => {
                self.canvas.set
            }
        }

    }
}

// fn get_row_from_canvas(canvas: &VectorCanvas, row: usize) -> Vec<Color> {
//     let row = canvas.canvas.get(row); 

//     match canvas.get(row) {
//         Some(row) => {
//             row
//         }
//     }    
// }

// fn get_color_from_row(canvas: &CanvasContainer, column: usize) -> Color {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c: VectorCanvas = VectorCanvas::create(10, 20);
        
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.canvas[0][0], Color::create(0.0, 0.0, 0.0)); 
    }
}