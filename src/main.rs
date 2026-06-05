use image::{DynamicImage, ImageReader, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("src/qais.png")?.decode()?;
    let mut buffer = img.clone().into_rgb8();
    let dither = 160;

    for (x,y,pixel) in buffer.enumerate_pixels_mut(){

        if (pixel[0] < dither) && (pixel[1] < dither) && (pixel[2] < dither){
            *pixel = Rgb([0,0,0]);

        }else{
            *pixel = Rgb([255,255,255])
        }
    }
    println!("{:?}", buffer);

    buffer.save("mqyas.png").unwrap();
    println!("image saved");
    Ok(())
}
