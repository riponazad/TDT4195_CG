// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::convert::TryInto;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};
use glm::{Mat4, vec3};

mod shader;
mod util;
mod mesh;
mod scene_graph;
use glm::translation;
use scene_graph::SceneNode;
mod toolbox;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  pointer_to_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, vertColors: &Vec<f32>, vertNormals: &Vec<f32>) -> u32 {
    // Implement me!
    // This should:


    // * Generate a VAO and bind it
    let mut vao_id:u32 = 0;
    gl::GenVertexArrays(1,&mut vao_id);
    gl::BindVertexArray(vao_id);

    // * Generate a VBO and bind it
    let mut vbo_id:u32 = 0;
    gl::GenBuffers(1, &mut vbo_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices), //vertices.as_ptr() as *const _ 
        gl::STATIC_DRAW
    );
    // * Configure a VAP for the data and enable it
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        0 as *const _
    );
    gl::EnableVertexAttribArray(0);


    // * Generate a VBO for colors and bind it
    let mut vbo2_id:u32 = 0;
    gl::GenBuffers(1, &mut vbo2_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo2_id);
    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertColors),
        pointer_to_array(vertColors), //vertices.as_ptr() as *const _ 
        gl::STATIC_DRAW
    );
    // * Configure a VAP for the colors of vertices and enable it
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        0,
        0 as *const _
    );
    gl::EnableVertexAttribArray(1);


    // * Generate a VBO for normals and bind it
    let mut vbo3_id:u32 = 0;
    gl::GenBuffers(1, &mut vbo3_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo3_id);
    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertNormals),
        pointer_to_array(vertNormals), //vertices.as_ptr() as *const _ 
        gl::STATIC_DRAW
    );
    // * Configure a VAP for the colors of vertices and enable it
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        0 as *const _
    );
    gl::EnableVertexAttribArray(2);


    // * Generate a IBO and bind it
    let mut idx:u32 = 0;
    gl::GenBuffers(1, &mut idx);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, idx);

    // * Fill it with data
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW
    );

    // * Return the ID of the VAO
    return vao_id;
}


//function to draw the scenegraph
unsafe fn draw_scene(node: &scene_graph::SceneNode, view_projection_matrix: &Mat4, transformation_so_far:& Mat4) {
    // Check if node is drawable, if so: set uniforms and draw
    //node.print();
    let mut trans: Mat4 = glm::identity();
    if node.vao_id != 0 {
        
        //Compute the current node’s relative transformation matrix
        let temp_ref = -node.reference_point;
        trans = glm::translation(&temp_ref)*trans;
        trans = glm::scaling(&node.scale)*trans;
        trans = glm::rotation(node.rotation[2], &glm::vec3(0.0, 0.0, 1.0))*trans;
        trans = glm::rotation(node.rotation[1], &glm::vec3(0.0, 1.0, 0.0))*trans;
        trans = glm::rotation(node.rotation[0], &glm::vec3(1.0, 0.0, 0.0))*trans;


        trans = glm::translation(&node.reference_point)*trans;
        trans = glm::translation(&node.position)*trans;
        trans = transformation_so_far*trans;
        gl::UniformMatrix4fv(8, 1, gl::FALSE, trans.as_ptr());

        let mut mvp=view_projection_matrix*trans;
        gl::BindVertexArray(node.vao_id);
        let mut trans: Mat4 = *transformation_so_far;
        trans = glm::translation(&vec3(1.0,0.0,0.0)) * trans;
        gl::UniformMatrix4fv(3, 1, gl::FALSE, view_projection_matrix.as_ptr());
        gl::UniformMatrix4fv(3, 1, gl::FALSE, mvp.as_ptr());
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count, // indices.len() as i32,
            gl::UNSIGNED_INT,
            0 as *const _
        ); 
    }
    // Recurse
    for &child in &node.children {
        draw_scene(&*child, &view_projection_matrix, &trans);
    }
}


