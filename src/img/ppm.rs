use crate::img::{canvas::Canvas, canvas::VectorCanvas, color::Color };

// trait ImageWriter {
//     fn write_image(c: Canvas, f: ImageFormatter, file_name: String ) -> String;
// }

trait ImageFormatter {
    fn format<T: Canvas>(c: &mut T) -> String; 
}

const MIN_COLOR_VALUE: u8 = 0;
const MAX_COLOR_VALUE: u8 = 255;

impl PPMImageFormatter {
    fn build_header<T: Canvas>(c: &T) -> String {
        format!("P3\n{} {}\n255", c.width(), c.height()).to_string()
    }

    fn convert_canvas<T: Canvas>(c: &mut T) -> String {
        let mut file_data = String::new();
        let canvas_iter = c.iter();
        // let first_entry = canvas_iter.next(); 

        // match first_entry {
        //     Some(first_entry) => {
        //         file_data.push_str(&Self::convert_color(&first_entry));
        //     },
        //     None => {
        //         return file_data
        //     }
        // }
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
            c if *c < 0.0 => MIN_COLOR_VALUE,
            c if *c > 1.0 => MAX_COLOR_VALUE as u8, 
            c => (*c * MAX_COLOR_VALUE as f64).round() as u8
        }
    }

    fn convert_color(c: &Color) -> String {
        let mut output = String::with_capacity(11);
        let red = PPMImageFormatter::scale_color_to_range(c.r());
        let green = PPMImageFormatter::scale_color_to_range(c.g());
        let blue = PPMImageFormatter::scale_color_to_range(c.b());

        output.push_str(&red.to_string());
        output.push_str(" ");
        output.push_str(&green.to_string());
        output.push_str(" ");
        output.push_str(&blue.to_string());

        output
    }
}

impl ImageFormatter for PPMImageFormatter {
    fn format<T: Canvas>(c: &mut T) -> String {
        let header = PPMImageFormatter::build_header(c);
        let contents = PPMImageFormatter::convert_canvas(c);
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

    #[test]
    fn constructing_ppm_header() {
        let mut c: VectorCanvas = Canvas::create(5, 3);
        let expected_header: String = "P3\n5 3\n255".to_string();

        let ppm_header: String = PPMImageFormatter::build_header(&c);

        assert_eq!(ppm_header, expected_header);
    }

    #[test]
    fn convert_negative_value_to_range() {
        assert_eq!(PPMImageFormatter::scale_color_to_range(&-1.5), 0);
    }

    #[test]
    fn convert_excess_value_to_range() {
        assert_eq!(PPMImageFormatter::scale_color_to_range(&1.5), 255);
    }

    #[test]
    fn convert_value_to_range() {
        assert_eq!(PPMImageFormatter::scale_color_to_range(&0.5_f64), 128);
    }

    #[test]
    fn convert_color_to_string_tuple() {
        let c: Color = Color::create(-1.5, 0.5, 1.5);

        assert_eq!(PPMImageFormatter::convert_color(&c), "0 128 255".to_string());
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

        let file_contents: String = PPMImageFormatter::convert_canvas(&mut c);
        let expected: String = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \
                                            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \
                                            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        
        assert_eq!(file_contents, expected);
    }
}