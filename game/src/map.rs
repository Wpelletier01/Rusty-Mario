
use crate::declaration::{ASSETS_DIR, TILE_DIR, TILE_SIZE};
use crate::entity::{Entity, Tile};

use lib_game::GResult;
use lib_game::loader::{Level,load_level,load_tileset,Tileset};


pub type TileMap = Vec<Tile>;

/// initialise all the tile with the buffer tile id
async fn load_tilemap(tileset:&Tileset,lvl_map:&Level) -> GResult<TileMap> {

    let mut tilemap:TileMap = Vec::new();

    //let mut x:f32 = 14.0 * TILE_SIZE;
    let mut y:f32 = 14.0 * TILE_SIZE;

    for row in lvl_map.iter() {
        for (col_ctn,col) in row.iter().enumerate() {

            if col != &-1 {
                let mut found = false;

                for ti in tileset.iter() {

                    if &ti.get_id() == col {


                        let px = TILE_SIZE * col_ctn as f32;

                        let tile = Tile::new(ti, px, y).await?;
                        tilemap.push(tile);

                        found = true;
                        break;

                    }

                }

                if !found {
                    bail!("no tile with id '{}' exist",col);
                }


            }
        }
        y -= TILE_SIZE;
    }

    Ok(tilemap)

}



pub struct Map {

    level:          Level,
    tilemap:        TileMap,

}

impl Map {

    pub fn init() -> GResult<Self> {

        let plvl = format!("{}/lvl-1-1.csv",ASSETS_DIR);
        let level = load_level(&plvl)?;
        let tilemap = TileMap::new();


        Ok(Self {
            tilemap,
            level,
        })
    }

    pub async fn load(&mut self) -> GResult<()> {


        let tileset = load_tileset(TILE_DIR)?;


        self.tilemap = load_tilemap(&tileset,&self.level).await?;

        Ok(())

    }

    pub fn render(&self) {

        for tile in self.tilemap.iter() {

            tile.draw();

        }

    }

    pub fn get_tiles(&self) -> &TileMap { &self.tilemap }




}








