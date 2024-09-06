use std::cmp::PartialEq;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cgmath::{InnerSpace, Vector3};
use cgmath::num_traits::Euclid;
use winit::event::VirtualKeyCode::V;
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
#[derive(Copy, Clone)]
struct Sphere{
    r: f64,
    origin: Vector3<f64>,
    color: Color
}

struct Light {
    kind: LightType,
    pos_or_direction: Vector3<f64>,
    intensity: f64
}



const CANVAS_WIDTH: u32 = 1400;
const CANVAS_HEIGHT: u32 = 1400;

const CANVAS_WIDTH_I: i32 = CANVAS_WIDTH as i32;
const CANVAS_HEIGHT_I: i32 = CANVAS_HEIGHT as i32;

const CANVAS_WIDTH_I_HALF: i32 = CANVAS_WIDTH_I / 2;
const CANVAS_HEIGHT_I_HALF: i32 = CANVAS_HEIGHT_I / 2;

const VIEWPORT_WIDTH: i32 = 1;
const VIEWPORT_HEIGHT: i32 = 1;



const CAMERA_POSITION: Vector3<f64> = Vector3{x:0.0,y:0.0,z:0.0};

// const LIGHTS: [Light;3] = [
//     Light{kind:LightType::Ambient,pos_or_direction:Vector3{x:0.0,y:0.0,z:0.0},intensity:0.1},
//     Light{kind:LightType::Point,pos_or_direction:Vector3{x:1.0,y:1.5,z:2.0},intensity:0.9},
//     Light{kind:LightType::Directional,pos_or_direction:Vector3{x:-1.0,y:0.0,z:4.0},intensity:0.0}
// ];

const SPHERES: [Sphere;3] = [
   Sphere{r:1.0,origin:Vector3{x:0.0,y:0.5,z:6.0},color:Color::RED},
   Sphere{r:1.0,origin:Vector3{x:-1.0,y:0.9,z:3.0},color:Color::BLUE},
    Sphere{r:1.0,origin:Vector3{x:1.0,y:-1.0,z:4.0},color:Color::GREEN},
    // Sphere{r:1.5,origin:Vector3{x:1.0,y:-1.0,z:5.0},color:Color{r:1.0,g:1.0,b:0.0,a:1.0}},
    // Sphere{r:4.0,origin:Vector3{x:3.0,y:4.0,z:10.0},color:Color{r:1.0,g:0.0,b:1.0,a:1.0}},
    // Sphere{r:0.3,origin:Vector3{x:-1.0,y:0.9,z:2.0},color:Color{r:1.0,g:0.6,b:1.0,a:1.0}},
    // Sphere{r:0.1,origin:Vector3{x:-0.1,y:0.0,z:1.5},color:Color{r:0.6,g:0.3,b:0.5,a:1.0}},
    // Sphere{r:0.05,origin:Vector3{x:-0.1,y:0.0,z:1.3},color:Color{r:0.3,g:0.3,b:1.0,a:1.0}},
    // Sphere{r:0.01,origin:Vector3{x:-0.1,y:0.0,z:1.1},color:Color{r:0.3,g:0.5,b:1.0,a:1.0}},
    // Sphere{r:0.05,origin:Vector3{x:0.0,y:0.0,z:1.5},color:Color{r:0.3,g:0.5,b:1.0,a:1.0}},
    // Sphere{r:10.0,origin:Vector3{x:0.0,y:0.0,z:20.0},color:Color{r:0.5,g:0.75,b:1.0,a:1.0}},
    // Sphere{r:100.0,origin:Vector3{x:0.0,y:-101.0,z:0.0},color:Color{r:1.0,g:0.00,b:1.0,a:1.0}},
];

