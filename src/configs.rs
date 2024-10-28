pub const SPEED_PLAYER: f32 = 8.0;
pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const PADDING_BOUNDS: f32 = 80.0;
pub const BOUNDS: (f32, f32) = (
    WINDOW_WIDTH - PADDING_BOUNDS,
    WINDOW_HEIGHT - PADDING_BOUNDS,
);

pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_SPEED: f32 = 10.0;
pub const BULLET_TIME_SECS: f32 = 0.5;

pub const PLAYER_SPRITE: &str = "player.png";
pub const PLAYER_SIZE: (f32, f32) = (144., 75.);
pub const PLAYER_LASER_SPRITE: &str = "laser_a.png";
pub const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

pub const ENEMY_SPRITE: &str = "enemy.png";
pub const ENEMY_SIZE: (f32, f32) = (144., 75.);
pub const ENEMY_LASER_SPRITE: &str = "laser_b.png";
pub const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

pub const EXPLOSION_SHEET: &str = "exploded.png";
pub const EXPLOSION_LEN: usize = 16;

pub const SPRITE_SCALE: f32 = 0.5;
