use clap::Parser;
use color_eyre::{eyre::Context, Result, Section};
use image::{
    imageops::{dither, ColorMap},
    io::Reader as ImageReader,
    Rgb,
};
use num_bigint::BigUint;
use num_traits::Num;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image file
    file: String,

    /// The width of the prime number image generated.
    #[arg(long, default_value_t = 30)]
    width: u32,

    /// The height of the prime number image generated.
    #[arg(long, default_value_t = 60)]
    height: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let Args {
        file,
        width,
        height,
    } = Args::parse();

    let mut img = ImageReader::open(&file)
        .wrap_err(format!("Unable to read image: '{}'", &file))
        .suggestion("try using a file that exists next time")?
        .decode()
        .wrap_err(format!("Unable to decode image: '{}'", &file))
        .suggestion(format!("are you sure '{}' is a valid image file", file))?
        .thumbnail(width, height)
        .into_rgb8();

    dither(&mut img, &MyColorMap);

    let (width, height) = (img.width(), img.height());

    let digits = img
        .pixels()
        .map(|x| (x.0[0] % 10))
        .map(|x| x.to_string())
        .collect::<String>();
    let img_num = BigUint::from_str_radix(&digits, 10).unwrap();

    println!("I have converted the image into a number");
    print_big_num(&img_num, width, height);

    println!("I am now calculating the prime number version, this will take a long time");
    let img_num = next_prime(&img_num);
    print_big_num(&img_num, width, height);

    Ok(())
}

fn next_prime(n: &BigUint) -> BigUint {
    let n = num_bigint_dig::prime::next_prime(&num_bigint_dig::BigUint::from_bytes_le(
        &n.to_bytes_le(),
    ));
    BigUint::from_bytes_le(&n.to_bytes_le())
}

fn _is_prime(n: &Vec<u32>) -> bool {
    num_bigint_dig::prime::probably_prime(
        &num_bigint_dig::BigUint::from_bytes_le(&BigUint::new(n.to_owned()).to_bytes_le()),
        2,
    )
}

fn print_big_num(digits: &BigUint, width: u32, height: u32) {
    let digits = digits.to_string();

    let mut padded = "0".repeat((width * height) as usize - digits.len());
    padded.push_str(&digits);
    let padded = padded.chars().collect::<Vec<_>>();

    for y in 0..width {
        for x in 0..height {
            match padded.get((y * width + x) as usize) {
                Some(x) => print!("{x}"),
                None => print!(" "),
            }
        }
        println!();
    }
    println!();

    println!("num = {digits}")
}

struct MyColorMap;

impl ColorMap for MyColorMap {
    type Color = Rgb<u8>;

    fn index_of(&self, _: &Self::Color) -> usize {
        unimplemented!()
    }

    fn map_color(&self, color: &mut Self::Color) {
        let [r, g, b] = color.0;
        let grayscale = ((r as u32 + g as u32 + b as u32) / 3) as u8;
        color.0 = [grayscale; 3];
    }
}
