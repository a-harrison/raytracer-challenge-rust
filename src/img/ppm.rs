use crate::img::{ canvas::Canvas, color::Color };

// trait ImageWriter {
//     fn write_image(c: Canvas, f: ImageFormatter, file_name: String ) -> String;
// }

trait ImageFormat {
    fn format<T: Canvas>(c: &mut T) -> String; 
}

struct PPMImageFormat;

impl PPMImageFormat {
    const MIN_COLOR_VALUE: u8 = 0;
    const MAX_COLOR_VALUE: u8 = 255;    

    fn build_header<T: Canvas>(c: &T) -> String {
        format!("P3\n{} {}\n255", c.width(), c.height()).to_string()
    }

    fn convert_canvas<T: Canvas>(c: &mut T) -> String {
        let mut file_data = String::new();
        let canvas_iter = c.iter();

        // TODO: Find a better way to do the join here.
        let mut not_first_elem: bool = false;

        for elem in canvas_iter {
            if not_first_elem {
                file_data.push_str(" ");
            } else {
                not_first_elem = true;
            }
            file_data.push_str(&Self::convert_color(&elem));
        }

        file_data
    }

    fn scale_color_to_range(c: &f64) -> u8 {
        match c {
            c if *c < 0.0 => Self::MIN_COLOR_VALUE,
            c if *c > 1.0 => Self::MAX_COLOR_VALUE as u8, 
            c => (*c * Self::MAX_COLOR_VALUE as f64).round() as u8
        }
    }

    fn convert_color(c: &Color) -> String {
        let mut output = String::with_capacity(11);
        let red = PPMImageFormat::scale_color_to_range(c.r());
        let green = PPMImageFormat::scale_color_to_range(c.g());
        let blue = PPMImageFormat::scale_color_to_range(c.b());

        output.push_str(&red.to_string());
        output.push_str(" ");
        output.push_str(&green.to_string());
        output.push_str(" ");
        output.push_str(&blue.to_string());

        output
    }
}

impl ImageFormat for PPMImageFormat {
    fn format<T: Canvas>(c: &mut T) -> String {
        let header = PPMImageFormat::build_header(c);
        let contents = PPMImageFormat::convert_canvas(c);
        let mut file_contents: String = String::new(); 

        file_contents.push_str(&header); 
        file_contents.push_str(&contents);
        
        file_contents
    }
}

// let writer: PPMWriter = ImageWriter::create(); 
// writer.co

#[cfg(test)]
mod tests {
    use super::*;
    use crate::img::canvas::VectorCanvas;

    #[test]
    fn constructing_ppm_header() {
        let c: VectorCanvas = Canvas::create(5, 3);
        let expected_header: String = "P3\n5 3\n255".to_string();

        let ppm_header: String = PPMImageFormat::build_header(&c);

        assert_eq!(ppm_header, expected_header);
    }

    #[test]
    fn convert_negative_value_to_range() {
        assert_eq!(PPMImageFormat::scale_color_to_range(&-1.5), 0);
    }

    #[test]
    fn convert_excess_value_to_range() {
        assert_eq!(PPMImageFormat::scale_color_to_range(&1.5), 255);
    }

    #[test]
    fn convert_value_to_range() {
        assert_eq!(PPMImageFormat::scale_color_to_range(&0.5_f64), 128);
    }

    #[test]
    fn convert_color_to_string_tuple() {
        let c: Color = Color::create(-1.5, 0.5, 1.5);

        assert_eq!(PPMImageFormat::convert_color(&c), "0 128 255".to_string());
    }

    #[test]
    fn construct_ppm_pixel_data() {
        let mut c: VectorCanvas = Canvas::create(5, 3);
        
        let c1 = Color::create(1.5, 0.0, 0.0); 
        let c2 = Color::create(0.0, 0.5, 0.0); 
        let c3 = Color::create(-0.5, 0.0, 1.0); 

        c.write_pixel(0, 0, c1); 
        c.write_pixel(2, 1, c2); 
        c.write_pixel(4, 2, c3); 

        let file_contents: String = PPMImageFormat::convert_canvas(&mut c);
        let expected: String = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \
                                            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \
                                            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        
        assert_eq!(file_contents, expected);
    }
}