use std::fs;
use std::path::Path;
use std::io::BufReader;
use exif;
use image;

fn gcd_custom(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn print_exif_metadata(path: &Path) {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let mut bufreader = BufReader::new(file);     // ownership will be moved
    let exifreader = exif::Reader::new();
    let exif_data = match exifreader.read_from_container(&mut bufreader) {
        Ok(data) => data,
        Err(_) => {
            println!("No EXIF metadata found.");
            return;
        }
    };
    for field in exif_data.fields() {
        match field.tag.number() {
            271 => println!("Camera Make: {:?}", field.value),
            272 => println!("Camera Model: {:?}", field.value),
            306 => println!("Date Taken: {:?}", field.value),
            34855 => println!("ISO: {:?}", field.value),
            33434 => println!("Exposure Time: {:?}", field.value),
            33437 => println!("Aperture: {:?}", field.value),
            37386 => println!("Focal length: {:?}", field.value),
            42036 => println!("Lens: {:?}", field.value),
            _ => {}
        }
    }
}

fn print_basic_metadata(file_size: u64, format: &str, width: u32, height: u32,) {
    let gcd = gcd_custom(width as u64, height as u64);
    let aspect_width = width as u64 / gcd;
    let aspect_height = height as u64 / gcd;
    println!("File Size: {} bytes", file_size);
    println!("Format: {}", format);
    println!("Width: {} px", width);
    println!("Height: {} px", height);
    println!("Aspect Ratio: {}:{}", aspect_width, aspect_height);
}

fn main() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read input");
    let cleaned = name.trim();
    let path = Path::new(cleaned);
    let metadata = fs::metadata(path).expect("Metadata failed");
    let img = image::open(path).expect("Not a valid image");
    let width = img.width();
    let height = img.height();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("unknown");           // Convert file extension to text; if no extension exists, use "unknown". Borrowed string is used.
    println!("Basic Metadata:");
    print_basic_metadata(metadata.len(), ext,width, height,);
    println!("\n");
    println!("EXIF Metadata:");
    print_exif_metadata(path);
}