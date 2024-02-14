use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Component, Default)]
pub struct Grid {
    pub nb_tiles: usize,
    pub grid_size: usize,
}

impl Grid {
    pub fn from(nb_tiles: usize, grid_size: usize) -> Self {
        Self {
            nb_tiles,
            grid_size,
        }
    }
    pub fn get_range(self) -> Vec<i32> {
        let size_tile: i32 = (self.grid_size / self.nb_tiles) as i32;
        let range =
            (-(self.grid_size as i32) / 2..self.grid_size as i32 / 2).step_by(size_tile as usize);
        let mut range: Vec<i32> = range.collect();
        if range.len() > self.nb_tiles {
            range.pop();
        }
        range
    }

    pub fn build(self) -> Vec<SpriteBundle> {
        let mut sprits: Vec<SpriteBundle> = Vec::new();
        let size_tile: i32 = (self.grid_size / self.nb_tiles) as i32;
        let range = self.get_range();
        for i in range.clone().into_iter() {
            for j in range.clone().into_iter() {
                sprits.push(SpriteBundle {
                    sprite: Sprite {
                        // color: Color::rgb(rand::random(), rand::random(), rand::random()),
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(size_tile as f32 - 2., size_tile as f32 - 2.)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(i as f32, j as f32, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
        sprits
    }

    pub fn update(self, i: usize, j: usize, color: Color) -> Vec<SpriteBundle> {
        let mut sprits: Vec<SpriteBundle> = Vec::new();
        let size_tile: i32 = (self.grid_size / self.nb_tiles) as i32;
        let range = self.get_range();
        sprits.push(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(size_tile as f32 - 2., size_tile as f32 - 2.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(range[i] as f32, range[j] as f32, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
        sprits
    }
}
