#[macro_use]
extern crate simple_error;

mod declaration;
mod entity;
mod map;
mod state;
mod player;



use macroquad::prelude::*;



fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Mario".to_owned(),
        fullscreen: false,
        window_width: declaration::WIDTH as i32,
        window_height: declaration::HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() -> lib_game::GResult<()> {

    let bkg_col = Color::new(0.41,0.54,1.0,1.0);

    let mut state = state::State::init().await?;

    loop {
        clear_background(bkg_col);
        state.handle_input();

        state.update();

        state.render();

        next_frame().await;
    }


}
