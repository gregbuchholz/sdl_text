/* Minimal example to showcase the Rust SDL2 bindings working with an emscripten target (asmjs,wasm32).
 * Build:
 *  source emsdk/emsdk_env.sh 
 *  cd src/
 *  emcc -c gxx_personality_v0_stub.cpp
 *  cargo build --target=wasm32-unknown-emscripten --release
 * Run:
 *   emrun index.html
 */
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::convert::TryInto;
use std::path::Path;

fn main() -> Result<(), String> {
    
    #[cfg(target_os = "emscripten")]
    let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?; 
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let (width,height) = (640,480);
    let window = video_subsystem
        .window("SDL Text Test", width, height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();

    let font_path = Path::new("assets/DejaVuSansMono.ttf");
    let font = ttf_context.load_font(font_path, 12)?;

    let ts1 = font.render("Hello SDL! Solid").solid(Color::RGBA(125, 0, 125, 255)).map_err(|e| e.to_string())?;
    let tt1 = creator.create_texture_from_surface(&ts1).map_err(|e| e.to_string())?;
    let t1_rect = Rect::new(100, 100, tt1.query().width, tt1.query().height);
    
    let ts2 = font.render("Hello SDL! Shaded").shaded(Color::RGBA(125, 0, 125, 255),Color::RGBA(200,200,200,255)).map_err(|e| e.to_string())?;
    let tt2 = creator.create_texture_from_surface(&ts2).map_err(|e| e.to_string())?;
    let t2_rect = Rect::new(100, 120, tt2.query().width, tt2.query().height);
    
    let ts3 = font.render("Hello SDL! Blended").blended(Color::RGBA(125, 0, 125, 255)).map_err(|e| e.to_string())?;
    let tt3 = creator.create_texture_from_surface(&ts3).map_err(|e| e.to_string())?;
    let t3_rect = Rect::new(100, 140, tt3.query().width, tt3.query().height);

    //setup the background image
    let initial_bg_rect = Rect::new(0,0, width, height);
    let bg_rect_dest = initial_bg_rect.clone();
    let bg_rect_src = initial_bg_rect.clone(); 
    let mut bg_texture = creator
        .create_texture_target(PixelFormatEnum::ARGB8888, width, height)
        .map_err(|e| e.to_string())?;
    
    canvas.with_texture_canvas(&mut bg_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(230,230,230,255));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(0,0,255,255));
            { 
                let w:i32 = width.try_into().unwrap();
                let h:i32 = height.try_into().unwrap();
                texture_canvas.draw_line(Point::new(w-1,0),
                                         Point::new(0,h-1)).unwrap();
                texture_canvas.set_draw_color(Color::RGBA(0,255,128,255));
                texture_canvas.draw_rect(Rect::new(300,200,150,100)).unwrap();
            }
            texture_canvas.copy(&tt1, None, t1_rect).unwrap();
        }).map_err(|e| e.to_string())?;
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        let mut potential_event = Some(event_pump.wait_event()); //blocking wait for events
        
        canvas.set_draw_color(Color::RGBA(230,230,230,255));
        canvas.clear(); 
        canvas.copy(&bg_texture, bg_rect_src, bg_rect_dest).unwrap();
        //canvas.copy(&tt1, None, t1_rect)?;
        canvas.copy(&tt2, None, t2_rect)?;
        canvas.copy(&tt3, None, t3_rect)?;
       
        canvas.set_draw_color(Color::RGBA(255,0,255,255));

        while let Some(event) = potential_event { 
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape),..} 
                | Event::KeyDown {keycode: Some(Keycode::Q),..} 
                | Event::Quit { .. } => { 
                    break 'mainloop; },
                Event::MouseMotion {x, y, .. } => {
                    canvas.set_draw_color(Color::RGBA(255,0,0,255));
                    canvas.draw_line(Point::new(0,0), Point::new(x,y)).unwrap();
                    ()},
                _ => {
                    println!("{:?}",event);
                    ()}
            } //match
            potential_event = event_pump.poll_event();
        } //while

        canvas.present();
    };
    
    Ok(())
}
