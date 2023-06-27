
use crate::entity::{
    Entity,
    Dynamic,
    Goomba,
    MysteryBlocks,
    SpriteUser,
    get_normalized_position
};

use crate::declaration::{
    TILE_SIZE,
    NORM_HEIGHT_TILE_SIZE,
    NORM_WIDTH_TILE_SIZE,
    GRAVITY,
    ASSETS_DIR

};
use crate::map::TileMap;

use lib_game::shape::{Rect, Shape};
use lib_game::vector::Vec2;
use lib_game::sprite::SpriteSheet;
use lib_game::GResult;
use lib_game::collision;

use macroquad::prelude::{
    DrawTextureParams,
    Texture2D,
    load_texture,
    draw_texture_ex,
    vec2 as v2,
    Rect as r,
    WHITE
};



#[derive(Debug,PartialEq)]
pub enum PStatus {
    Walk,
    Idle,
    Jump,
    Dead
}

pub struct Player {
    spos:               Vec2,
    shape:              Rect,
    velocity:           Vec2,
    can_jump:           bool,
    jumping:            bool,
    jump_ctn:           i32,
    fall_ctn:           i32,
    spritesheet_src:    Texture2D,
    draw_info:          DrawTextureParams,
    spritesheet:        SpriteSheet,
    walk_frame_ctn:     i8,
    status:             PStatus,
    dead_velocity:      f32
}

impl Player {

    pub async fn new(start_pos:Vec2) -> GResult<Self> {

        let p = format!("{}/small-mario.png",ASSETS_DIR);

        let spritesheet_src = load_texture(&p).await?;
        let mut spritesheet = SpriteSheet::new(TILE_SIZE,TILE_SIZE);

        // sprite
        spritesheet.add_sprite("idle",1,77.0,0.0,0.0)?; // default
        spritesheet.add_sprite("walk",3,0.0,0.0,0.0)?;
        spritesheet.add_sprite("jump",1,48.0,0.0,0.0)?;
        spritesheet.add_sprite("dead",1,105.0,0.0,0.0)?;

        // default frame
        let frame = spritesheet.get_current_frame()?;



        let dinfo = DrawTextureParams {
            dest_size: Some(v2(NORM_WIDTH_TILE_SIZE,NORM_HEIGHT_TILE_SIZE)),
            source: Some(r::new(frame.position.x,frame.position.y,frame.size.x,frame.size.y)),
            rotation: 0.0,
            flip_x: false,
            flip_y: true,
            pivot: None
        };



        Ok(Self {
            spos:   start_pos,
            shape: Rect::new(start_pos.x,start_pos.y,TILE_SIZE,TILE_SIZE),
            velocity: Vec2::new(0.0,0.0),
            can_jump: true,
            jumping:  false,
            fall_ctn: 0,
            spritesheet_src,
            spritesheet,
            draw_info:dinfo,
            walk_frame_ctn: 0,
            status: PStatus::Walk,
            jump_ctn: 10,
            dead_velocity: 0.0
        })

    }

    pub fn reset(&mut self) {
        self.clear_velocity();
        self.shape.pos = self.spos;
        self.change_sprite_status(PStatus::Walk);
    }

    pub fn jump(&mut self) {

        if self.can_jump {
            self.jumping = true;
            self.can_jump = false;
        }

    }
    pub fn flip_spritesheet(&mut self,y:bool) {
        if self.spritesheet.should_flip() && !y || !self.spritesheet.should_flip() && y {
            self.spritesheet.flip_sprite();
        }

    }

    pub fn get_x(&self) -> f32 { self.shape.pos.x }
    pub fn get_y(&self) -> f32 { self.shape.pos.y }
    pub fn get_rect(&self) -> &Rect { &self.shape }
    pub fn get_xvelocity(&self) -> f32 { self.velocity.x }
    pub fn get_yvelocity(&self) -> f32 { self.velocity.y }
    pub fn get_height(&self) -> f32 { self.shape.get_height() }

    pub fn set_xvelocity(&mut self,x:f32) { self.velocity.x += x; }

