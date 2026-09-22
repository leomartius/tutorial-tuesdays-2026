use log::info;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use super::level::Level;
use super::{Tile, Vec2};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const ROOM_MIN_SIZE: i32 = 5;
pub const ROOM_MAX_SIZE: i32 = 9;
pub const ROOM_MAX_COUNT: usize = 30;

struct Room {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Room {
    fn random(
        min_width: i32,
        min_height: i32,
        max_width: i32,
        max_height: i32,
        map_width: i32,
        map_height: i32,
        rng: &mut impl RngExt,
    ) -> Self {
        let width = rng.random_range(min_width..=max_width);
        let height = rng.random_range(min_height..=max_height);
        let x = rng.random_range(0..map_width - width - 1);
        let y = rng.random_range(0..map_height - height - 1);
        Self {
            x0: x,
            y0: y,
            x1: x + width + 1,
            y1: y + height + 1,
        }
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }

    fn carve(&self, level: &mut Level) {
        level.fill_rect(
            self.x0 + 1,
            self.y0 + 1,
            self.x1 - 1,
            self.y1 - 1,
            Tile::Floor,
        );
    }

    fn tunnel_to(&self, other: &Room, level: &mut Level, rng: &mut impl RngExt) {
        let (xa, ya) = self.random_xy(rng);
        let (xb, yb) = other.random_xy(rng);

        let xm;
        let ym;
        if rng.random() {
            xm = xa;
            ym = yb;
        } else {
            xm = xb;
            ym = ya;
        }

        level.fill_rect(xa, ya, xm, ym, Tile::Floor);
        level.fill_rect(xm, ym, xb, yb, Tile::Floor);
    }

    fn random_xy(&self, rng: &mut impl RngExt) -> (i32, i32) {
        let x = rng.random_range(self.x0 + 1..=self.x1 - 1);
        let y = rng.random_range(self.y0 + 1..=self.y1 - 1);
        (x, y)
    }
}

pub fn generate_level() -> Level {
    let mut rng = seeded_rng();
    let mut rooms: Vec<Room> = Vec::with_capacity(ROOM_MAX_COUNT);

    'outer: for _ in 0..ROOM_MAX_COUNT {
        let room = Room::random(
            ROOM_MIN_SIZE,
            ROOM_MIN_SIZE,
            ROOM_MAX_SIZE,
            ROOM_MAX_SIZE,
            MAP_WIDTH,
            MAP_HEIGHT,
            &mut rng,
        );
        for other_room in &rooms {
            if room.intersects(other_room) {
                continue 'outer;
            }
        }
        rooms.push(room);
    }

    let (x, y) = rooms[0].random_xy(&mut rng);
    let mut level = Level::new(MAP_WIDTH, MAP_HEIGHT, Vec2 { x, y });
    for room in &rooms {
        room.carve(&mut level);
    }

    for room1_room2 in rooms.windows(2) {
        if let [room1, room2] = room1_room2 {
            room1.tunnel_to(room2, &mut level, &mut rng);
        }
    }

    level
}

fn seeded_rng() -> impl RngExt {
    let seed: u64 = rand::rng().random();
    info!("Level seed is 0x{seed:08X?}");
    SmallRng::seed_from_u64(seed)
}
