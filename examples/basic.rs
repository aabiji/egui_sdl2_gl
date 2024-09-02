use egui_sdl2_gl::sdl2::event::Event;
use egui_sdl2_gl::sdl2::video::{GLProfile, SwapInterval};
use egui_sdl2_gl::{egui, gl, sdl2};
use egui_sdl2_gl::{DpiScaling, ShaderVersion};
use std::time::Instant;

fn main() {
    // Create window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem
        .window("Demo", 600, 400)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Set Opengl window attributes
    let _gl_ctx = window.gl_create_context().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)
        .unwrap();

    let (mut painter, mut egui_state) =
        egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = egui::Context::default();

    let start_time = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut value = 0;

    'running: loop {
        unsafe {
            // Clear window with black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Begin rendering
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        // Define UI
        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.heading("Hello world");
            ui.add(egui::Slider::new(&mut value, 0..=10).text("Slider"));
        });

        // Process egui output
        let output = egui_ctx.end_frame();
        egui_state.process_output(&window, &output.platform_output);

        // Render
        let paint_jobs = egui_ctx.tessellate(output.shapes, output.pixels_per_point);
        painter.paint_jobs(None, output.textures_delta, paint_jobs);
        window.gl_swap_window();

        let repaint_delay = output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .expect("Missing ViewportId::ROOT")
            .repaint_delay
            .as_secs();
        // Only sleep if we don't immediately need to rerender
        let sleep = if repaint_delay > 0 { 10 } else { 0 };
        'event_loop: loop {
            match event_pump.wait_event_timeout(sleep) {
                Some(Event::Quit { .. }) => break 'running,
                Some(event) => egui_state.process_input(&window, event, &mut painter),
                None => break 'event_loop, // No more events
            }
        }
    }
}