    pub fn is_dying(&self) -> bool { self.status == PStatus::Dead }

    pub fn clear_velocity(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }


    pub fn update_sprite(&mut self) -> GResult<()> {

        match self.status {

            PStatus::Idle => {

                let frame = self.spritesheet.get_current_frame().unwrap();

                self.draw_info.source = Some(r::new(
                    frame.position.x,
                    frame.position.y,
                    frame.size.x,
                    frame.size.y
                ));

            },
            PStatus::Walk => {

                if self.walk_frame_ctn > 3 {

                    self.spritesheet.increment_current_sprite().unwrap();

                    let frame = self.spritesheet.get_current_frame().unwrap();
                    self.draw_info.flip_x = self.spritesheet.should_flip();

                    self.draw_info.source = Some(r::new(
                        frame.position.x,
                        frame.position.y,
                        frame.size.x,
                        frame.size.y
                    ));

                    self.walk_frame_ctn = 0;

                } else {
                    self.walk_frame_ctn += 1;
                }


            },
            PStatus::Jump => {

                let frame = self.spritesheet.get_current_frame().unwrap();
                self.draw_info.flip_x = self.spritesheet.should_flip();
                self.draw_info.source = Some(r::new(
                    frame.position.x,
                    frame.position.y,
                    frame.size.x,
                    frame.size.y
                ));

            },
            PStatus::Dead => {

                if self.dead_velocity > -5.0 {

                    self.shape.pos.y += self.dead_velocity;
                    self.dead_velocity -= 0.5;

                } else {
                    self.shape.pos.y -= 5.0;
                }


            }


        }



        Ok(())

    }


    fn die(&mut self) {

        self.clear_velocity();
        self.change_sprite_status(PStatus::Dead);
        self.dead_velocity = 10.0;

    }


    fn check_collision_w_mblocks(&mut self, mblocks: &mut [MysteryBlocks]) {

        let mut tmp_rect = self.shape;
        tmp_rect.pos += self.velocity;


        for block in mblocks.iter_mut() {

            if collision::rect_vs_rect(&tmp_rect,block.get_rect()) {

                if collision::rect_vs_rect_horizontally(
                    &self.shape,
                    block.get_rect(),
                    self.get_xvelocity()) {

                    if self.velocity.x > 0.0 {

                        self.shape.pos.x =block.get_rect().get_x() - self.shape.get_width();
                        self.velocity.x = 0.0;

                    } else {

                        self.shape.pos.x = block.get_rect().get_max_x();
                        self.velocity.x = 0.0;

                    }

                }

                if collision::rect_vs_rect_vertically(
                    &self.shape,
                    block.get_rect(),
                    self.get_yvelocity()) {

                    if self.get_yvelocity() < 0.0 {

                        self.fall_ctn = 0;
                        self.jumping = false;
                        self.jump_ctn = 10;
                        self.shape.pos.y = block.get_rect().get_y() + self.get_height();
                        self.velocity.y = 0.0;

                    } else {
                        block.collect();
                        self.shape.pos.y = block.get_rect().get_y() - TILE_SIZE;
                        self.velocity.y = 0.0;
                        self.fall_ctn = 0;
                        self.jumping = false;


                    }

                }


            }


        }


    }

    fn check_collision_w_enemy(&mut self, goombas: &mut [Goomba]) {


        for goomba in goombas.iter_mut() {

            let mut tmp_rect = self.shape;
            tmp_rect.pos += self.velocity;

            if !goomba.is_dying() && collision::rect_vs_rect(&tmp_rect,goomba.get_rect()) {
                if collision::rect_vs_rect_vertically(
                    self.get_rect(),
                    goomba.get_rect(),
                    self.get_yvelocity()) {

                    if self.velocity.y < 0.0 {
                        goomba.die();
                    } else {
                        self.die();
                        break;
                    }

                } else {
                    self.die();
                    break;
                }

            }

        }


    }

