//Copyright 2024 Leonel Alejandro Aguilera Correia

//Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use image::GrayImage;

fn main() {
    let mut image_vec: Vec<u8> = Vec::new();
    for y in -64..64
    {
        for x in -64..64
        {
            image_vec.push(
                {
                    let distance = x*x + y*y;
                    if distance > 50*50
                    {
                        0
                    }
                    else
                    {
                        255
                    }
                }
            );
        }
    }
    println!("{}", image_vec.len());

    let output_image = GrayImage::from_raw(128, 128, image_vec);
    println!("{:?}", output_image);

    output_image.expect("Image buffer not big enough").save("Circle.png").expect("Could not save image");
}
