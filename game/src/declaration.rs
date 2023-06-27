
pub const ASSETS_DIR: &str = "../assets";
pub const TILE_DIR: &str = "../assets/tiles";

pub const WIDTH:f32 = 960.0;
pub const HEIGHT:f32 = 540.0;

pub const TILE_SIZE:f32 = 16.0;
pub const NORM_WIDTH_TILE_SIZE:f32 = (TILE_SIZE/ (WIDTH/2.0));
pub const NORM_HEIGHT_TILE_SIZE:f32 = (TILE_SIZE/ (HEIGHT/2.0));

pub const MAP_HEIGHT:f32 = TILE_SIZE * 14.0;
pub const MAP_WIDTH:f32 = TILE_SIZE * 211.0;

pub const GRAVITY: f32 = 10.0;


#[derive(PartialEq,Copy, Clone)]
pub enum Direction {
    Bottom,
    Top,
    Left,
    Right
}