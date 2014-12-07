use std::io;
use std::num::FromPrimitive;
use serialize::{json, Decodable};

use cgmath::{Transform, AffineMatrix3};
use cgmath::{Point3, Vector3};
use gfx;
use gfx::{Device, DeviceHelper, ToSlice};

use errors::{Res, GameError};
use meshutils::CubeMaker;
use texture::{Texture, TextureSlice};

static TILE_SIZE : f32 = 1.0;


#[deriving(PartialEq, Eq, FromPrimitive, Copy, Show)]
pub enum Tile {
    /* environment */
        /// out of bounds ground
        Oob = 1,
        /// ground tanks can drive on
        Ground = 9,
        /// ground with a hole in it
        Hole = 17, 
        /// water
        Water = 5,
        /// swamp
        Swamp = 6,

    /* obstacles */
        /// wall height 1
        Wall1 = 2,
        /// wall height 2 
        Wall2 = 10,
        /// wall height 3
        Wall3 = 18,
        /// wall height 4
        Wall4 = 26,
        /// wall height 5
        Wall5 = 34,
        /// indestructible box height 1
        Box1 = 3,
        /// indestructible box height 2
        Box2 = 11,
        /// indestructible box height 3
        Box3 = 19,
        /// indestructible box height 4
        Box4 = 27,
        /// indestructible box height 5
        Box5 = 35,

    /* spawns */
        Player1Spawn = 13,
        Player2Spawn = 21,
        Player3Spawn = 29,
        Player4Spawn = 37,

    /* enemy spawns */
        /// basic enemy (does not move)
        StationaryEnemy = 8,
        /// basic enemy
        BasicEnemy = 16,
        /// faster enemy type
        FastEnemy = 24,
        /// enemy tank with heat seeker
        HeatSeekerEnemy = 40,
        /// enemy tank with fast rockets
        RocketEnemy = 32,

    /* object spawns */
        /// mine
        Mine = 25,
        /// destructible crate
        Crate = 4,
}

#[deriving(Copy, Show)]
pub enum EnemyType {
    Stationary,
    Basic,
    Fast,
    HeatSeeker,
    Rocket,
}

#[deriving(Copy, Show)]
pub enum ObjectType {
    Mine,
    Crate,
}

#[deriving(Copy, Show)]
pub enum Spawn {
    Player(u8),
    Enemy(EnemyType),
    Object(ObjectType),
}

impl Tile {

    /// construct a tile type from a gid
    pub fn from_gid(gid: u8) -> Option<Tile> {
        FromPrimitive::from_u8(gid)
    }

    /// true if the tiel is out of bounds
    pub fn is_oob(&self) -> bool {
        match *self {
            Tile::Oob => true,
            _ => false,
        }
    }

    /// true if the tile is a ground tile.
    ///
    /// Tanks can stand on these tiles.
    pub fn is_ground(&self) -> bool {
        // if things spawn on it, they are on a ground tile.
        if self.is_spawn() {
            return true;
        }

        match *self {
            // out of bounds is also a ground tile just that
            // entering it should not be possible.
            Tile::Oob => true,
            // a ground tile is ground.
            Tile::Ground => true,
            _ => false,
        }
    }

    /// true if the tile is a fluid
    pub fn is_fluid(&self) -> bool {
        match *self {
            Tile::Water | Tile::Swamp => true,
            _ => false,
        }
    }

    /// true if the tile is an obstacle
    pub fn is_obstacle(&self) -> bool {
        self.height() > 0
    }

    /// true if the tile is a spawn
    pub fn is_spawn(&self) -> bool {
        self.get_spawn().is_some()
    }

    /// returns the height of the tile
    pub fn height(&self) -> u8 {
        match *self {
            Tile::Wall1 | Tile::Box1 => 1,
            Tile::Wall2 | Tile::Box2 => 2,
            Tile::Wall3 | Tile::Box3 => 3,
            Tile::Wall4 | Tile::Box4 => 4,
            Tile::Wall5 | Tile::Box5 => 5,
            _ => 0,
        }
    }

    /// false if bullets can pass this tile
    pub fn blocks_bullet(&self) -> bool {
        self.height() > 0
    }

    /// resolves what the tile spawns
    pub fn get_spawn(&self) -> Option<Spawn> {
        match *self {
            Tile::Player1Spawn => Some(Spawn::Player(0)),
            Tile::Player2Spawn => Some(Spawn::Player(1)),
            Tile::Player3Spawn => Some(Spawn::Player(2)),
            Tile::Player4Spawn => Some(Spawn::Player(3)),
            Tile::StationaryEnemy => Some(Spawn::Enemy(EnemyType::Stationary)),
            Tile::BasicEnemy => Some(Spawn::Enemy(EnemyType::Basic)),
            Tile::FastEnemy => Some(Spawn::Enemy(EnemyType::Fast)),
            Tile::HeatSeekerEnemy => Some(Spawn::Enemy(EnemyType::HeatSeeker)),
            Tile::RocketEnemy => Some(Spawn::Enemy(EnemyType::Rocket)),
            Tile::Mine => Some(Spawn::Object(ObjectType::Mine)),
            Tile::Crate => Some(Spawn::Object(ObjectType::Crate)),
            _ => None,
        }
    }

    /// get the gid
    pub fn get_gid(&self) -> u8 {
        *self as u8
    }

