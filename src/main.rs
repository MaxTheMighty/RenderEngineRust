
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cgmath::{InnerSpace, Vector2, Vector3};
use winit::event::VirtualKeyCode::V;

struct Sphere{
    r: f64,
    origin: Vector3<f64>,
    color: Color
}

const CANVAS_WIDTH: u32 = 1680;
const CANVAS_HEIGHT: u32 = 1050;

const CANVAS_WIDTH_I: i32 = CANVAS_WIDTH as i32;
const CANVAS_HEIGHT_I: i32 = CANVAS_HEIGHT as i32;

const VIEWPORT_WIDTH: i32 = 1680;
const VIEWPORT_HEIGHT: i32 = 1050;



const CAMERA_POSITION: Vector3<f64> = Vector3{x:0.0,y:0.0,z:0.0};

const SPHERES: [Sphere;1] = [
   // Sphere{r:20.0,origin:Vector3{x:-5.0,y:-1.0,z:1.0},color:Color::RED},
   // Sphere{r:20.0,origin:Vector3{x:40.0,y:0.0,z:2.0},color:Color::BLUE},
    Sphere{r:100.0,origin:Vector3{x:-100.0,y:-50.0,z:20.0},color:Color::GREEN}
];
fn main() -> Result<(), Error>  {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<Color> = Vec::new();
    let mut viewport_distance: f64 = 1.0;
    my_buffer.resize((((CANVAS_HEIGHT + 1) * (CANVAS_WIDTH + 1))) as usize, Color::BLACK);
    let window = {
        let size = LogicalSize::new(CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
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
            // println!("redraw requested");
            render_to_my_buffer(&mut my_buffer,viewport_distance);
            // my_buffer[0] = Color::RED;
            copy_to_pixels(&my_buffer,pixels.frame_mut());
            // do_drawing(pixels.frame_mut());
            // draw_point(100,100,,pixels.frame_mut());
            // println!("redraw requested");

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

fn render_to_my_buffer(my_buffer: &mut Vec<Color>, viewport_distance: f64) {
    // println!("do_drawing called");
    for x in -CANVAS_WIDTH_I/2..CANVAS_WIDTH_I/2 {
        for y in -CANVAS_HEIGHT_I/2..CANVAS_HEIGHT_I/2 {
            let d = convert_from_canvas_to_viewport(x,y, viewport_distance);
            let color = trace_ray(CAMERA_POSITION, d, viewport_distance, f64::INFINITY);  //trace ray from (d.x,d.y,d.z)
            draw_point(x,y,color,my_buffer);
            //draw color on canvas at points x and y
        }
    }
}

fn copy_to_pixels(my_buffer: &Vec<Color>, pixels_buffer: &mut [u8]){
    for (index, pixel) in pixels_buffer.chunks_exact_mut(4).enumerate() {
        pixel[0] = (my_buffer[index].r as u8 * 255);
        pixel[1] = (my_buffer[index].g as u8 * 255);
        pixel[2] = (my_buffer[index].b as u8 * 255);
        pixel[3] = (my_buffer[index].a as u8 * 255);
    }
}

// fn do_drawing(my_buffer: &Vec<(u8, u8, u8, u8)>){
//     println!("do_drawing called");
//     for x in -CANVAS_WIDTH_I/2..CANVAS_WIDTH_I/2 {
//         for y in -CANVAS_HEIGHT_I/2..CANVAS_HEIGHT_I/2 {
//             let d = convert_from_canvas_to_viewport(x,y);
//             let color = trace_ray(CAMERA_POSITION, d, 1.0, f64::INFINITY);//trace ray from (d.x,d.y,d.z)
//
//             // draw_point(x,y,color,pixels_buffer,my_buffer);
//             //draw color on canvas at points x and y
//         }
//     }
//     // draw_point(buffer)
// }

fn trace_ray(ray_origin: Vector3<f64>, ray_direction: Vector3<f64>, point_min: f64, point_max: f64) -> Color{
    //find out where the ray is going
    //and if it interescts with a sphere,
    //return the points along the ray where it interesects
    //because the straight line is going through the sphere, it interesects it twice
    //front and back
    //we just want front
    // println!("trace ray called");
    let mut closest_solution: f64 = f64::INFINITY;
    let mut closest_color: Color = Color::WHITE;
    let mut intersections: (f64,f64);
    for sphere in SPHERES{
        intersections = ray_sphere_intersection(&sphere, ray_origin, ray_direction);
        if(intersections.0 < closest_solution && intersections.0 > point_min && intersections.0 < point_max){
            closest_solution = intersections.0;
            closest_color = sphere.color;
        }
        if(intersections.1 < closest_solution && intersections.1 > point_min && intersections.1 < point_max){
            closest_solution = intersections.1;
            closest_color = sphere.color;
        }
    }

    return closest_color;

    // return Vector3::new(1,2,3);

}

//this function uses geometry to solve for an equation that checks for line sphere intersection
fn ray_sphere_intersection(sphere: &Sphere, ray_origin: Vector3<f64>, ray_direction: Vector3<f64>) -> (f64,f64) {
    let co = ray_origin-sphere.origin; //
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * co.dot(ray_direction);
    let c = (co.dot(co) - (sphere.r * sphere.r));
    let discriminant = (b * b - (4.0 * a * c));
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    return ((-b + (discriminant).sqrt()) / (2.0*a), (-b - (discriminant).sqrt()) / (2.0*a));
/*
IntersectRaySphere(O, D, sphere) {
    r = sphere.radius
    CO = O - sphere.center

    a = dot(D, D)
    b = 2*dot(CO, D)
    c = dot(CO, CO) - r*r

    discriminant = b*b - 4*a*c
    if discriminant < 0 {
        return inf, inf
    }

    t1 = (-b + sqrt(discriminant)) / (2*a)
    t2 = (-b - sqrt(discriminant)) / (2*a)
    return t1, t2
}
 */

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
    // for (index, pixel) in pixels_buffer.chunks_exact_mut(4).enumerate() {
    //     pixel[0] = my_buffer[index].0;
    //     pixel[1] = my_buffer[index].1;
    //     pixel[2] = my_buffer[index].2;
    //     pixel[3] = my_buffer[index].3;
    //
    // }
    // buffer[index] = color.r as u8 * 255;
    // buffer[index + 1] = color.g as u8 * 255;
    // buffer[index + 2] = color.b as u8 * 255;
    // buffer[index + 3] = 255;
}

fn convert_from_screen_to_raster(x: i32, y: i32) -> (u32, u32) {
    return (((CANVAS_WIDTH_I / 2) + x) as u32, ((CANVAS_HEIGHT_I / 2) - y) as u32);
}

fn convert_from_canvas_to_viewport(x: i32, y: i32, viewport_distance: f64) -> Vector3<f64>{
    let x_out = x * (VIEWPORT_WIDTH as f64 /CANVAS_WIDTH_I as f64) as i32;
    let y_out = y * (VIEWPORT_HEIGHT as f64 /CANVAS_HEIGHT_I as f64) as i32;
    let z_out = viewport_distance;

    return Vector3::new(x_out as f64, y_out as f64, z_out);
}
