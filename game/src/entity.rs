
use crate::declaration::{
    TILE_SIZE,
    ASSETS_DIR,
    WIDTH,
    HEIGHT,
    NORM_HEIGHT_TILE_SIZE,
    NORM_WIDTH_TILE_SIZE,
    MAP_WIDTH,
    MAP_HEIGHT,
    GRAVITY
};
use lib_game::GResult;
use lib_game::shape::{Circle, Rect, Shape, ShapeType};
use lib_game::tile::TileInfo;
use lib_game::vector::Vec2;
use lib_game::collision;
use lib_game::Direction;

use lib_game::sprite::SpriteSheet;


use macroquad::prelude::{
    Texture2D,
    load_texture,
    WHITE,
    DrawTextureParams,
    Rect as r,
    draw_texture_ex,
    vec2
};

use crate::map::TileMap;


pub trait Entity {

    fn get_shape_type(&self) -> ShapeType;
    fn draw(&self);
    fn get_normalized_position(&self) -> (f32,f32);


}

pub trait SpriteUser {

    fn reload_sprite(&mut self);

}



pub struct Tile {

    texture:    Texture2D,
    shape:      Rect,
    draw_info:  DrawTextureParams,
    wall:       bool
}

impl Tile {
    
    pub async fn new(info:&TileInfo,x:f32,y:f32) -> GResult<Self> {
        
        let tex = load_texture(info.get_src()).await?;

        Ok(Self {
            texture: tex,
            shape: Rect::new(x,y,TILE_SIZE,TILE_SIZE),
            draw_info: DrawTextureParams {
                dest_size: Some(vec2(NORM_WIDTH_TILE_SIZE,NORM_HEIGHT_TILE_SIZE)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: true,
                pivot: None
            },
            wall: info.is_a_wall()
        })

    }

    pub fn get_rect(&self) -> &Rect { &self.shape }

    pub fn is_a_wall(&self) -> bool { self.wall }

}


impl Entity for Tile {

    fn get_shape_type(&self) -> ShapeType { ShapeType::Rect }

    fn draw(&self) {

        let (x,y) = self.get_normalized_position();

        draw_texture_ex(
            self.texture,
            x,
            y,
            WHITE,
            self.draw_info.clone()
        );


    }

    fn get_normalized_position(&self) -> (f32,f32) {

        (
            (self.shape.get_x()) / (WIDTH/2.0) - 1.0,
            (self.shape.get_y()) / (HEIGHT/2.0) - 1.0
        )

    }


}


// Enemy =============================

const GVELOCITY: f32 = 1.0;


pub struct Goomba {
    spos:       Vec2,
    sdirection: Direction,
    sfreeze:    bool,
    shape:      Rect,
    texture:    Texture2D,
    draw_info:  DrawTextureParams,
    spritesheet: SpriteSheet,
    velocity:   Vec2,
    direction:  Direction,
    walk_frame_cnt: u8,
    die_frame_cnt: u8,
    dead:           bool,
    disappear:      bool,
    freeze:         bool,
    fall_ctn:      u8,
}

impl Goomba {

    pub async fn new(sx:f32,sy:f32,direction:Direction,freeze:bool) -> GResult<Self> {

        let p = format!("{}/goomba.png",ASSETS_DIR);

        let tex = load_texture(&p).await?;
        let mut spritesheet = SpriteSheet::new(&p,TILE_SIZE,TILE_SIZE);

        spritesheet.add_sprite("walk",2,0.0,0.0,1.0)?;
        spritesheet.add_sprite("dead",1,34.0,0.0,0.0)?;

        // default frame
        let frame = spritesheet.get_current_frame()?;

        let dinfo = DrawTextureParams {
            dest_size: Some(vec2(NORM_WIDTH_TILE_SIZE,NORM_HEIGHT_TILE_SIZE)),
            source: Some(r::new(frame.position.x,frame.position.y,frame.size.x,frame.size.y)),
            rotation: 0.0,
            flip_x: false,
            flip_y: true,
            pivot: None
        };

        Ok(Self {
            spos: Vec2::new(sx,sy),
            sdirection: direction,
            sfreeze: freeze,
            shape: Rect::new(sx,sy,TILE_SIZE,TILE_SIZE),
            texture: tex,
            draw_info: dinfo,
            spritesheet,
            velocity: Vec2::new(0.0,0.0),
            direction,
            walk_frame_cnt: 0,
            die_frame_cnt: 0,
            dead: false,
            disappear: false,
            freeze,
            fall_ctn: 0,

        })


    }

