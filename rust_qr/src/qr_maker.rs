// qr_maker

use qrcode::QrCode;
use qrcode::render::svg;
use image::Luma;
use image::png::PngEncoder;
use base64;
use image::EncodableLayout;
use image::ColorType;


pub fn generate_qr(url: &str) -> String {
    // generate a QR code from the url
    let code = QrCode::new(url).unwrap();
    // render the QR code into an image
    let image = code.render()
        // .min_dimensions(200, 200)
        .dark_color(svg::Color("#20123a"))
        .light_color(svg::Color("#fff"))
        .build();

    let image_with_class = image.replace("<svg ", "<svg class=\"qr_img\" ");

    return image_with_class;
}

// generate a QR code from the url as a png and encode it as base64
pub fn generate_qr_png(url: &str) -> String {
    // Encode some data into bits.
    let code = QrCode::new(url).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // encode the image as a base64 string
    let mut png_data = Vec::new();
    let encoder = PngEncoder::new(&mut png_data);
    encoder.encode(
        image.as_bytes(),
        image.width(),
        image.height(),
        ColorType::L8
    ).expect("Error encoding PNG");

    let png_base64 = base64::encode(png_data);

    return png_base64;


}
