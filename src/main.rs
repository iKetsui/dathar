use image::{ImageReader, Rgb, RgbImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let import_img = ImageReader::open("src/qais.png")?.decode()?;
    let mut buffer = import_img.clone().into_rgb8();
    let dither = 135;
    let (width,height) = buffer.dimensions();
    let mut img = RgbImage::new(width,height);
    let method = 0;

    if method == 1 {
        for (x,y,pixel) in buffer.enumerate_pixels_mut(){
            if (pixel[0] < dither) && (pixel[1] < dither) && (pixel[2] < dither){
                img.put_pixel(x,y,Rgb([0,0,0]));
            }else{
                img.put_pixel(x,y,Rgb([255,255,255]));
            }
        }
        img.save("mqyas.png").unwrap();
        println!("image saved");
    }

    if method == 2 {

    }

    Ok(())
}
