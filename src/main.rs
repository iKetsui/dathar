use image::{ImageReader, Rgb, RgbImage, GrayImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let import_img = ImageReader::open("src/qais.png")?.decode()?;
    let mut buffer = import_img.clone().into_rgb8();
    let threshold = 100;
    let factor = 4;
    let (width,height) = buffer.dimensions();
    let mut img = RgbImage::new(width,height);
    let method = 1;


    if method == 1 {
        for (x,y,pixel) in buffer.enumerate_pixels_mut(){
            let mut r = if pixel[0] < threshold {0} else{255}; 
            let mut g = if pixel[1] < threshold {0} else{255}; 
            let mut b = if pixel[2] < threshold {0} else{255}; 

            println!("{}", r);

            img.put_pixel(x,y,Rgb([r,g as u8 ,b as u8]));
        }
        img.save("mqyas.png").unwrap();
        println!("image saved");
    }

    if method == 2 {

    }

    Ok(())
}
