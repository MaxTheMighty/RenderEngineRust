use std::cmp::PartialEq;
use std::ops::{Neg};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cgmath::{InnerSpace, Vector3};
use RenderEngine::color::Color;
use RenderEngine::object::{Normal, Object};
use RenderEngine::sphere::Sphere;
use crate::LightType::{Ambient, Directional, Point};


#[derive(PartialEq)]
enum LightType{
    Ambient,
    Directional,
    Point
}

#[derive(PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
    In,
    Out
}

fn rotate_direction(direction: &Direction ) -> Direction {
    match direction{
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::In,
        Direction::In => Direction::Out,
        Direction::Out => Direction::Up,
    }

}


struct Light {
    kind: LightType,
    pos_or_direction: Vector3<f64>,
    intensity: f64
}

const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = 1000;

const CANVAS_WIDTH_I: i32 = CANVAS_WIDTH as i32;
const CANVAS_HEIGHT_I: i32 = CANVAS_HEIGHT as i32;

const CANVAS_WIDTH_I_HALF: i32 = CANVAS_WIDTH_I / 2;
const CANVAS_HEIGHT_I_HALF: i32 = CANVAS_HEIGHT_I / 2;

const VIEWPORT_WIDTH: i32 = 1;
const VIEWPORT_HEIGHT: i32 = 1;



const CAMERA_POSITION: Vector3<f64> = Vector3{x:0.0,y:0.0,z:0.0};




fn main() -> Result<(), Error>  {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<Color> = Vec::new();
    let viewport_distance: f64 = 1.0;
    my_buffer.resize((((CANVAS_HEIGHT + 1) * (CANVAS_WIDTH + 1))) as usize, RenderEngine::color::BLACK);
    let mut direction: Direction = Direction::In;
    let mut mouse_pos: (i32, i32) = (0,0);
    let mut light_z: f64 = 4.0;
    let mut lights: Vec<Light> = vec![
        Light{kind:LightType::Ambient,pos_or_direction:Vector3{x:0.0,y:0.0,z:0.0},intensity:0.10},
        Light{kind:LightType::Point,pos_or_direction:Vector3{x:-4.0,y:-4.0,z:4.0},intensity:0.20},
        Light{kind:LightType::Directional,pos_or_direction:Vector3{x:-4.0,y:-5.0,z:1.0},intensity:0.20},
        Light{kind:LightType::Point,pos_or_direction:Vector3{x:0.0,y:10.0,z:5.5},intensity:0.50},
    ];
    let mut spheres: Vec<Sphere> = vec![
        Sphere{r:1.0,origin:Vector3{x:0.0,y:-1.0,z:7.0},color:RenderEngine::color::RED,specular_reflection:1.0},
        Sphere{r:1.0,origin:Vector3{x:2.0,y:0.0,z:6.0},color:RenderEngine::color::BLUE,specular_reflection:2.0},
        Sphere{r:1.0,origin:Vector3{x:-2.0,y:0.0,z:8.0},color:RenderEngine::color::GREEN,specular_reflection:-1.0},
    ];
    let window = {
        let size = LogicalSize::new(CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        let window_pos = LogicalPosition::new(0.0,0.0);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_position(window_pos)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(CANVAS_WIDTH, CANVAS_HEIGHT, surface_texture)?
    };

    // pixels.frame_mut()[0] = 100;
    event_loop.run(move |event, _, control_flow| unsafe {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            //do some drawing here
            render_to_my_buffer(&mut my_buffer, &mut spheres, &lights, viewport_distance);

            copy_to_pixels(&my_buffer,pixels.frame_mut());
            // update_light_position(&mut lights, &mut direction);
            update_single_light_position(&mut lights[1], &mut direction);
            match input.mouse(){
                Some(mouse) => {
                    mouse_pos = convert_from_window_to_screen((mouse.0/2.0) as i32 ,(mouse.1/2.0) as i32);
                    spheres[0].origin = convert_from_canvas_to_viewport(mouse_pos.0, mouse_pos.1, viewport_distance);
                    spheres[0].origin.z = light_z;
                }

                None => {}
            }


            light_z += input.scroll_diff() as f64;

        }

        if input.update(&event){
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Up) || input.key_pressed(VirtualKeyCode::W) {
                spheres[0].r += 0.5;
            }
            if input.key_pressed(VirtualKeyCode::Down) || input.key_pressed(VirtualKeyCode::S) {
                spheres[0].r -= 0.5;
            }
            if input.key_pressed(VirtualKeyCode::Left) || input.key_pressed(VirtualKeyCode::A) {
                spheres[0].specular_reflection -= 1.0;
                println!("Specular reflection: {}", spheres[0].specular_reflection);
            }
            if input.key_pressed(VirtualKeyCode::Right) || input.key_pressed(VirtualKeyCode::D) {
                spheres[0].specular_reflection += 1.0;
                println!("Specular reflection: {}", spheres[0].specular_reflection);
            }

            // if input.key_pressed(VirtualKeyCode::Up){
            //     viewport_distance+=1.0;
            // }




        }


        pixels.render().expect("TODO: panic message");


        window.request_redraw();


    });
}

