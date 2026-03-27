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
    rotation:(f64,f64,f64),
    scale:(f64,f64,f64),
    color:Pixel,
}
fn calculate_3d_rotation_matrix(rotation:&(f64,f64,f64), position:(f64,f64,f64)) -> (f64,f64,f64){
    let mut pos_ret:(f64,f64,f64) = position;

    let cosa = rotation.0.to_radians().cos();
    let cosb = rotation.1.to_radians().cos();
    let cosy = rotation.2.to_radians().cos();

    let sina = rotation.0.to_radians().sin();
    let sinb = rotation.1.to_radians().sin();
    let siny = rotation.2.to_radians().sin();
    pos_ret.0 = (position.0 * (cosb * cosy) + position.1 *((sina * sinb * cosy) - (cosa * siny)) + position.2 * ((cosa * sinb * cosy) + (sina * siny)));
    pos_ret.1 = (position.0 * (cosb * siny) + position.1 *((sina * sinb * siny) + (cosa * cosy)) + position.2 * ((cosa * sinb * siny) - (sina * cosy)));
    pos_ret.2 = (position.0 * (-sinb) + position.1 *(sina * cosb) + position.2 * (cosa * cosb));

    return pos_ret;
}

impl Object3d {
    fn normalize(&self) -> Vec<(f64,f64)> {
        let mut vertexs_normalized: Vec<(f64,f64)> = vec![];
        let fov_rad = (FOV as f64).to_radians();
        let aspect_ratio = SCREEN_SIZE.0 as f64 / SCREEN_SIZE.1 as f64;

        for &(mut x3d, mut y3d, mut z3d) in &self.vertexs {
            
            
            (x3d,y3d,z3d) = calculate_3d_rotation_matrix(&self.rotation, (x3d,y3d,z3d));

            x3d += self.position.0;
            y3d += self.position.1;
            x3d *= self.scale.0;
            y3d *= self.scale.1;
            z3d *= self.scale.2;

            
            let z = z3d + self.position.2;

            if z <= 0.1 { // unikamy dzielenia przez 0
                vertexs_normalized.push((0.0,0.0));
                continue;
            }

            let x = (x3d) / (z * (fov_rad / 2.0).tan());
            let y = (y3d) / (z * (fov_rad / 2.0).tan());

            // normalizacja do ekranu
            let screen_x = ((x * aspect_ratio + 1.0) * 0.5 * SCREEN_SIZE.0 as f64);
            let screen_y = ((1.0 - (y + 1.0) * 0.5) * SCREEN_SIZE.1 as f64);

            vertexs_normalized.push((screen_x, screen_y));
        }

        vertexs_normalized
    }
}

fn save_ppm(pixels: &Vec<Pixel>, name:String){ // saves vector of (u8,u8,u8) to a ppm file
    println!("save_ppm"); 
    let mut file = File::create(name).unwrap();
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



fn orient(line: ((f64,f64),(f64,f64),(f64,f64))) -> f64{ // chcecks the orientation of a line and
                                                         // point
    (line.1.0 - line.0.0)as f64 * (line.2.1 - line.0.1 as f64) - (line.1.1 - line.0.1)as f64*(line.2.0 - line.0.0 as f64)
}

fn point_in_triangle(triangle: &Vec<(f64,f64)>, point: (f64,f64)) -> bool{ // chcecks if a point is
                                                                         // in a triangle
    let o1 = orient((triangle[0],triangle[1],point));
    let o2 = orient((triangle[1],triangle[2],point));
    let o3 = orient((triangle[2],triangle[0],point));

    return (o1>=0.0 && o2>=0.0 && o3>=0.0) || (o1<=0.0 && o2<=0.0 && o3<=0.0) || (o1==0.0 && o2==0.0 && o3==0.0);
}




fn DrawTriangle(pixels: &mut Vec<Pixel>, triangle: &Vec<(f64,f64)>,color: Pixel){ // drawing a
                                                                                // triangle
    // finding max and min values for minimum amount of iterations
    let mut MaxX = cmp::max(triangle[0].0 as i32, cmp::max(triangle[1].0 as i32, triangle[2].0 as i32))as i32;
    let mut MaxY = cmp::max(triangle[0].1 as i32, cmp::max(triangle[1].1 as i32, triangle[2].1 as i32))as i32;
    let mut MinX = cmp::min(triangle[0].0 as i32, cmp::min(triangle[1].0 as i32, triangle[2].0 as i32))as i32;
    let mut MinY = cmp::min(triangle[0].1 as i32, cmp::min(triangle[1].1 as i32, triangle[2].1 as i32))as i32;
    let mut triangle_to_draw:Vec<(i32,i32,f32)> = vec![];
    
    if MaxX < 0 || MinX > SCREEN_SIZE.0 || MaxY < 0 || MinY > SCREEN_SIZE.1{
        return;
    }

    if MaxX > SCREEN_SIZE.0{
        MaxX = SCREEN_SIZE.0
    }
    if MaxY > SCREEN_SIZE.1{
        MaxY = SCREEN_SIZE.1;
    }
    if MinX < 0{
        MinX = 0;
    }
    if MinY < 0{
        MinY = 0;
    }
    
    for i in MinY..MaxY{
        for j in MinX..MaxX{
            if  point_in_triangle(triangle, (j as f64, i as f64)){
                triangle_to_draw.push((j,i,1.0));
            }
        }
    }

    put_pixels(pixels, triangle_to_draw,color);
    
}
fn DrawObject(pixels: &mut Vec<Pixel>,obj:&Object3d){
    let vertexs = obj.normalize();
    println!("vertex: {:?}",vertexs);
    let mut triangle_to_draw:Vec<(f64,f64)> = vec![];
    for i in 0..obj.triangles.len(){
        triangle_to_draw = vec![];
        triangle_to_draw.push((vertexs[obj.triangles[i].0 as usize].0, vertexs[obj.triangles[i].0 as usize].1));
        triangle_to_draw.push((vertexs[obj.triangles[i].1 as usize].0, vertexs[obj.triangles[i].1 as usize].1));
        triangle_to_draw.push((vertexs[obj.triangles[i].2 as usize].0, vertexs[obj.triangles[i].2 as usize].1));
        DrawTriangle(pixels, &triangle_to_draw, obj.color)
    }
}
fn main(){
    let mut pixels:Vec<Pixel> = Vec::with_capacity((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize);
    pixels.resize((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize, (0,0,0));



    let mut cube:Object3d =  Object3d{
        vertexs: vec![
            (-1.0, -1.0, -1.0), // 0
            ( 1.0, -1.0, -1.0), // 1
            ( 1.0,  1.0, -1.0), // 2
            (-1.0,  1.0, -1.0), // 3
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
            position: (0.8,1.0,22.0),
            scale: (5.0,5.0,5.0),
            rotation: (0.0,0.0,0.0),
            color: (255,0,0),
        };
    DrawObject(&mut pixels, &cube);
    save_ppm(&pixels, "plik.ppm".to_string())
    
}