    fn check_collision_w_static(&mut self,tiles:&TileMap) {

        let mut tmp_rect = *self.get_rect();
        tmp_rect.pos += self.velocity;

        for tile in tiles.iter() {

            if tile.is_a_wall() && collision::rect_vs_rect(&tmp_rect, tile.get_rect()) {

                if collision::rect_vs_rect_horizontally(
                    self.get_rect(),
                    tile.get_rect(),
                    self.get_xvelocity()) {

                    if self.get_xvelocity() > 0.0 {

                        self.shape.pos.x = tile.get_rect().get_x() - self.get_rect().get_width();
                        self.velocity.x = 0.0;

                    } else  {

                        self.shape.pos.x = tile.get_rect().get_max_x();
                        self.velocity.x = 0.0;

                    }



                }

                if collision::rect_vs_rect_vertically(
                    self.get_rect(),
                    tile.get_rect(),
                    self.get_yvelocity()) {

                    if self.get_yvelocity() < 0.0 {

                        self.fall_ctn = 0;
                        self.jumping = false;
                        self.jump_ctn = 10;
                        self.shape.pos.y = tile.get_rect().get_y() + self.get_height();
                        self.velocity.y = 0.0;

                    } else {
                        self.shape.pos.y = tile.get_rect().get_y() - 16.0;
                        self.velocity.y = 0.0;
                        self.fall_ctn = 0;
                        self.jumping = false;

                    }

                }



            }



        }



    }


    fn change_sprite_status(&mut self,status:PStatus) {

        if self.status != status {
            self.status = status;

            match self.status {

                PStatus::Walk => {
                    self.spritesheet.change_current("walk").unwrap();
                },
                PStatus::Idle => {
                    self.spritesheet.change_current("idle").unwrap();
                },
                PStatus::Jump => {
                    self.spritesheet.change_current("jump").unwrap();
                },
                PStatus::Dead => {
                    self.spritesheet.change_current("dead").unwrap();

                }

            }

            self.reload_sprite();

        }
    }


}


impl Entity for Player {

    fn draw(&self) {

        let (nx,ny) = get_normalized_position(&self.shape.pos);

        draw_texture_ex(
            self.spritesheet_src,
            nx,
            ny,
            WHITE,
            self.draw_info.clone()
        );


    }


}

impl Dynamic<(&TileMap,&mut [MysteryBlocks],&mut [Goomba])> for Player {
    fn reset(&mut self) {
        self.clear_velocity();
        self.shape.pos = self.spos;
        self.change_sprite_status(PStatus::Walk);

    }

    fn update(&mut self, entity: (&TileMap, &mut [MysteryBlocks], &mut [Goomba])) {

        let (tiles,mblocks,goombas) = entity;

        if self.status != PStatus::Dead {

            if self.jumping {

                if self.jump_ctn >= -10 {
                    self.velocity.y += (self.jump_ctn as f32 + 10.0) * 0.5;
                    self.jump_ctn -= 1;

                } else {
                    self.jumping = false;
                    self.jump_ctn = 0;
                }

            }

            let mut gravity_velocity = (self.fall_ctn as f32 / 60.0 ) * -GRAVITY;
            if gravity_velocity < -5.0 {
                gravity_velocity = -5.0;
            }

            self.fall_ctn += 1;
            self.velocity.y += gravity_velocity;

            if self.velocity.x + self.shape.pos.x < 0.0 {
                self.velocity.x = 0.0;
            }

            self.check_collision_w_mblocks(mblocks);
            self.check_collision_w_static(tiles);
            self.check_collision_w_enemy(goombas);


            if self.velocity.y == 0.0 {
                self.can_jump = true;
            } else if self.velocity.y < 0.0 {
                self.can_jump = false;
            }

            self.shape.pos += self.velocity;

            if self.status != PStatus::Dead {
                if self.velocity.x == 0.0 && self.velocity.y == 0.0 {
                    self.change_sprite_status(PStatus::Idle)
                } else if self.velocity.x != 0.0 && self.velocity.y == 0.0 {
                    self.change_sprite_status(PStatus::Walk);
                } else if self.velocity.y != 0.0 {
                    self.change_sprite_status(PStatus::Jump);
                }

            }


        }

        self.update_sprite().unwrap();



    }

}


impl SpriteUser for Player {
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