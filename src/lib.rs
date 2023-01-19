pub mod reader{
  use std::fs::File;
  use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum ImageType {
      GIF,
      JPEG,
      PNG
  }

  pub fn mine_type(image_path: &str) -> ImageType {
    let mut file = File::open(image_path).unwrap();
    let mut buf = [0; 8];
    file.read_exact(&mut buf).unwrap();
    if &buf[..4] == b"\xff\xd8\xff\xe0" {
        return ImageType::JPEG;
    } else if &buf[..8] == b"\x89PNG\r\n\x1a\n" {
        return ImageType::PNG
    } else if &buf[..6] == b"GIF87a" || &buf[..6] == b"GIF89a" {
        return ImageType::GIF;
    } else {
        panic!("This image type is unknown");
    }
  }
}