    /// Return a texture slice for this tile from an atlas.  The atlas
    /// needs to be 8x8.
    pub fn get_texture_slice<'a>(&self, tex: &'a Texture) -> TextureSlice<'a> {
        let idx = (self.get_gid() - 1) as u16;
        let w = tex.width() / 8;
        let h = tex.height() / 8;
        let x = (idx % 8) * w;
        let y = (idx / 8) * h;
        tex.slice(x, y, w, h)
    }

    /// get detail debug info
    pub fn debug(&self) -> String {
        format!("<{} is_oob={}, is_ground={}, height={} spawn={}>",
            *self,
            self.is_oob(),
            self.is_ground(),
            self.height(),
            self.get_spawn(),
        )
    }
}

pub struct Map {
    width: u16,
    height: u16,
    tiles: Vec<Tile>,
}

#[deriving(Decodable)]
struct MapLayerData {
    data: Vec<u8>,
}

#[deriving(Decodable)]
struct MapData {
    width: u16,
    height: u16,
    layers: Vec<MapLayerData>,
}

impl Map {

    pub fn open(path: &Path) -> Res<Map> {
        let mut file = try!(io::File::open(path));
        let json = try!(json::from_reader(&mut file));
        let mut decoder = json::Decoder::new(json);
        let md : MapData = try!(Decodable::decode(&mut decoder));
        let tiles : Vec<Tile> = md.layers[0].data.iter().map(
                |&x| Tile::from_gid(x).unwrap_or(Tile::Oob)).collect();

        if (md.width * md.height) as uint != tiles.len() {
            Err(GameError::InvalidMap("Invalid dimensions"))
        } else {
            Ok(Map {
                width: md.width,
                height: md.height,
                tiles: tiles,
            })
        }
    }

    #[inline(always)]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[inline(always)]
    pub fn get_tile(&self, x: u16, y: u16) -> Tile {
        self.tiles[(y * self.width + x) as uint]
    }

    pub fn get_camera_view(&self) -> AffineMatrix3<f32> {
        let left = (self.width() as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0;
        let top = (self.height() as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0;
        Transform::look_at(
            &Point3::new(left, left * 2.0, top - 2.0),
            &Point3::new(left, 0.0, top),
            &Vector3::unit_z(),
        )
    }

    pub fn create_mesh(&self, device: &mut gfx::GlDevice,
                       texture_map: &Texture) -> MapMesh {
        let mut builder = MapMeshBuilder::new(
            device, texture_map, self, TILE_SIZE);
        builder.build_mesh();
        builder.finish()
    }
}

pub struct MapMesh<'a> {
    map: &'a Map,
    mesh: gfx::Mesh,
    slice: gfx::Slice,
}

impl<'a> MapMesh<'a> {

    pub fn get_mesh(&self) -> &gfx::Mesh {
        &self.mesh
    }

    pub fn get_slice(&self) -> gfx::Slice {
        self.slice
    }

    pub fn get_map(&self) -> &Map {
        self.map
    }
}

struct MapMeshBuilder<'a> {
    device: &'a mut gfx::GlDevice,
    map: &'a Map,
    texture_map: &'a (Texture + 'a),
    tile_size: f32,
    cube_maker: CubeMaker,
}

impl<'a> MapMeshBuilder<'a> {

    pub fn new(device: &'a mut gfx::GlDevice, texture_map: &'a Texture,
               map: &'a Map, tile_size: f32) -> MapMeshBuilder<'a> {
        MapMeshBuilder {
            device: device,
            map: map,
            texture_map: texture_map,
            tile_size: tile_size,
            cube_maker: CubeMaker::new(),
        }
    }

    fn get_pos(&self, x: u16, y: u16, z: u16) -> (f32, f32, f32) {
        ((x as f32) * self.tile_size,
         (z as f32) * self.tile_size,
         ((self.map.height() - y - 1) as f32) * self.tile_size)
    }

    pub fn add_ground_tile(&mut self, x: u16, y: u16) {
        let pos = self.get_pos(x, y, 0);
        let tex = Tile::Ground.get_texture_slice(self.texture_map);
        self.cube_maker.add_top_side(pos, self.tile_size, &tex);
    }

    pub fn add_box(&mut self, x: u16, y: u16, height: u8, tile: Tile) {
        for z in range(1, height + 1) {
            let pos = self.get_pos(x, y, z as u16);
            let tex = tile.get_texture_slice(self.texture_map);
            self.cube_maker.add_left_side(pos, self.tile_size, &tex);
            self.cube_maker.add_right_side(pos, self.tile_size, &tex);
            self.cube_maker.add_far_side(pos, self.tile_size, &tex);
            self.cube_maker.add_near_side(pos, self.tile_size, &tex);
            if z == height {
                self.cube_maker.add_top_side(pos, self.tile_size, &tex);
            }
        }
    }

    pub fn build_mesh(&mut self) {
        for y in range(0, self.map.height()) {
            for x in range(0, self.map.width()) {
                let tile = self.map.get_tile(x, y);
                if tile.is_ground() {
                    self.add_ground_tile(x, y);
                } else if tile.height() > 0 {
                    self.add_box(x, y, tile.height(), tile);
                }
            }
        }
    }

    pub fn finish(self) -> MapMesh<'a> {
        let (vertex_data, index_data) = self.cube_maker.finish();
        let mesh = self.device.create_mesh(vertex_data.as_slice());
        let slice = self.device
            .create_buffer_static::<u16>(index_data.as_slice())
            .to_slice(gfx::PrimitiveType::TriangleList);
        MapMesh {
            map: self.map,
            mesh: mesh,
            slice: slice,
        }
    }
}
