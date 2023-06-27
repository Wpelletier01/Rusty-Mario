
use lib_game::GResult;
use lib_game::vector::Vec2;
use lib_game::Direction;


use crate::entity::{Entity,Goomba,MysteryBlocks};
use crate::player::Player;
use crate::map::Map;

use macroquad::prelude::*;

use crate::declaration::{ASSETS_DIR, HEIGHT, WIDTH};

const PLAYER_VELOCITY: f32 = 2.0;


#[derive(PartialEq)]
pub enum GameStatus {

    GamePlay,
    Pause,
    Win

}


pub struct State {
    camera:             Camera2D,
    game_status:        GameStatus,
    player:             Player,
    map:                Map,
    scrolling_offset:   f32,
    goombas:            Vec<Goomba>,
    mystery_blocks:     Vec<MysteryBlocks>,
    win_message:        Texture2D
}

impl State {

    pub async fn init() -> GResult<Self> {

        let camera = Camera2D {
            zoom: vec2(2.4,2.4),
            offset: vec2(1.4,1.25),
            ..Default::default()
        };

        let mut map = Map::init()?;
        map.load().await?;

        let player = Player::new(Vec2{ x: 80.0, y: 48.0 }).await?;

        let mystery_blocks = vec![
            MysteryBlocks::new(256.0,96.0).await?,
            MysteryBlocks::new(336.0,96.0).await?,
            MysteryBlocks::new(352.0,160.0).await?,
            MysteryBlocks::new(368.0,96.0).await?,
            MysteryBlocks::new(1024.0,112.0).await?,
            MysteryBlocks::new(1280.0,96.0).await?,
            MysteryBlocks::new(1536.0,160.0).await?,
            MysteryBlocks::new(1536.0,96.0).await?,
            MysteryBlocks::new(1648.0,96.0).await?,
            MysteryBlocks::new(1728.0,96.0).await?,
            MysteryBlocks::new(1776.0,160.0).await?,
            MysteryBlocks::new(1776.0,96.0).await?,
            MysteryBlocks::new(1824.0,96.0).await?,
            MysteryBlocks::new(2096.0,160.0).await?,
            MysteryBlocks::new(2112.0,160.0).await?,
            MysteryBlocks::new(2752.0,96.0).await?,

        ];

        let goombas = vec![

            Goomba::new(352.0,48.0,Direction::Right,false).await?,
            Goomba::new(640.0,48.0,Direction::Left, false).await?,
            Goomba::new(848.0,48.0,Direction::Left,false).await?,
            Goomba::new(880.0,48.0,Direction::Left,false).await?,
            // start on the platform
            Goomba::new(1312.0,176.0,Direction::Left,true).await?,
            Goomba::new(1344.0,176.0,Direction::Left,true).await?,
            // after the platform
            Goomba::new(1552.0,48.0,Direction::Left,true).await?,
            Goomba::new(1584.0,48.0,Direction::Left,true).await?,

            Goomba::new(2085.0,48.0,Direction::Right,true).await?,
            Goomba::new(2064.0,48.0,Direction::Right,true).await?,
            Goomba::new(2040.0,48.0,Direction::Left,true).await?,
            Goomba::new(2016.0,48.0,Direction::Left,true).await?,
            Goomba::new(2752.0,48.0,Direction::Left,false).await?,
            Goomba::new(2768.0,48.0,Direction::Right,false).await?

        ];

        // message when you finish the level
        let p_win_msg = format!("{}/win-message.png",ASSETS_DIR);
        let win_message = load_texture(&p_win_msg).await?;

        set_camera(&camera);

        Ok(Self {
            camera,
            game_status: GameStatus::GamePlay,
            player,
            map,
            goombas,
            scrolling_offset: 0.0,
            mystery_blocks,
            win_message
        })

    }



    pub fn handle_input(&mut self) {

        self.player.clear_velocity();


        if self.game_status == GameStatus::GamePlay {

            if is_key_down(KeyCode::A)  {
                self.player.set_xvelocity(-PLAYER_VELOCITY);
                self.player.flip_spritesheet(true);
            }

            if is_key_down(KeyCode::D) {
                self.player.set_xvelocity(PLAYER_VELOCITY);
                self.player.flip_spritesheet(false);
            }

            if is_key_pressed(KeyCode::Space)  {
                self.player.jump();
            }

        }

        if is_key_pressed(KeyCode::U) {
            self.reset();
        }

    }

    fn reset(&mut self) {

        self.player.reset();

        for goomba in self.goombas.iter_mut() {
            goomba.reset();
        }

        for block in self.mystery_blocks.iter_mut() {
            block.reset();
        }


        self.game_status = GameStatus::GamePlay;

        self.camera.offset = vec2(1.4,1.25);
        set_camera(&self.camera);

    }

    pub fn update(&mut self) {

        if self.game_status == GameStatus::GamePlay {

            self.player.update(self.map.get_tiles(),&mut self.mystery_blocks,&mut self.goombas);


            if self.player.get_y() <= 0.0 {
                self.reset();

            }


            if self.player.get_rect().get_max_x() >= 200.0
                && self.player.get_xvelocity() != 0.0
                && !self.player.is_dying()
                && self.player.get_rect().get_max_x() <= 3164.0 {


                self.camera.offset += vec2(-self.player.get_xvelocity()/(WIDTH/2.0)*2.4,0.0);
                set_camera(&self.camera);


            }


            for goomba in self.goombas.iter_mut() {
                if !goomba.is_disappear() {
                    goomba.update(self.map.get_tiles());
                }

            }

            for mbox in self.mystery_blocks.iter_mut() {
                mbox.update();
            }


            if self.player.get_x() >= 1080.0 {
                self.goombas[4].defreeze();
                self.goombas[5].defreeze();
            }

            if self.player.get_x() >= 1328.0 {
                self.goombas[6].defreeze();
                self.goombas[7].defreeze();
            }

            if self.player.get_x() >= 1800.0 {
                self.goombas[8].defreeze();
                self.goombas[9].defreeze();
                self.goombas[10].defreeze();
                self.goombas[11].defreeze();
            }

            if self.player.get_x() >= 3200.0 {
                self.game_status = GameStatus::Win;
            }
        }


    }

    pub fn render(&mut self) {
        match self.game_status {

            GameStatus::GamePlay => {
                self.map.render();
                self.player.draw();

                for goomba in self.goombas.iter() {
                    if !goomba.is_disappear() {
                        goomba.draw();
                    }

                }

                for mbox in self.mystery_blocks.iter() {
                    mbox.draw()
                }


            },
            GameStatus::Win => {
                self.map.render();
                self.player.draw();


                for goomba in self.goombas.iter() {
                    if !goomba.is_disappear() {
                        goomba.draw();
                    }

                }

                for mbox in self.mystery_blocks.iter() {
                    mbox.draw()
                }


                draw_texture_ex(
                    self.win_message,
                    (2968.0 / (WIDTH/2.0)) - 1.0,
                    (90.0 / (HEIGHT/2.0)) - 1.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            400.0 / (WIDTH/2.0),
                            200.0 / (HEIGHT/2.0)
                        )),
                        source: None,
                        pivot: None,
                        flip_y: true,
                        flip_x: false,
                        rotation: 0.0
                    }
                );


            }

            _ => {}

        }


    }


}