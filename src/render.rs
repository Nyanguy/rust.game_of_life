extern crate piston_window;

use piston_window::*;
use crate::game_field::GAMEFIELD;
use std::string::String;

pub fn play(filename: String, cell_size: u64, dimensions: [u32; 2]) -> std::io::Result<()> {

    let mut gf = GAMEFIELD::new();
    gf.read_file(filename)?;

    let mut window: PistonWindow =
        WindowSettings::new("RGOL", dimensions)
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        if let Some(e) = event.press_args() {
            match e {
                Button::Keyboard(Key::Q) => gf.next_generation(),
                _ => ()
            }
        };
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            std::thread::sleep(std::time::Duration::from_secs(2));
            for cell in gf.get_cells() {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [cell.get_x() as f64, 
                           cell.get_y() as f64, 
                           cell_size as f64, 
                           cell_size as f64],
                           context.transform,
                           graphics);
            }
        });
    }
    Ok(())
}