    pub fn is_dying(&self) -> bool { self.dead }
    pub fn is_disappear(&self) -> bool { self.disappear }
    pub fn defreeze(&mut self) { self.freeze = false; }
    pub fn die(&mut self) {

        if !self.dead {
            self.dead = true;
            self.spritesheet.change_current("dead").unwrap();
            self.reload_sprite();
        }


    }

    pub fn reset(&mut self) {
        self.shape.pos = self.spos;
        self.direction = self.sdirection;
        self.freeze = self.sfreeze;

        self.disappear = false;
        self.dead = false;
        self.die_frame_cnt = 0;


        self.spritesheet.change_current("walk").unwrap();
        self.reload_sprite();

    }




    pub fn update(&mut self, tiles:&TileMap) {



        if !self.dead && !self.freeze  {

            match self.direction {
                Direction::Left => self.velocity.x = -GVELOCITY,
                Direction::Right => self.velocity.x = GVELOCITY,
                _ => {}
            }

            let mut gravity_velocity = (self.fall_ctn as f32 / 60.0 ) * -5.0;
            if gravity_velocity < -5.0 {
                gravity_velocity = -5.0;
            }

            self.fall_ctn += 1;
            self.velocity.y += gravity_velocity;



            for tile in tiles.iter() {

                if tile.is_a_wall() && collision::rect_vs_rect_vertically(
                    &self.shape,
                    tile.get_rect(),
                    self.velocity.y)  {

                    self.velocity.y = 0.0;
                    self.shape.pos.y = tile.get_rect().get_y() + TILE_SIZE;
                    self.fall_ctn = 0;
                }

                if tile.is_a_wall() && collision::rect_vs_rect_horizontally(
                    &self.shape,
                    tile.get_rect(),
                    self.velocity.x) {

                    if self.velocity.x > 0.0 {

                        self.direction = Direction::Left;
                        self.velocity.x = -GVELOCITY;

                    } else {
                        self.direction = Direction::Right;
                        self.velocity.x = GVELOCITY;
                    }

                }




            }


            if self.direction == Direction::Left && self.shape.pos.x + self.velocity.x <= 0.0 {
                self.direction = Direction::Right;
                self.velocity.x = GVELOCITY;
            }


            self.shape.pos += self.velocity;

            if self.shape.get_y() <= 0.0 {
                self.dead = true;
                self.disappear = true;
            }

        }


        self.update_sprite();


    }



    fn update_sprite(&mut self) {
        if self.dead {

            if self.die_frame_cnt > 30 {
                self.disappear = true;
            } else {
                self.die_frame_cnt += 1;
            }

        } else if self.walk_frame_cnt > 10 {
            self.spritesheet.increment_current_sprite().unwrap();
            self.reload_sprite();

            self.walk_frame_cnt = 0;

        } else {
            self.walk_frame_cnt += 1;
        }



    }

