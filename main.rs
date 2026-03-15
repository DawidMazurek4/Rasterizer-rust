use std::fs::File;
use std::io::Write;
use std::cmp;

type Value3d = (i32, i32, i32);
type Value2d = (i32, i32);
type Pixel = (u8,u8,u8);
const SCREEN_SIZE:(i32, i32) = (500,500);

fn save_ppm(pixels: &Vec<Pixel>){ // saves vector of (u8,u8,u8) to a ppm file
    println!("save_ppm"); 
    let mut file = File::create("plik.ppm").unwrap();
    file.write_all(format!("P3\n{} {}\n255\n",SCREEN_SIZE.0,SCREEN_SIZE.1).as_bytes()).unwrap();

    for i in 0..pixels.len(){
        file.write_all(format!("{} {} {} ",pixels[i].0,pixels[i].1,pixels[i].2).as_bytes()).unwrap();
    }
}

fn put_pixels(pixels: &mut Vec<Pixel>,position: Vec<Value2d>,color: Pixel){ // puts color pixels in
                                                                            // pixel
    println!("put_pixels");
    for (x,y) in position{
        if x>= 0 && x < SCREEN_SIZE.0 && y >= 0 && y < SCREEN_SIZE.1{
            pixels[(x + (y * SCREEN_SIZE.0))as usize] = color;
        }
    }
}



fn orient(line: ((i32,i32),(i32,i32),(i32,i32))) -> i32{ // chcecks the orientation of a line and
                                                         // point
    (line.1.0 - line.0.0) * (line.2.1 - line.0.1) - (line.1.1 - line.0.1)*(line.2.0 - line.0.0)
}

fn point_in_triangle(triangle: &Vec<Value2d>, point: (i32,i32)) -> bool{ // chcecks if a point is
                                                                         // in a triangle
    let o1 = orient((triangle[0],triangle[1],point));
    let o2 = orient((triangle[1],triangle[2],point));
    let o3 = orient((triangle[2],triangle[0],point));

    return (o1>=0 && o2>=0 && o3>=0) || (o1<=0 && o2<=0 && o3<=0);
}


fn DrawTriangle(pixels: &mut Vec<Pixel>, triangle: &Vec<Value2d>,color: Pixel){ // drawing a
                                                                                // triangle
    // finding max and min values for minimum amount of iterations
    let MaxX = cmp::max(triangle[0].0, cmp::max(triangle[1].0, triangle[2].0))as i32;
    let MaxY = cmp::max(triangle[0].1, cmp::max(triangle[1].1, triangle[2].1))as i32;
    let MinX = cmp::min(triangle[0].0, cmp::min(triangle[1].0, triangle[2].0))as i32;
    let MinY = cmp::min(triangle[0].1, cmp::min(triangle[1].1, triangle[2].1))as i32;
    let mut triangle_to_draw:Vec<(Value2d)> = vec![];
    for i in MinY..MaxY{
        for j in MinX..MaxX{
            if point_in_triangle(triangle,(j,i)){
                triangle_to_draw.push((j,i));
            }
        }
    }
    put_pixels(pixels, triangle_to_draw,color);
    
}

fn main(){
    let mut pixels:Vec<Pixel> = Vec::with_capacity((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize);
    pixels.resize((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize, (0,0,0));
    DrawTriangle(&mut pixels, &vec![(250,0)as Value2d,(0,500)as Value2d,(500,500)as Value2d], (100,255,30)); 
    save_ppm(&pixels);
}
