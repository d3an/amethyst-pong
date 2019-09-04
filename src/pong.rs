use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::timing::Time,
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

// Pong Structure Definition
#[derive(Default)]
pub struct Pong {
	ball_spawn_timer: Option<f32>,
	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

// Pong Structure Implementation
impl SimpleState for Pong {
	// Initialize the Game State on game start
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		// Wait one second before spawning the ball.
		self.ball_spawn_timer.replace(1.0);

		// Load the spritesheet necessary to render the graphics.
		// `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
		initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());
		initialize_camera(world);
	}

	// Update the Game State with the necessary data
	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		if let Some(mut timer) = self.ball_spawn_timer.take() {
			// If the timer isn't expired yet, subtract the time that passed since the last update.
			{
				let time = data.world.res.fetch::<Time>();
				timer -= time.delta_seconds();
			}

			if timer <= 0.0 {
				// When timer expire, spawn the ball
				initialize_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
			} else {
				// If timer is not expired yet, put it back onto the state.
				self.ball_spawn_timer.replace(timer);
			}
		}
		Trans::None
	}
}



// Paddle Side Enumeration
#[derive(PartialEq, Eq)]
pub enum Side {
	Left,
	Right,
}

// Paddle Structure Definition
pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32,
}

// Paddle Structure Implementation
impl Paddle {
	fn new(side: Side) -> Paddle {
		Paddle {
			side,
			width: PADDLE_WIDTH,
			height: PADDLE_HEIGHT,
		}
	}
}

// ECS Paddle Addition
impl Component for Paddle {
	type Storage = DenseVecStorage<Self>;
}


// Ball Structure

pub struct Ball {
	pub velocity: [f32; 2],
	pub radius: f32,
}

impl Component for Ball {
	type Storage = DenseVecStorage<Self>;
}

/* 
=================================================================
==================== Additional Functions =======================
=================================================================
*/

// Initializes camera given our world
fn initialize_camera(world: &mut World) {
	// Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.with(transform)
		.build();
}

// Initializes left and right paddles
fn initialize_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();

	// Correctly position the paddles
	let y = ARENA_HEIGHT / 2.0;
	left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
	right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

	// Assign the sprites for the paddles
	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet.clone(),
		sprite_number: 0, // paddle is the first sprite in the sprite_sheet
	};

	// Create left paddle entity
	world
		.create_entity()
		.with(sprite_render.clone())
		.with(Paddle::new(Side::Left))
		.with(left_transform)
		.build();

	// Create right paddle entity
	world
		.create_entity()
		.with(sprite_render.clone())
		.with(Paddle::new(Side::Right))
		.with(right_transform)
		.build();
}

// Loads spritesheet and extracts necessary embedded sprites
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	// Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
    	let loader = world.read_resource::<Loader>();
    	let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    	loader.load(
    		"texture/pong_spritesheet.png",
    		ImageFormat::default(),
    		(),
    		&texture_storage,
    	)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
    	"texture/pong_spritesheet.ron",
    	SpriteSheetFormat(texture_handle),
    	(),
    	&sprite_sheet_store,
    )
}

// Initializes a single ball in the general centre of the arena
fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	// Create the translation
	let mut local_transform = Transform::default();
	local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

	// Assign the sprite for the ball
	let sprite_render = SpriteRender {
		sprite_sheet: sprite_sheet_handle,
		sprite_number: 1, // Ball is the second sprite on the sprite sheet
	};

	world
		.create_entity()
		.with(sprite_render)
		.with(Ball {
			radius: BALL_RADIUS,
			velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
		})
		.with(local_transform)
		.build();
}















