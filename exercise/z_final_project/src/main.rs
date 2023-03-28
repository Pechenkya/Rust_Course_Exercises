// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png (Now everything works as in the end of the task)
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use image::DynamicImage;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }

    let first_arg = args.remove(0);

    if first_arg.as_str() == "fractal"
    {
        if args.len() != 1 {
            print_usage_and_exit();
        }
        let to = args.remove(0);
        fractal(to);
    }
    else if first_arg.as_str() == "generate" 
    {
        if args.len() != 1 {
            print_usage_and_exit();
        }
        let to = args.remove(0);
        generate(to);
    }
    else 
    {
        let from = first_arg;
        let to = args.remove(0);
        
        let mut img = image::open(from).expect("Failed to open from.");
        while !args.is_empty() {
            let subcommand = args.remove(0);
            match subcommand.as_str() {
                // EXAMPLE FOR CONVERSION OPERATIONS
                "blur" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    let blur_rate = args.remove(0).parse::<f32>().expect("BLUR_RATE parsing error");
    
                    // Improve the blur implementation -- see the blur() function below DONE
                    blur(&mut img, blur_rate);
                }
    
                // **OPTION**
                // Brighten -- see the brighten() function below
                "brighten" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    let brightness = args.remove(0).parse::<i32>().expect("BRIGHTNESS parsing error");
    
                    brighten(&mut img, brightness);
                }
    
                // **OPTION**
                // Crop -- see the crop() function below
                "crop" => {
                    if args.len() < 4 {
                        print_usage_and_exit();
                    }
                    let x = args.remove(0).parse::<u32>().expect("X parsing error");
                    let y = args.remove(0).parse::<u32>().expect("Y parsing error");
                    let width = args.remove(0).parse::<u32>().expect("WIDTH parsing error");
                    let height = args.remove(0).parse::<u32>().expect("HEIGHT parsing error");
    
                    crop(&mut img, x, y, width, height);
                }
    
                // **OPTION**
                // Rotate -- see the rotate() function below
                "rotate" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    let rotation_amount = args.remove(0).parse::<u32>().expect("ROTATION_RATE parsing error");
    
                    rotate(&mut img, rotation_amount);
                }
    
                // **OPTION**
                // Invert -- see the invert() function below
                "invert" => invert(&mut img),
    
                // **OPTION**
                // Grayscale -- see the grayscale() function below
                "grayscale" => grayscale(&mut img),
    
                // For everything else...
                _ => {
                    print_usage_and_exit();
                }
            }
        }

        img.save(to).expect("Failed to save to.");
    }  
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur from to BLUR_RATE");
    println!("brighten from to BRIGHTNESS");
    println!("crop from to X Y WIDTH HEIGHT");
    println!("rotate from to ROTATION_AMOUNT (in degrees)");
    println!("invert from to");
    println!("grayscale from to");
    println!("fractal to");
    println!("generate to");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(img: &mut DynamicImage, blur_rate: f32) {
    // Here's how you open an existing image file
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    *img = img.blur(blur_rate);
    // Here's how you save an image to a file.
}

fn brighten(img: &mut DynamicImage, brightness: i32) {
    // See blur() for an example of how to open / save an image.
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    *img = img.brighten(brightness);
    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
}

fn crop(img: &mut DynamicImage, x: u32, y: u32, width: u32, height:u32) {
    // See blur() for an example of how to open an image.
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    img.crop(x, y, width, height);
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
}

fn rotate(img: &mut DynamicImage, rotation_amount: u32) {
    // See blur() for an example of how to open an image.
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    match rotation_amount % 360
    {
        270 => *img = img.rotate270(), 
        180 => *img = img.rotate180(),
        90 => *img = img.rotate90(),
        _ => {}
    }

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(img: &mut DynamicImage,) {
    // See blur() for an example of how to open an image.
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    // See blur() for an example of how to save the image.
}

fn grayscale(img: &mut DynamicImage,) {
    // See blur() for an example of how to open an image.
    // .grayscale() takes no arguments. It returns a new image.
    *img = img.grayscale();
    // See blur() for an example of how to save the image.
}

fn generate(to: String) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;
    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let blue = (0.3 * x as f32) as u8;
        let green = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut red = 0;
        while red < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            red += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(to).unwrap();
    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(to: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(to).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run from.png to.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read from.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to to.png
//
// Good luck!