// const SPHERES: [Sphere;4] = [
//     Sphere{r:1.0,origin:Vector3{x:0.0,y:0.5,z:3.0},color:Color::RED},
//     Sphere{r:1.0,origin:Vector3{x:-1.0,y:0.9,z:3.0},color:Color::BLUE},
//     Sphere{r:1.0,origin:Vector3{x:1.0,y:-1.0,z:3.0},color:Color::GREEN},
//     Sphere{r:1.5,origin:Vector3{x:1.0,y:-1.0,z:3.0},color:Color{r:1.0,g:1.0,b:0.0,a:1.0}},
//     // Sphere{r:4.0,origin:Vector3{x:3.0,y:4.0,z:10.0},color:Color{r:1.0,g:0.0,b:1.0,a:1.0}},
//     // Sphere{r:0.3,origin:Vector3{x:-1.0,y:0.9,z:2.0},color:Color{r:1.0,g:0.6,b:1.0,a:1.0}},
//     // Sphere{r:0.1,origin:Vector3{x:-0.1,y:0.0,z:1.5},color:Color{r:0.6,g:0.3,b:0.5,a:1.0}},
//     // Sphere{r:0.05,origin:Vector3{x:-0.1,y:0.0,z:1.3},color:Color{r:0.3,g:0.3,b:1.0,a:1.0}},
//     // Sphere{r:0.01,origin:Vector3{x:-0.1,y:0.0,z:1.1},color:Color{r:0.3,g:0.5,b:1.0,a:1.0}},
//     // Sphere{r:0.05,origin:Vector3{x:0.0,y:0.0,z:1.5},color:Color{r:0.3,g:0.5,b:1.0,a:1.0}},
//     // Sphere{r:10.0,origin:Vector3{x:0.0,y:0.0,z:20.0},color:Color{r:0.5,g:0.75,b:1.0,a:1.0}},
//     // Sphere{r:100.0,origin:Vector3{x:0.0,y:-101.0,z:0.0},color:Color{r:1.0,g:0.00,b:1.0,a:1.0}},
// ];


