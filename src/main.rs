use image::{ImageReader, Rgb, RgbImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let import_img = ImageReader::open("src/qais.png")?.decode()?;
    let mut buffer = import_img.clone().into_rgb8();
    //let mut buffer = import_img.clone().grayscale().into_rgb8();
    let threshold = 100;
    //let factor = 4;
    let (width,height) = buffer.dimensions();
    let mut img = RgbImage::new(width,height);
    let method = 2;

    let mut error_r : u16;
    let mut error_g : u16;
    let mut error_b : u16;

    let mut new_r : u16;
    let mut new_g : u16;
    let mut new_b : u16;

    let mut u8r : u8;
    let mut u8g : u8;
    let mut u8b : u8;


    if method == 1 {
        for (x,y,pixel) in buffer.enumerate_pixels_mut(){
            let  r = if pixel[0] < threshold {0} else{255}; 
            let  g = if pixel[1] < threshold {0} else{255}; 
            let  b = if pixel[2] < threshold {0} else{255}; 

            println!("{}", r);

            img.put_pixel(x,y,Rgb([r,g,b]));
        }
        img.save("mqyas.png").unwrap();
        println!("image saved");
    }

    if method == 2 {
        for (x,y,pixel) in buffer.enumerate_pixels_mut(){
            let r = if pixel[0] < threshold {0} else{255}; 
            let g = if pixel[1] < threshold {0} else{255}; 
            let b = if pixel[2] < threshold {0} else{255}; 

            
            println!("{}", r);
            
            if pixel[0] >= r {error_r = pixel[0] as u16 - r as u16} else {error_r = 0}
            if pixel[1] >= g {error_g = pixel[1] as u16 - g as u16} else {error_g = 0}
            if pixel[2] >= b {error_b = pixel[2] as u16 - b as u16} else {error_b = 0}

            // section 1 
            if x+1 < width{
            let shift_x = x+1; 
            let shift_y = y;

            new_r = error_r * 7/16;
            new_g = error_g * 7/16;
            new_b = error_b * 7/16;

            u8r = pixel[0] + new_r as u8;
            u8g = pixel[1] + new_g as u8;
            u8b = pixel[2] + new_b as u8;

            img.put_pixel(shift_x,shift_y,Rgb([u8r,u8g,u8b]));
            }

            // section 2
            if x > 0 && y + 1 < height {
            let shift_x = x-1;
            let shift_y = y+1;
            

            new_r = error_r * 3/16;
            new_g = error_g * 3/16;
            new_b = error_b * 3/16;


            u8r = pixel[0] + new_r as u8;
            u8g = pixel[1] + new_g as u8;
            u8b = pixel[2] + new_b as u8;


            img.put_pixel(shift_x,shift_y,Rgb([u8r,u8g,u8b]));
            }

            // section 3 
            if y + 1 < height {
            let shift_x = x;
            let shift_y = y+1;
            



            new_r = error_r * 5/16;
            new_g = error_g * 5/16;
            new_b = error_b * 5/16;


            u8r = pixel[0] + new_r as u8;
            u8g = pixel[1] + new_g as u8;
            u8b = pixel[2] + new_b as u8;

            img.put_pixel(shift_x,shift_y,Rgb([u8r,u8g,u8b]));
            }


            // section 4 
            if x + 1 < width && y + 1 < height {
            let shift_x = x+1;
            let shift_y = y+1;
            

            new_r = error_r * 1/16;
            new_g = error_g * 1/16;
            new_b = error_b * 1/16;

            u8r = pixel[0] + new_r as u8;
            u8g = pixel[1] + new_g as u8;
            u8b = pixel[2] + new_b as u8;

            img.put_pixel(shift_x,shift_y,Rgb([u8r,u8g,u8b]));
            }
        }
        img.save("mqyas.png").unwrap();
        println!("debuj: img saved");
    }

    Ok(())
}
