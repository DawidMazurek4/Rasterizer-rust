use std::fs::File;
use std::io::Write;
use std::cmp;

type Value3d = (i32, i32, i32);
type Value2d = (i32, i32);
type Pixel = (u8,u8,u8);
const SCREEN_SIZE:(i32, i32) = (500,500);
const FOV:f64 = 90.0;

struct Object3d{
    vertexs:Vec<(f64,f64,f64)>,
    triangles:Vec<(i32,i32,i32)>,
    position:(f64,f64,f64),
    scale:(f64,f64,f64),
    color:Pixel,
}

impl Object3d {
    fn normalize(&self) -> Vec<(i32,i32)> {
        let mut vertexs_normalized: Vec<(i32,i32)> = vec![];
        let fov_rad = (FOV as f64).to_radians();
        let aspect_ratio = SCREEN_SIZE.0 as f64 / SCREEN_SIZE.1 as f64;

        for &(x3d, y3d, z3d) in &self.vertexs {
            let z = z3d * self.scale.2 + self.position.2;

            if z <= 0.1 { // unikamy dzielenia przez 0
                vertexs_normalized.push((0,0));
                continue;
            }

            let x = (x3d * self.scale.0 + self.position.0) / (z * (fov_rad / 2.0).tan());
            let y = (y3d * self.scale.1 + self.position.1) / (z * (fov_rad / 2.0).tan());

            // normalizacja do ekranu
            let screen_x = ((x * aspect_ratio + 1.0) * 0.5 * SCREEN_SIZE.0 as f64).round() as i32;
            let screen_y = ((1.0 - (y + 1.0) * 0.5) * SCREEN_SIZE.1 as f64).round() as i32;

            vertexs_normalized.push((screen_x, screen_y));
        }

        vertexs_normalized
    }
}

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
    // println!("put_pixels {:?}", position);
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
    let start_x = (px as f32) - 0.5;
    let start_y = (py as f32) - 0.5;
    for sy in 0..s {
        for sx in 0..s {
            let sub_x = start_x + (sx as f32 * step) + (step / 2.0);
            let sub_y = start_y + (sy as f32 * step) + (step / 2.0);

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
fn DrawObject(pixels: &mut Vec<Pixel>,obj:&Object3d){
    let vertexs = obj.normalize();
    println!("vertex: {:?}",vertexs);
    let mut triangle_to_draw:Vec<(i32,i32)> = vec![];
    for i in 0..obj.triangles.len(){
        triangle_to_draw = vec![];
        triangle_to_draw.push((vertexs[obj.triangles[i].0 as usize].0 as i32, vertexs[obj.triangles[i].0 as usize].1 as i32));
        triangle_to_draw.push((vertexs[obj.triangles[i].1 as usize].0 as i32, vertexs[obj.triangles[i].1 as usize].1 as i32));
        triangle_to_draw.push((vertexs[obj.triangles[i].2 as usize].0 as i32, vertexs[obj.triangles[i].2 as usize].1 as i32));
        DrawTriangle(pixels, &triangle_to_draw, obj.color)
    }
}
fn main(){
    let mut pixels:Vec<Pixel> = Vec::with_capacity((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize);
    pixels.resize((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize, (0,0,0));



    let cube:Object3d =  Object3d{
        vertexs: vec![
            (-1.0, -1.0, 0.0), // 0
            ( 1.0, -1.0, 0.0), // 1
            ( 1.0,  1.0, 0.0), // 2
            (-1.0,  1.0, 0.0), // 3
            (-1.0, -1.0,  1.0), // 4
            ( 1.0, -1.0,  1.0), // 5
            ( 1.0,  1.0,  1.0), // 6
            (-1.0,  1.0,  1.0), // 7
            ],

            triangles: vec![
                // tył
                (0, 1, 2), (0, 2, 3),

                // przód
                (4, 5, 6), (4, 6, 7),

                // dół
                (0, 1, 5), (0, 5, 4),

                // góra
                (3, 2, 6), (3, 6, 7),

                // lewo
                (0, 3, 7), (0, 7, 4),

                // prawo
                (1, 2, 6), (1, 6, 5),
            ],
            position: (2000.0,2000.0,20000.0),
            scale: (10000.0,10000.0,10000.0),
            color: (255,0,0,),
        };
    DrawObject(&mut pixels, &cube);
    // DrawTriangle(&mut pixels, &vec![(8,1)as Value2d,(1,8)as Value2d,(10,9)as Value2d], (200,0,0));
    save_ppm(&pixels);
}
