# RapidImage
Find the size of an image  
inspired by https://github.com/sdsykes/fastimage

## Installation
Add this to your `Cargo.toml`:

```
[dependencies]
rapidimage = "0.1.0"
```

## Usage
```
extern crate rapidimage;

let image_type = rapidimage::reader::mine_type("/some/local/file.gif");
println!("{:?}", image_type);
=> GIF
```
