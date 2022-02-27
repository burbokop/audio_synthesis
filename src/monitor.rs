use std::time::{Instant, Duration};

use sdl2::{pixels::{Color, PixelFormatEnum}, rect::Point, event::Event, keyboard::Keycode, surface::Surface};





pub fn monitor_once<I>(iter: &mut I, frame_rate: u32) 
where
    I: Iterator<Item=f32> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    const width: u32 = 600;
    const height: u32 = 200;

    let window = video_subsystem.window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: i32 = 0;

    let mut timer = Instant::now();

    let masks = PixelFormatEnum::RGBA32.into_masks().unwrap();
    let mut chart_surface = Surface::from_pixelmasks(512, 512, masks).unwrap().into_canvas().unwrap();

    let mut prevPoint: Option<Point> = None;

    'running: loop {
        //timer.elapsed().as_micros();


        let color_factor = (((i as f64 * 0.01).sin() + 1.) * 127.) as u8;
        i += 1;


        canvas.set_draw_color(Color::RGB(color_factor, 64, 255 - color_factor));
        //canvas.clear();


        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        match iter.next() {
            Some(v) => {
                let y = (v + 1.) * (height as f32) / 2.;
                let x = (i % width as i32) as f32;
                let point = Point::new(x as i32, y as i32);
                match prevPoint {
                    Some(pp) => canvas.draw_line(pp, point).unwrap(),
                    None => canvas.draw_point(point).unwrap(),
                } 
                                
                if i % width as i32 == 0 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    prevPoint = None;
                } else {
                    prevPoint = Some(point);
                }
            },
            None => break,
        }


          //chart_surface.surface()



        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}