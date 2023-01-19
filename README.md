# RapidImage
Find the size of an image
inspired by https://github.com/sdsykes/fastimage

## Installation
Add this to your `Cargo.toml`:

```
[dependencies]
rand = "0.8.5"
```

## Usage
```
extern crate rapidimage;

let image_type = rapidimage::reader::mine_type("/some/local/file.gif");
println!("{:?}", image_type);
=> GIF
```