fn update_single_light_position(light: &mut Light, direction: &mut Direction) {
    let bound: f64 = 2.5;
    let step: f64 = 0.5;
    match direction{
        Direction::Up => {
            light.pos_or_direction.y-=step;
            if(light.pos_or_direction.y < -bound){
                *direction = rotate_direction(&direction); //is this allowed?
            }
        }
        Direction::Down => {
            light.pos_or_direction.y+=step;
            if(light.pos_or_direction.y > bound){
                *direction = rotate_direction(&direction); //is this allowed?
            }
        }
        Direction::Left => {
            light.pos_or_direction.x-=step;
            if(light.pos_or_direction.x < -bound){
                *direction = rotate_direction(&direction); //is this allowed?
            }
        }
        Direction::Right => {
            light.pos_or_direction.x+=step;
            if(light.pos_or_direction.x > bound){
                *direction = rotate_direction(&direction); //is this allowed?
            }
        }

        Direction::In => {
            light.pos_or_direction.z+=step;
            if(light.pos_or_direction.z > bound){
                *direction = rotate_direction(&direction);
            }
        }

        Direction::Out => {
            light.pos_or_direction.z-=step;
            if(light.pos_or_direction.z < -bound){
                *direction = rotate_direction(&direction);
            }
        }
        _ => {}
    }

}

fn update_light_position(lights: &mut Vec<Light>, direction: &mut Direction) {
    let bound: f64 = 2.5;
    let step: f64 = 0.5;
    for light in lights.iter_mut(){
        if(light.kind == Ambient){
            continue;
        }
        if(light.kind == Directional){
            continue;
        }
        match direction{
            Direction::Up => {
                light.pos_or_direction.y-=step;
                if(light.pos_or_direction.y < -bound){
                    *direction = rotate_direction(&direction); //is this allowed?
                }
            }
            Direction::Down => {
                light.pos_or_direction.y+=step;
                if(light.pos_or_direction.y > bound){
                    *direction = rotate_direction(&direction); //is this allowed?
                }
            }
            Direction::Left => {
                light.pos_or_direction.x-=step;
                if(light.pos_or_direction.x < -bound){
                    *direction = rotate_direction(&direction); //is this allowed?
                }
            }
            Direction::Right => {
                light.pos_or_direction.x+=step;
                if(light.pos_or_direction.x > bound){
                    *direction = rotate_direction(&direction); //is this allowed?
                }
            }

            Direction::In => {
                light.pos_or_direction.z+=step;
                if(light.pos_or_direction.z > bound){
                    *direction = rotate_direction(&direction);
                }
            }

            Direction::Out => {
                light.pos_or_direction.z-=step;
                if(light.pos_or_direction.z < -bound){
                    *direction = rotate_direction(&direction);
                }
            }
            _ => {}
        }
    }

}

fn render_to_my_buffer(my_buffer: &mut Vec<Color>, spheres: &mut Vec<Sphere>, lights: &Vec<Light>, viewport_distance: f64) {
    // println!("do_drawing called");
    for x in -CANVAS_WIDTH_I/2..CANVAS_WIDTH_I/2 {
        for y in -CANVAS_HEIGHT_I/2..CANVAS_HEIGHT_I/2 {
            let d = convert_from_canvas_to_viewport(x,y, viewport_distance);
            let color = trace_ray(CAMERA_POSITION, d, lights, viewport_distance, f64::INFINITY, spheres);  //trace ray from (d.x,d.y,d.z)
            draw_point(x,y,color,my_buffer);
            //draw color on canvas at points x and y
        }
    }
}

fn copy_to_pixels(my_buffer: &Vec<Color>, pixels_buffer: &mut [u8]){
    for (index, pixel) in pixels_buffer.chunks_exact_mut(4).enumerate() {
        pixel[0] = (my_buffer[index].r);
        pixel[1] = (my_buffer[index].g);
        pixel[2] = (my_buffer[index].b);
        pixel[3] = (my_buffer[index].a);
    }
}