fn constuct_helicopter(helicopter_mesh: &mesh::Helicopter)-> scene_graph::Node {
    let helicopter_body_vao = unsafe {
        create_vao(
            &helicopter_mesh.body.vertices,
            &helicopter_mesh.body.indices,
            &helicopter_mesh.body.colors,
            &helicopter_mesh.body.normals
        )
    };

    //create vao for the door of the helicopter
    let helicopter_door_vao = unsafe {
        create_vao(
            &helicopter_mesh.door.vertices,
            &helicopter_mesh.door.indices,
            &helicopter_mesh.door.colors,
            &helicopter_mesh.door.normals
        )
    };

    //create vao for the main rotor of the helicopter
    let helicopter_main_rotor_vao = unsafe {
        create_vao(
            &helicopter_mesh.main_rotor.vertices,
            &helicopter_mesh.main_rotor.indices,
            &helicopter_mesh.main_rotor.colors,
            &helicopter_mesh.main_rotor.normals
        )
    };

    //create vao for the tail rotor of the helicopter
    let helicopter_tail_rotor_vao = unsafe {
        create_vao(
            &helicopter_mesh.tail_rotor.vertices,
            &helicopter_mesh.tail_rotor.indices,
            &helicopter_mesh.tail_rotor.colors,
            &helicopter_mesh.tail_rotor.normals
        )
    };
    let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao, helicopter_mesh.body.index_count);
    let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao, helicopter_mesh.door.index_count);
    let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao, helicopter_mesh.main_rotor.index_count);
    let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter_mesh.tail_rotor.index_count);


    // chancging some values for evaluation
    helicopter_body_node.rotation[1] = 90.0;
    helicopter_body_node.position[2] = -10.0;
    //Organize the scene graph
    helicopter_body_node.add_child(&helicopter_door_node);
    helicopter_body_node.add_child(&helicopter_main_rotor_node);
    helicopter_body_node.add_child(&helicopter_tail_rotor_node);

    //set the reference points for the nodes
    helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
    helicopter_main_rotor_node.reference_point = glm::vec3(0.0, 2.3, 0.0);
    helicopter_body_node.reference_point = glm::vec3(0.0, 0.0, 0.0);

    return helicopter_body_node;

}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here

        //Declaring verties as input buffers
        let vertices: Vec<f32> = vec![
            /* -0.6, -0.6, 0.5,
            0.6, -0.6, 0.5,
            0.0, 0.6, 0.5,
            -0.6, 0.4, 0.0,
            0.0, -0.8, 0.0,
            0.6, 0.4, 0.0,
            -0.8, -0.1, 0.9,
            0.4, -0.8, 0.9,
            0.4, 0.6, 0.9,*/
            -0.8, -0.8, -1.0,
            -0.3, -0.8, -1.0,
            -0.5, -0.3, -1.0,
            0.3, -0.8, -1.0,
            0.8, 0.1, -1.0,
            0.0, -0.3, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.6, -1.0,
            -0.6, 0.6, -1.0,
        ];

        //Declaring colors of all verties
        let vertColors: Vec<f32> = vec![
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            0.7, 0.1, 0.5, 1.0,
            0.0, 0.1, 0.7, 1.0,
            0.1, 0.9, 0.5, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            0.7, 0.1, 0.5, 1.0
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
        ];

        //let my_vao = unsafe { create_vao(&vertices, &indices, &vertColors) };

        //load the terrain model obj
        let terrain_mesh = mesh::Terrain::load("./resources/lunarsurface.obj");

        //create vao for the terrain
        let terrain_vao = unsafe { 
            create_vao(
                &terrain_mesh.vertices,
                &terrain_mesh.indices,
                &terrain_mesh.colors,
                &terrain_mesh.normals
            ) 
        };

        //root of the scene
        let mut root_scene = SceneNode::new();

        //Generate SceneNodes for every objects
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain_mesh.index_count);
        root_scene.reference_point = glm::vec3(0.0,0.0,0.0);
        terrain_node.reference_point = glm::vec3(0.0,0.0,0.0);
        root_scene.add_child(&terrain_node);

        let helicopter_mesh = mesh::Helicopter::load("./resources/helicopter.obj");
        let mut mul_helicopter: Vec<scene_graph::Node> = Vec::new();
        for _i in 0..5 {
            let helicopter = constuct_helicopter(&helicopter_mesh);
            terrain_node.add_child(&helicopter);
            mul_helicopter.push(helicopter);
        }

        //load the helicopter model obj
        // let helicopter_mesh = mesh::Helicopter::load("./resources/helicopter.obj");
        //
        // //create vao for the body of the helicopter
        // let helicopter_body_vao = unsafe {
        //     create_vao(
        //         &helicopter_mesh.body.vertices,
        //         &helicopter_mesh.body.indices,
        //         &helicopter_mesh.body.colors,
        //         &helicopter_mesh.body.normals
        //     )
        // };
        //
        // //create vao for the door of the helicopter
        // let helicopter_door_vao = unsafe {
        //     create_vao(
        //         &helicopter_mesh.door.vertices,
        //         &helicopter_mesh.door.indices,
        //         &helicopter_mesh.door.colors,
        //         &helicopter_mesh.door.normals
        //     )
        // };
        //
        // //create vao for the main rotor of the helicopter
        // let helicopter_main_rotor_vao = unsafe {
        //     create_vao(
        //         &helicopter_mesh.main_rotor.vertices,
        //         &helicopter_mesh.main_rotor.indices,
        //         &helicopter_mesh.main_rotor.colors,
        //         &helicopter_mesh.main_rotor.normals
        //     )
        // };
        //
        // //create vao for the tail rotor of the helicopter
        // let helicopter_tail_rotor_vao = unsafe {
        //     create_vao(
        //         &helicopter_mesh.tail_rotor.vertices,
        //         &helicopter_mesh.tail_rotor.indices,
        //         &helicopter_mesh.tail_rotor.colors,
        //         &helicopter_mesh.tail_rotor.normals
        //     )
        // };
        //
        //
        // let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao, helicopter_mesh.body.index_count);
        // let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao, helicopter_mesh.door.index_count);
        // let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao, helicopter_mesh.main_rotor.index_count);
        // let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter_mesh.tail_rotor.index_count);
        //
        //
        // // chancging some values for evaluation
        // helicopter_body_node.rotation[1] = 90.0;
        // helicopter_body_node.position[2] = -10.0;
        // //Organize the scene graph
        // helicopter_body_node.add_child(&helicopter_door_node);
        // helicopter_body_node.add_child(&helicopter_main_rotor_node);
        // helicopter_body_node.add_child(&helicopter_tail_rotor_node);
        //
        // //set the reference points for the nodes
        // helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        // helicopter_main_rotor_node.reference_point = glm::vec3(0.0, 2.3, 0.0);
        // helicopter_body_node.reference_point = glm::vec3(0.0, 0.0, 0.0);

        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };
        unsafe{
            simple_shader.activate();
        }


        // Variable to store motion values in x, y, and z coordinates
        let mut motion: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0];  //translationXYZ [0, 1, 2], rotationXYZ [3, 4, 5]

        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut prevous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(prevous_frame_time).as_secs_f32();
            prevous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Resized");
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        /* VirtualKeyCode::A => {
                            _arbitrary_number += delta_time;
                        }
                        VirtualKeyCode::D => {
                            _arbitrary_number -= delta_time;
                        } */


                        // Translation -> X
                        VirtualKeyCode::D =>{
                            motion[0] += 20.0*delta_time;
                        }
                        VirtualKeyCode::A =>{
                            motion[0] -= 20.0*delta_time;
                        }
                        // Translation -> Y
                        VirtualKeyCode::W =>{
                            motion[1] += 20.0*delta_time;
                        }
                        VirtualKeyCode::S =>{
                            motion[1] -= 20.0*delta_time;
                        }
                        // Translation -> Z
                        VirtualKeyCode::Space =>{
                            motion[2] += 20.0*delta_time;
                        }
                        VirtualKeyCode::LShift =>{
                            motion[2] -= 20.0*delta_time;
                        }

                        // Rotation -> X
                        VirtualKeyCode::Left =>{
                            motion[3] += 20.0*delta_time;
                        }
                        VirtualKeyCode::Right =>{
                            motion[3] -= 20.0*delta_time;
                        }
                        // Rotation -> Y
                        VirtualKeyCode::Up =>{
                            motion[4] += 20.0*delta_time;
                        }
                        VirtualKeyCode::Down =>{
                            motion[4] -= 20.0*delta_time;
                        }
                        // Rotation -> Z
                        VirtualKeyCode::Key1 =>{
                            motion[5] += 20.0*delta_time;
                        }
                        VirtualKeyCode::Key5 =>{
                            motion[5] -= 20.0*delta_time;
                        }

                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the acumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camodelmera transforms here (exercise 2 & 3)
            let perspective: Mat4 = glm::perspective(
                window_aspect_ratio,
                90.0,
                1.0,
                1000.0
            );
            let mut transformMatrix: Mat4 = glm::identity();
            transformMatrix = glm::translation(&glm::vec3(motion[0], motion[1], motion[2])) * transformMatrix;
            transformMatrix = glm::rotation(motion[3].to_radians(), &glm::vec3(1.0, 0.0, 0.0)) * transformMatrix;
            transformMatrix = glm::rotation(motion[4].to_radians(), &glm::vec3(0.0, 1.0, 0.0)) * transformMatrix;
            transformMatrix = glm::rotation(motion[5].to_radians(), &glm::vec3(0.0, 0.0, 1.0)) * transformMatrix;
            transformMatrix = perspective * transformMatrix;
            /* unsafe{
                gl::UniformMatrix4fv(3, 1, gl::FALSE, transformMatrix.as_ptr());
            } */

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                //gl::Uniform1f(2, elapsed.sin());
                // Rotor spining animation
                // helicopter_main_rotor_node.rotation.y = 100.0*elapsed;
                // helicopter_tail_rotor_node.rotation.x = 200.0*elapsed;

                //heading
                // let heading = toolbox::simple_heading_animation(elapsed/3.0 as f32);
                // //setup for animation
                // helicopter_body_node.position.x = heading.x;
                // helicopter_body_node.position.z = heading.z;
                // helicopter_body_node.rotation.z = heading.roll;
                // helicopter_body_node.rotation.y = heading.yaw;
                // helicopter_body_node.rotation.x = heading.pitch;

                for i in 0..5 {
                    let path = toolbox::simple_heading_animation(elapsed - 600.0 * i as f32);
                    mul_helicopter[i].position.x = path.x;
                    mul_helicopter[i].position.z = path.z;
                    mul_helicopter[i].rotation.x = path.pitch;
                    mul_helicopter[i].rotation.y = path.yaw;
                    mul_helicopter[i].rotation.z = path.roll;
                    mul_helicopter[i][0].rotation.y = 5.0 * elapsed; // rotate main rotor
                    mul_helicopter[i][1].rotation.x = 10.0 * elapsed; // rotate tail rotor
                }

                // == // Issue the necessary gl:: commands to draw your scene here
                /* gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter_mesh.body.index_count, // indices.len() as i32,
                    gl::UNSIGNED_INT,
                    0 as *const _
                ); */
                let mut tra: Mat4 = glm::identity();
                // unsafe{
                //     gl::UniformMatrix4fv(40, 1, gl::FALSE, tra.as_ptr());
                // }
                draw_scene(&root_scene, &transformMatrix, &tra);

                let mut trans_so_far: Mat4 = glm::identity();
                
                //trans_so_far = glm::translation(&glm::vec3(motion[0], motion[1], motion[2])) * trans_so_far;

                draw_scene(&root_scene, &transformMatrix, &trans_so_far);

                //gl::DrawArrays(gl::TRIANGLES, 0, 3);



            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size! width: {}, height: {}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
