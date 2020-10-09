use amethyst::{
    assets::Handle,
    core::{math::Vector3, Time, Transform},
    ecs::{prelude::*, Component, DenseVecStorage, Entity, World},
    prelude::{Builder, WorldExt},
    renderer::{
        palette::{Hsl, RgbHue, Srgba},
        resources::Tint,
        SpriteRender, SpriteSheet,
    },
};

use super::Universe;

pub struct TileEntities(pub Vec<Entity>);

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
}

impl Tile {
    pub const SIZE: f32 = 20.0;
    pub const MAP_WIDTH: u32 = Universe::WIDTH_TILE;
    pub const MAP_HEIGHT: u32 = Universe::HEIGHT_TILE;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ttype: TileType::default(),
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Neutral,
    Blue,
    Red,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Neutral
    }
}

pub fn create_tiles(world: &mut World) {
    //world.register::<Tile>();
    let sprite_render = {
        let sprite_sheet = world.fetch::<Handle<SpriteSheet>>();
        SpriteRender::new((*sprite_sheet).clone(), 0)
    };
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    let mut tiles = Vec::with_capacity(Tile::MAP_HEIGHT as usize * Tile::MAP_WIDTH as usize);

    for y in 0..Tile::MAP_HEIGHT {
        for x in 0..Tile::MAP_WIDTH {
            let tile_component = Tile::default();
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                x as f32 * Tile::SIZE + Tile::SIZE as f32 / 2.0,
                y as f32 * Tile::SIZE + Tile::SIZE as f32 / 2.0,
                -1.0,
            );
            transform.set_scale(Vector3::new(
                Tile::SIZE as f32 / 4.0,
                Tile::SIZE as f32 / 4.0,
                1.0,
            ));
            let entity = world
                .create_entity()
                .with(tile_component)
                .with(transform)
                .with(sprite_render.clone())
                .with(tint.clone())
                .build();

            tiles.push(entity);
        }
    }
    let tiles = TileEntities(tiles);
    world.insert(tiles);
}

pub struct TileSystem;

impl<'s> System<'s> for TileSystem {
    type SystemData = (
        WriteStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, (tiles, transforms, mut tints, time): Self::SystemData) {
        (&tiles, &transforms, &mut tints)
            .par_join()
            .for_each(|(_tile, transform, tint)| {
                let (x, y) = (
                    ((transform.translation().x - Tile::SIZE as f32 / 2.0) / Tile::SIZE as f32)
                        as u32,
                    ((transform.translation().y - Tile::SIZE as f32 / 2.0) / Tile::SIZE as f32)
                        as u32,
                );
                let index_frac = (x + y) as f32 / (Tile::MAP_WIDTH + Tile::MAP_HEIGHT) as f32;
                let frame_interval = 2000;
                let time_frac =
                    (time.frame_number() % frame_interval) as f32 / frame_interval as f32;
                let hue = (index_frac - time_frac) * std::f32::consts::PI * 2.0;
                tint.0 = Hsl::new(RgbHue::from_radians(hue), 1.0, 0.5).into();
            });
        println!(
            "average FPS: {}",
            time.frame_number() as f64 / time.absolute_time_seconds()
        );
    }
}
