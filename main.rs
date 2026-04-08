use std::fs::File;
use std::fs;
use std::io::Write;
use std::cmp;


type Pixel = (u8,u8,u8,f64);
const SCREEN_SIZE:(i32, i32) = (500,500);
const FOV:f64 = 90.0;

struct ObjFace {
    v: i32,
    vt: Option<i32>,
    vn: Option<i32>,
}

struct LightSource{
    position:(f64,f64,f64),
    color:(u8,u8,u8),
    power:i32,
}

struct Object3d{
    vertexs:Vec<(f64,f64,f64)>,
    triangles:Vec<(i32,i32,i32)>,
    position:(f64,f64,f64),
    rotation:(f64,f64,f64),
    scale:(f64,f64,f64),
    color:(u8,u8,u8),
}
fn calculate_3d_rotation_matrix(rotation:&(f64,f64,f64), position:(f64,f64,f64)) -> (f64,f64,f64){
    let mut pos_ret:(f64,f64,f64) = position;

    let cosa = rotation.0.to_radians().cos();
    let cosb = rotation.1.to_radians().cos();
    let cosy = rotation.2.to_radians().cos();

    let sina = rotation.0.to_radians().sin();
    let sinb = rotation.1.to_radians().sin();
    let siny = rotation.2.to_radians().sin();
    pos_ret.0 = position.0 * (cosb * cosy) + position.1 *((sina * sinb * cosy) - (cosa * siny)) + position.2 * ((cosa * sinb * cosy) + (sina * siny));
    pos_ret.1 = position.0 * (cosb * siny) + position.1 *((sina * sinb * siny) + (cosa * cosy)) + position.2 * ((cosa * sinb * siny) - (sina * cosy));
    pos_ret.2 = position.0 * -sinb + position.1 *(sina * cosb) + position.2 * (cosa * cosb);

    return pos_ret;
}

impl Object3d {
    fn normalize(&self) -> (Vec<(f64,f64)>, Vec<f64>) {
        let mut vertexs_normalized: Vec<(f64,f64)> = vec![];
        let mut z_buffer: Vec<f64> = vec![];
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

            if z <= 0.1 {
                continue;
            }

            let x = x3d / (z * (fov_rad / 2.0).tan());
            let y = y3d / (z * (fov_rad / 2.0).tan());

            let screen_x = (x * aspect_ratio + 1.0) * 0.5 * SCREEN_SIZE.0 as f64;
            let screen_y = (1.0 - (y + 1.0) * 0.5) * SCREEN_SIZE.1 as f64;

            vertexs_normalized.push((screen_x, screen_y));
            z_buffer.push(z);
        }

