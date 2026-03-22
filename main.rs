use std::fs::File;
use std::io::Write;
use std::cmp;

type Value3d = (i32, i32, i32);
type Value2d = (i32, i32);
type Pixel = (u8,u8,u8);
const SCREEN_SIZE:(i32, i32) = (12,12);

fn save_ppm(pixels: &Vec<Pixel>){ // saves vector of (u8,u8,u8) to a ppm file
    println!("save_ppm"); 
    let mut file = File::create("plik.ppm").unwrap();
    file.write_all(format!("P3\n{} {}\n255\n",SCREEN_SIZE.0,SCREEN_SIZE.1).as_bytes()).unwrap();

    for i in 0..pixels.len(){
        file.write_all(format!("{} {} {} ",pixels[i].0,pixels[i].1,pixels[i].2).as_bytes()).unwrap();
    }
}

fn put_pixels(pixels: &mut Vec<Pixel>,position: Vec<(i32,i32,f32)>,color: Pixel){ // puts color pixels in
                                                                            // pixel
    println!("put_pixels {:?}", position);
    for (x,y,i) in position{
        if x>= 0 && x < SCREEN_SIZE.0 && y >= 0 && y < SCREEN_SIZE.1{
            pixels[(x + (y * SCREEN_SIZE.0))as usize] = ((color.0 as f32* i)as u8,(color.1 as f32 * i)as u8,(color.2 as f32 * i)as u8);
        }
    }
}



fn orient(line: ((i32,i32),(i32,i32),(f64,f64))) -> f64{ // chcecks the orientation of a line and
                                                         // point
    (line.1.0 - line.0.0)as f64 * (line.2.1 - line.0.1 as f64) - (line.1.1 - line.0.1)as f64*(line.2.0 - line.0.0 as f64)
}

fn point_in_triangle(triangle: &Vec<Value2d>, point: (f64,f64)) -> bool{ // chcecks if a point is
                                                                         // in a triangle
    let o1 = orient((triangle[0],triangle[1],point));
    let o2 = orient((triangle[1],triangle[2],point));
    let o3 = orient((triangle[2],triangle[0],point));

    return (o1>=0.0 && o2>=0.0 && o3>=0.0) || (o1<=0.0 && o2<=0.0 && o3<=0.0);
}

fn pixel_division(triangle: &Vec<Value2d>, px: i32, py: i32, s: i16) -> f32 {
    let mut color_intensity: f32 = 0.0;
    let total_samples = (s * s) as f32;
    let step = 1.0 / s as f32;
    let start = (px as f32) - 0.5;
    for sy in 0..s {
        for sx in 0..s {
            let sub_x = start + (sx as f32 * step) + (step / 2.0);
            let sub_y = start + (sy as f32 * step) + (step / 2.0);

            if point_in_triangle(triangle, (sub_x as f64, sub_y as f64)) {
                color_intensity += 1.0 / total_samples;
            }
        }
    }
    
    color_intensity
}


fn DrawTriangle(pixels: &mut Vec<Pixel>, triangle: &Vec<Value2d>,color: Pixel){ // drawing a
                                                                                // triangle
    // finding max and min values for minimum amount of iterations
    let MaxX = cmp::max(triangle[0].0, cmp::max(triangle[1].0, triangle[2].0))as i32;
    let MaxY = cmp::max(triangle[0].1, cmp::max(triangle[1].1, triangle[2].1))as i32;
    let MinX = cmp::min(triangle[0].0, cmp::min(triangle[1].0, triangle[2].0))as i32;
    let MinY = cmp::min(triangle[0].1, cmp::min(triangle[1].1, triangle[2].1))as i32;
    let mut triangle_to_draw:Vec<(i32,i32,f32)> = vec![];
    for i in MinY..MaxY{
        for j in MinX..MaxX{
            let p_d:f32 = pixel_division(triangle, j, i, 2);
            if  p_d != 0 as f32{
                triangle_to_draw.push((j,i,p_d));
            }
        }
    }
    put_pixels(pixels, triangle_to_draw,color);
    
}

fn main(){
    let mut pixels:Vec<Pixel> = Vec::with_capacity((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize);
    pixels.resize((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize, (0,0,0));
    DrawTriangle(&mut pixels, &vec![(8,1)as Value2d,(1,8)as Value2d,(10,9)as Value2d], (200,0,0));
    save_ppm(&pixels);
}