    pub fn get_rect(&self) -> &Rect { &self.shape }



}

impl Entity for Goomba {
    fn get_shape_type(&self) -> ShapeType { ShapeType::Rect }
    fn draw(&self) {

        let (nx,ny) = self.get_normalized_position();

        draw_texture_ex(
            self.texture,
            nx,
            ny,
            WHITE,
            self.draw_info.clone()
        );

    }
    fn get_normalized_position(&self) -> (f32, f32) {
        (
            (self.shape.get_x()) / (WIDTH/2.0) - 1.0,
            (self.shape.get_y()) / (HEIGHT/2.0) - 1.0
        )
    }

}

impl SpriteUser for Goomba {
    fn reload_sprite(&mut self) {

        let frame = self.spritesheet.get_current_frame().unwrap();
        self.draw_info.flip_x = self.spritesheet.should_flip();
        self.draw_info.source = Some(r::new(
            frame.position.x,
            frame.position.y,
            frame.size.x,
            frame.size.y
        ));

    }

}


pub struct MysteryBlocks {

    shape:          Rect,
    texture:        Texture2D,
    draw_info:      DrawTextureParams,
    spritesheet:    SpriteSheet,
    collected:      bool,
    sprite_update:  u8
}

impl MysteryBlocks {

    pub async fn new(x:f32,y:f32) -> GResult<Self> {

        let p = format!("{}/mblock.png",ASSETS_DIR);

        let tex = load_texture(&p).await?;

        let mut spritesheet = SpriteSheet::new(&p,TILE_SIZE,TILE_SIZE);

        spritesheet.add_sprite("normal",3,0.0,0.0,0.0)?;
        spritesheet.add_sprite("collected",1,48.0,0.0,0.0)?;

        // default frame
        let frame = spritesheet.get_current_frame()?;

        let dinfo = DrawTextureParams {
            dest_size: Some(vec2(NORM_WIDTH_TILE_SIZE,NORM_HEIGHT_TILE_SIZE)),
            source: Some(r::new(frame.position.x,frame.position.y,frame.size.x,frame.size.y)),
            rotation: 0.0,
            flip_x: false,
            flip_y: true,
            pivot: None
        };

        Ok(Self{
            shape: Rect::new(x,y,TILE_SIZE,TILE_SIZE),
            texture: tex,
            spritesheet,
            draw_info: dinfo,
            collected: false,
            sprite_update: 0

        })

    }

    pub fn update(&mut self) {


        self.update_sprite();
    }

    pub fn reset(&mut self) {

        if self.collected {
            self.collected = false;
            self.spritesheet.change_current("normal").unwrap();
            self.reload_sprite();
        }


    }


    pub fn get_rect(&self) -> &Rect { &self.shape }

    pub fn is_collected(&self) -> bool { self.collected }

    pub fn collect(&mut self) {
        if !self.collected {
            self.collected = true;

            self.spritesheet.change_current("collected").unwrap();
            self.reload_sprite();

        }

    }

    fn update_sprite(&mut self) {

        if !self.collected {

            if self.sprite_update > 15 {
                self.spritesheet.increment_current_sprite().unwrap();

                self.reload_sprite();

                self.sprite_update = 0;
            } else {
                self.sprite_update += 1;
            }


        }


    }

}

impl Entity for MysteryBlocks {

    fn get_shape_type(&self) -> ShapeType { ShapeType::Rect }
    fn draw(&self) {
        let (nx,ny) = self.get_normalized_position();

        draw_texture_ex(
            self.texture,
            nx,
            ny,
            WHITE,
            self.draw_info.clone()
        );

    }
    fn get_normalized_position(&self) -> (f32, f32) {
        (
            (self.shape.get_x()) / (WIDTH/2.0) - 1.0,
            (self.shape.get_y()) / (HEIGHT/2.0) - 1.0
        )
    }

}

impl SpriteUser for MysteryBlocks {
    fn reload_sprite(&mut self) {
        let frame = self.spritesheet.get_current_frame().unwrap();
        self.draw_info.flip_x = self.spritesheet.should_flip();
        self.draw_info.source = Some(r::new(
            frame.position.x,
            frame.position.y,
            frame.size.x,
            frame.size.y
        ));
    }

}