        (vertexs_normalized, z_buffer)
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

fn put_pixels(pixels: &mut Vec<Pixel>,position: Vec<(i32,i32)>,color: (u8,u8,u8),z_pos: f64){
    for (x,y) in position{
        if x>= 0 && x < SCREEN_SIZE.0 && y >= 0 && y < SCREEN_SIZE.1 && pixels[(x + (y * SCREEN_SIZE.0))as usize].3 < z_pos{
            pixels[(x + (y * SCREEN_SIZE.0))as usize] = (color.0,color.1,color.2,z_pos);
        }
    }
}



fn orient(line: ((f64,f64),(f64,f64),(f64,f64))) -> f64{ // chcecks the orientation of a line and
                                                         // point
    (line.1.0 - line.0.0) * ((line.2.1 - line.0.1)) - (line.1.1 - line.0.1)*(line.2.0 - line.0.0)
}

fn point_in_triangle(triangle: &Vec<(f64,f64)>, point: (f64,f64)) -> bool{ // chcecks if a point is
                                                                         // in a triangle
    let o1 = orient((triangle[0],triangle[1],point));
    let o2 = orient((triangle[1],triangle[2],point));
    let o3 = orient((triangle[2],triangle[0],point));

    return (o1>=0.0 && o2>=0.0 && o3>=0.0) || (o1<=0.0 && o2<=0.0 && o3<=0.0) || (o1==0.0 && o2==0.0 && o3==0.0);
}




fn draw_triangle(pixels: &mut Vec<Pixel>, triangle: &Vec<(f64,f64)>,color: (u8,u8,u8), z_pos: f64){ // drawing a
                                                                                // triangle
    // finding max and min values for minimum amount of iterations
    let mut max_x = cmp::max(triangle[0].0 as i32, cmp::max(triangle[1].0 as i32, triangle[2].0 as i32))as i32;
    let mut max_y = cmp::max(triangle[0].1 as i32, cmp::max(triangle[1].1 as i32, triangle[2].1 as i32))as i32;
    let mut min_x = cmp::min(triangle[0].0 as i32, cmp::min(triangle[1].0 as i32, triangle[2].0 as i32))as i32;
    let mut min_y = cmp::min(triangle[0].1 as i32, cmp::min(triangle[1].1 as i32, triangle[2].1 as i32))as i32;
    let mut triangle_to_draw:Vec<(i32,i32)> = vec![];
    
    if max_x < 0 || min_x > SCREEN_SIZE.0 || max_y < 0 || min_y > SCREEN_SIZE.1{
        return;
    }

    if max_x > SCREEN_SIZE.0{
        max_x = SCREEN_SIZE.0
    }
    if max_y > SCREEN_SIZE.1{
        max_y = SCREEN_SIZE.1;
    }
    if min_x < 0{
        min_x = 0;
    }
    if min_y < 0{
        min_y = 0;
    }
    
    for i in min_y..=max_y{
        for j in min_x..=max_x{
            if  point_in_triangle(triangle, (j as f64, i as f64)){
                triangle_to_draw.push((j,i));
            }
        }
    }

    put_pixels(pixels, triangle_to_draw,color, z_pos);
    
}
fn draw_object(pixels: &mut Vec<Pixel>, obj: &Object3d) {
    let (vertexs, z_vals) = obj.normalize();
    let mut triangle_to_draw: Vec<(f64, f64)> = vec![];

    for i in 0..obj.triangles.len() {
        triangle_to_draw.clear();

        let (i0, i1, i2) = (
            obj.triangles[i].0 as usize,
            obj.triangles[i].1 as usize,
            obj.triangles[i].2 as usize,
        );

        if i0 >= vertexs.len() || i1 >= vertexs.len() || i2 >= vertexs.len() {
            eprintln!("Skipping triangle {}: invalid vertex index", i);
            continue;
        }

        triangle_to_draw.push((vertexs[i0].0, vertexs[i0].1));
        triangle_to_draw.push((vertexs[i1].0, vertexs[i1].1));
        triangle_to_draw.push((vertexs[i2].0, vertexs[i2].1));

        let z_pos = (z_vals[i0] + z_vals[i1] + z_vals[i2]) / 3.0;

        draw_triangle(pixels, &triangle_to_draw, obj.color, z_pos);
    }
}

fn import_obj_file(
    filename: &str,
) -> (
    Vec<(f64, f64, f64)>,      // vertices
    Vec<(f64, f64)>,           // textures
    Vec<(f64, f64, f64)>,      // normals
    Vec<(i32, i32, i32)>,      // triangles
) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut vertices: Vec<(f64, f64, f64)> = vec![];
    let mut textures: Vec<(f64, f64)> = vec![];
    let mut normals: Vec<(f64, f64, f64)> = vec![];
    let mut faces: Vec<Vec<ObjFace>> = vec![];

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if let Some(kind) = parts.next() {
            match kind {
                "v" => {
                    let nums: Vec<f64> = parts.map(|x| x.parse::<f64>().unwrap()).collect();
                    if nums.len() == 3 {
                        vertices.push((nums[0], nums[1], nums[2]));
                    }
                }
                "vt" => {
                    let nums: Vec<f64> = parts.map(|x| x.parse::<f64>().unwrap()).collect();
                    if nums.len() >= 2 {
                        textures.push((nums[0], nums[1]));
                    }
                }
                "vn" => {
                    let nums: Vec<f64> = parts.map(|x| x.parse::<f64>().unwrap()).collect();
                    if nums.len() == 3 {
                        normals.push((nums[0], nums[1], nums[2]));
                    }
                }
                "f" => {
                    let face: Vec<ObjFace> = parts
                        .map(|part| {
                            let mut split = part.split('/');
                            let v = split.next().unwrap().parse::<i32>().unwrap() - 1;
                            let vt = split
                                .next()
                                .and_then(|s| if !s.is_empty() { s.parse::<i32>().ok().map(|n| n-1) } else { None });
                            let vn = split
                                .next()
                                .and_then(|s| if !s.is_empty() { s.parse::<i32>().ok().map(|n| n-1) } else { None });
                            ObjFace { v, vt, vn }
                        })
                        .collect();
                    faces.push(face);
                }
                _ => {}
            }
        }
    }

    // triangulate faces
    let mut triangles: Vec<(i32, i32, i32)> = vec![];
    for face in &faces {
        match face.len() {
            3 => triangles.push((face[0].v, face[1].v, face[2].v)),
            4 => {
                triangles.push((face[0].v, face[1].v, face[2].v));
                triangles.push((face[0].v, face[2].v, face[3].v));
            }
            n if n > 4 => {
                for i in 1..(n-1) {
                    triangles.push((face[0].v, face[i].v, face[i+1].v));
                }
            }
            _ => {}
        }
    }

    (vertices, textures, normals, triangles)
}
fn main(){
    let mut pixels:Vec<Pixel> = Vec::with_capacity((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize);
    pixels.resize((SCREEN_SIZE.0 * SCREEN_SIZE.1)as usize, (0,0,0,0.0));



    let mut cube:Object3d =  Object3d{
        vertexs: vec![],
        triangles: vec![],
        position: (0.4,-1.0,50.0),
        scale: (1.0,1.0,1.0),
        rotation: (0.0,0.0,0.0),
        color: (0,200,0),
        };
    
    let (v, vt, vn, t) = import_obj_file("monkey.obj");
    cube.vertexs = v;
    cube.triangles = t;
    draw_object(&mut pixels, &cube);
    save_ppm(&pixels, "plik.ppm".to_string())
    
}
