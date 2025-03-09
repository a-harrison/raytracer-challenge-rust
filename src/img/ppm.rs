use crate::img::{ canvas::Canvas, color::Color };

// trait ImageWriter {
//     fn write_image(c: Canvas, f: ImageFormatter, file_name: String ) -> String;
// }

trait ImageFormat {
    fn format<T: Canvas>(c: &mut T) -> String; 
}


#[allow(dead_code, unused)]
struct PPMImageFormat;

#[allow(dead_code, unused)]
impl PPMImageFormat {
    const MIN_COLOR_VALUE: u8 = 0;
    const MAX_COLOR_VALUE: u8 = 255;    

    fn build_header<T: Canvas>(c: &T) -> String {
        format!("P3\n{} {}\n255", c.width(), c.height()).to_string()
    }

    fn append_color_tuple(file_data: &mut String, color_tuple: &mut [String; 3], row_len: &mut usize) {
        let mut iter = color_tuple.iter();
        
        for elem in iter {
            if *row_len == 0 as usize {
                file_data.push_str(elem);
                *row_len += elem.len();
            } else if *row_len + elem.len() >= 70 {
                file_data.push_str("\n");
                file_data.push_str(elem);
                *row_len = elem.len();
            } else {
                file_data.push_str(" ");
                file_data.push_str(elem);
                *row_len += elem.len() + 1;
            }
        }
    }

    fn convert_and_append_row(file_data: &mut String, row: &Vec<Color>) {
        let iter = row.iter();
        let mut row_len: usize = 0;
        let mut color_tuple: [String; 3];

        for color in iter {
            color_tuple = PPMImageFormat::color_to_tuple(&color);
            PPMImageFormat::append_color_tuple(file_data, &mut color_tuple, &mut row_len);
            // file_data.push_str(" ");
            // row_len += 1;
        }
    
        file_data.push_str("\n");
        row_len = 0;
    }

    fn convert_canvas<T: Canvas>(c: &mut T) -> String {
        let mut file_data = String::new();
        // let mut row_len: usize = 0;
        let canvas_row_iter = c.iter();
        let mut color_tuple: [String; 3]; 

        for row in canvas_row_iter {
            PPMImageFormat::convert_and_append_row(&mut file_data, row);
        }

        // Remove trailing blank space
        file_data.pop();

        file_data
    }

    fn scale_color_to_range(c: &f64) -> u8 {
        match c {
            c if *c < 0.0 => Self::MIN_COLOR_VALUE,
            c if *c > 1.0 => Self::MAX_COLOR_VALUE as u8, 
            c => (*c * Self::MAX_COLOR_VALUE as f64).round() as u8
        }
    }

    fn color_to_tuple<'a>(c: &Color) -> [String; 3] {
        let output: [String; 3] = [
            PPMImageFormat::scale_color_to_range(c.r()).to_string(), 
            PPMImageFormat::scale_color_to_range(c.g()).to_string(),
            PPMImageFormat::scale_color_to_range(c.b()).to_string()
        ];

        output
    }
}

impl ImageFormat for PPMImageFormat {
    fn format<T: Canvas>(c: &mut T) -> String {
        let header = PPMImageFormat::build_header(c);
        let contents = PPMImageFormat::convert_canvas(c);
        let mut file_contents: String = String::with_capacity(header.len() + contents.len() + 1); 

        file_contents.push_str(&header); 
        file_contents.push_str(&contents);
        file_contents.push_str("\n");
        
        file_contents
    }
}

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

        assert_eq!(PPMImageFormat::color_to_tuple(&c), ["0", "128", "255"]);
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
        let expected: String = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                                            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                                            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        
        assert_eq!(file_contents, expected);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c: VectorCanvas = Canvas::create_with_default_color(10, 2, Color::create(1.0, 0.8, 0.6)); 

        let file_contents: String = PPMImageFormat::convert_canvas(&mut c);
        
        let expected: String = String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                                            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
                                            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                                            153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert_eq!(file_contents, expected);        
    }

    #[test]
    fn ppm_files_terminated_by_newline_character() {
        let mut c: VectorCanvas = Canvas::create(5, 3); 
        let ppm = PPMImageFormat::format(&mut c);

        assert_eq!(ppm.chars().last().unwrap(), 0xA as char);
    }
}