fn main() -> Result<(), Error>  {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<Color> = Vec::new();
    let mut viewport_distance: f64 = 1.0;
    // let mut window_pos: (i32,i32) = (0,0);
    my_buffer.resize((((CANVAS_HEIGHT + 1) * (CANVAS_WIDTH + 1))) as usize, Color::BLACK);
    let mut direction: Direction = Direction::In;
    let mut mouse_pos: (i32, i32) = (0,0);
    let mut light_z: f64 = 0.0;
    let mut lights: Vec<Light> = vec![
    Light{kind:LightType::Ambient,pos_or_direction:Vector3{x:0.0,y:0.0,z:0.0},intensity:0.0},
    Light{kind:LightType::Point,pos_or_direction:Vector3{x:-4.0,y:-4.0,z:4.0},intensity:0.85},
    Light{kind:LightType::Directional,pos_or_direction:Vector3{x:-1.0,y:0.0,z:4.0},intensity:0.05}
    ];
    let window = {
        let size = LogicalSize::new(CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            // .with_position()
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
            render_to_my_buffer(&mut my_buffer,&lights, viewport_distance);

            copy_to_pixels(&my_buffer,pixels.frame_mut());
            // update_light_position(&mut lights, &mut direction);
            match input.mouse(){
                Some(mouse) => {
                    mouse_pos = convert_from_window_to_screen((mouse.0/2.0) as i32 ,(mouse.1/2.0) as i32);
                    // draw_point(mouse_pos.0, mouse_pos.1, Color::RED, &mut my_buffer);
                    lights[1].pos_or_direction = convert_from_canvas_to_viewport(mouse_pos.0, mouse_pos.1, viewport_distance);
                    lights[1].pos_or_direction.z = light_z;
                    // lights[1].pos_or_direction.z = convert_from_canvas_to_viewport(mouse_pos.0, mouse_pos.1, viewport_distance);
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

            if input.key_pressed(VirtualKeyCode::Up){
                viewport_distance+=1.0;
            }




        }


        pixels.render().expect("TODO: panic message");


        window.request_redraw();


    });
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

fn render_to_my_buffer(my_buffer: &mut Vec<Color>, lights: &Vec<Light>, viewport_distance: f64) {
    // println!("do_drawing called");
    for x in -CANVAS_WIDTH_I/2..CANVAS_WIDTH_I/2 {
        for y in -CANVAS_HEIGHT_I/2..CANVAS_HEIGHT_I/2 {
            let d = convert_from_canvas_to_viewport(x,y, viewport_distance);
            let color = trace_ray(CAMERA_POSITION, d, lights, viewport_distance, f64::INFINITY);  //trace ray from (d.x,d.y,d.z)
            draw_point(x,y,color,my_buffer);
            //draw color on canvas at points x and y
        }
    }
}

fn copy_to_pixels(my_buffer: &Vec<Color>, pixels_buffer: &mut [u8]){
    for (index, pixel) in pixels_buffer.chunks_exact_mut(4).enumerate() {
        pixel[0] = (my_buffer[index].r * 255.0) as u8;
        pixel[1] = (my_buffer[index].g * 255.0) as u8;
        pixel[2] = (my_buffer[index].b * 255.0) as u8;
        pixel[3] = (my_buffer[index].a * 255.0) as u8;
    }
}

fn intensity_at_point(point: Vector3<f64>,  normal: Vector3<f64>, lights: &Vec<Light>) -> f64 {
    //
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
        intensity = intensity + ((light.intensity * top_factor) / (normal.magnitude() * light_vector.magnitude()));
    }
    return intensity;
}

fn trace_ray(ray_origin: Vector3<f64>, ray_direction: Vector3<f64>, lights: &Vec<Light>, point_min: f64, point_max: f64) -> Color{
    //find out where the ray is going
    //and if it interescts with a sphere,
    //return the points along the ray where it interesects
    //because the straight line is going through the sphere, it interesects it twice
    //front and back
    //we just want front
    // println!("trace ray called");
    let mut closest_solution: f64 = f64::INFINITY;
    let mut closest_sphere: Option<Sphere> = None;
    let mut intersections: (f64,f64);
    let mut point: Vector3<f64>;
    let mut sphere_normal: Vector3<f64>;
    let intensity: f64;
    for sphere in SPHERES{
        intersections = ray_sphere_intersection(&sphere, ray_origin, ray_direction);
        if(intersections.0 < closest_solution && intersections.0 > point_min && intersections.0 < point_max){
            closest_solution = intersections.0;
            closest_sphere = Some(sphere);
        }
        if(intersections.1 < closest_solution && intersections.1 > point_min && intersections.1 < point_max){
            closest_solution = intersections.1;
            closest_sphere = Some(sphere);
        }
    }
    match closest_sphere{
        None => Color::WHITE,
        Some(sphere) => {
            point = ray_origin + (closest_solution * ray_direction);
            sphere_normal = point - sphere.origin;
            sphere_normal = sphere_normal.normalize();
            intensity = intensity_at_point(point,sphere_normal,lights);
            return Color{r:sphere.color.r*intensity,g:sphere.color.g*intensity,b:sphere.color.b*intensity,a:1.0};
        }
    }


    // return Vector3::new(1,2,3);

}

//this function uses geometry to solve for an equation that checks for line sphere intersection
fn ray_sphere_intersection(sphere: &Sphere, ray_origin: Vector3<f64>, ray_direction: Vector3<f64>) -> (f64,f64) {
    let co = ray_origin-sphere.origin; //
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * co.dot(ray_direction);
    let c = (co.dot(co) - (sphere.r * sphere.r));
    let discriminant = (b * b - (4.0 * a * c));
    if(discriminant == 0.0){
        return (((-b - (discriminant).sqrt()) / 2.0*a), f64::INFINITY);
    }
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    return (t1, t2);


}

fn draw_point(x: i32, y: i32, color: Color, my_buffer: &mut Vec<(Color)>) {
    let pos = convert_from_screen_to_raster(x,y);
    let index = (((pos.1 * CANVAS_WIDTH) + pos.0)) as usize;
    // temporary for debugging
    // if(index > 1764000){
        // println!("debug");
        // return;
    // }
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
