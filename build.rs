use std::fs;
use std::path::Path;
use std::process::Command;

fn optimize_images(image_dir: &str, target_dir: &str) {
    fs::read_dir(Path::new(image_dir))
        .expect("Failed to read image directory")
        .for_each(|image| {
            let input_file = image.expect("Cannot read imgae");
            let output_file = Path::new(target_dir).join(input_file.file_name());
            Command::new("convert")
                .arg(input_file.path())
                .arg("-resize")
                .arg("x256")
                .arg(output_file.with_extension("webp"))
                .spawn()
                .expect("Failed to convert image");
        });
}

fn main() {
    let image_dir = "public/images";
    let target_dir = "target/site/images";
    println!("cargo:rerun-if-changed={image_dir}");
    optimize_images(image_dir, target_dir);
}