fn trace_ray(ray_origin: Vector3<f64>, ray_direction: Vector3<f64>, lights: &Vec<Light>, point_min: f64, point_max: f64, spheres: &mut Vec<Sphere>) -> Color{
    //find out where the ray is going
    //and if it interescts with a sphere,
    //return the points along the ray where it interesects
    //because the straight line is going through the sphere, it interesects it twice
    //front and back
    //we just want front
    let mut closest_solution: f64 = f64::INFINITY;
    let mut closest_sphere: Option<Sphere> = None;
    let mut closest_color = RenderEngine::color::WHITE;
    let mut intersections: (f64,f64);
    let mut point: Vector3<f64>;
    let mut sphere_normal: Vector3<f64>;
    let mut intensity: f64;
    for sphere in spheres{
        // intersections = ray_sphere_intersection(&sphere, ray_origin, ray_direction);
        intersections = sphere.ray_intersections(&ray_origin, &ray_direction);
        if(intersections.0 < closest_solution && intersections.0 > point_min && intersections.0 < point_max){
            closest_solution = intersections.0;
            closest_sphere = Some(*sphere);
            closest_color = sphere.color;
        }
        if(intersections.1 < closest_solution && intersections.1 > point_min && intersections.1 < point_max){
            closest_solution = intersections.1;
            closest_sphere = Some(*sphere);
            closest_color = sphere.color;
        }
    }
    match closest_sphere{
        None => RenderEngine::color::WHITE,
        Some(sphere) => {
            point = ray_origin + (closest_solution * ray_direction);
            sphere_normal = sphere.normal(&point);
            intensity = intensity_at_point(point, sphere_normal, lights, &sphere, &ray_direction);
            adjust_color(&mut closest_color,intensity);
            return closest_color;
        }
    }
}


fn intensity_at_point(point: Vector3<f64>,  normal: Vector3<f64>, lights: &Vec<Light>, sphere: &Sphere, ray_direction: &Vector3<f64>) -> f64 {
    let mut intensity: f64 = 0.0;
    let mut top_factor: f64;
    let mut light_vector: Vector3<f64>;
    for light in lights{
        match light.kind {
            LightType::Directional =>{
                light_vector = light.pos_or_direction;
            }
            LightType::Point => {
                light_vector = light.pos_or_direction - point; // do something here
            }
            LightType::Ambient=> {
                intensity += light.intensity;
                continue;
            }
        }
        top_factor = normal.dot(light_vector);
        if(top_factor < 0.0){
            continue;
        }
        intensity = intensity + ((light.intensity * top_factor) / (normal.magnitude() * light_vector.magnitude())); //get light from defined lights
        intensity = intensity + get_reflection_for_object(&sphere, &normal, &ray_direction.neg(), &light_vector);
    }
    return intensity;
}

fn get_reflection_for_object(sphere: &Sphere, normal: &Vector3<f64>, camera_point_vector: &Vector3<f64>, light_vector: &Vector3<f64>) -> f64 {
    let mut out: f64 = 0.0;
    if(sphere.specular_reflection <= 1.0){
        return out;
    }

    let reflection_vector = 2.0 * normal * normal.dot(*light_vector) - light_vector;
    let R = reflection_vector;
    let V = camera_point_vector;
    let s = sphere.specular_reflection;

    let reflection_vector_dot_v = reflection_vector.dot(*camera_point_vector);
    let r_dot_v = reflection_vector_dot_v;


    if (r_dot_v > 0.0){
        out = (r_dot_v/R.magnitude() * V.magnitude()).powf(s);
    }
    return out;
}


fn adjust_color(color: &mut Color, intensity: f64){
    // :(
    // i don't like this either
    let color_r_f: f64 = color.r as f64 * intensity;
    let color_g_f: f64 = color.g as f64 * intensity;
    let color_b_f: f64 = color.b as f64 * intensity;
    color.r = color_r_f.clamp(0.0, 255.0) as u8;
    color.g = color_g_f.clamp(0.0, 255.0) as u8;
    color.b = color_b_f.clamp(0.0, 255.0) as u8;
    color.a = 255;

}

fn draw_point(x: i32, y: i32, color: Color, my_buffer: &mut Vec<(Color)>) {
    let pos = convert_from_screen_to_raster(x,y);
    let index = (((pos.1 * CANVAS_WIDTH) + pos.0)) as usize;
    my_buffer[index] = color;

}

fn convert_from_screen_to_raster(x: i32, y: i32) -> (u32, u32) {
    return (((CANVAS_WIDTH_I / 2) + x) as u32, ((CANVAS_HEIGHT_I / 2) - y) as u32);
}

fn convert_from_window_to_screen(x: i32, y: i32) -> (i32, i32){
    return (x-CANVAS_WIDTH_I_HALF, CANVAS_HEIGHT_I_HALF-y);
}

fn convert_from_canvas_to_viewport(x: i32, y: i32, viewport_distance: f64) -> Vector3<f64>{
    let x_out = x as f64 * (VIEWPORT_WIDTH as f64 /CANVAS_WIDTH_I as f64);
    let y_out = y as f64 * (VIEWPORT_HEIGHT as f64 /CANVAS_HEIGHT_I as f64);
    let z_out = viewport_distance;

    return Vector3::new(x_out as f64, y_out as f64, z_out);
}
