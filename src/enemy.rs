use bevy::prelude::*;
use super::life_count::LifeCount;
use super::life_count;

const ENEMY_SIZE: f32 = 30.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(EnemyLifeCountTimer::default())
			.add_startup_system(setup)
			.add_system(count_up_enemy_life_count_system);
	}
}

#[derive(Component)]
pub struct Enemy;

struct EnemyLifeCountTimer(Timer);

impl Default for EnemyLifeCountTimer {
	fn default() -> Self {
		EnemyLifeCountTimer(Timer::from_seconds(1.0 / 40.0, true))
	}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

	// Load assets
	let alien_handles: Vec<Handle<Image>> = vec![
		asset_server.load("images/blue_alien.png"),
		asset_server.load("images/pink_alien.png"),
		asset_server.load("images/purple_alien.png"),
		asset_server.load("images/yellow_alien.png"),
	];

	// Enemy
	let enemy_rows = 4;
	let enemy_columns = 6;
	let enemy_spacing = 21.0;
	let enemy_size = Vec3::new(ENEMY_SIZE, ENEMY_SIZE, ENEMY_SIZE);
	let enemies_width = enemy_columns as f32 * (enemy_size.x + enemy_spacing) - enemy_spacing;
	// center the bricks and move them up a bit
	let enemies_offset = Vec3::new(-(enemies_width - enemy_size.x) / 2.0, 100.0, 0.0);
	for row in 0..enemy_rows {
		let y_position = row as f32 * (enemy_size.y + enemy_spacing);
		for column in 0..enemy_columns {
			let brick_position = Vec3::new(
				column as f32 * (enemy_size.x + enemy_spacing),
				y_position,
				0.0,
			) + enemies_offset;

			commands
				.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						custom_size: Some(Vec2::new(1.0, 1.0)),
						..Default::default()
					},
					transform: Transform {
						translation: brick_position,
						scale: enemy_size,
						..Default::default()
					},
					texture: alien_handles[row].clone().into(),
					..Default::default()
				})
				.insert(Enemy)
				.insert(LifeCount::default());
		}
	}
}

fn count_up_enemy_life_count_system(
	time: Res<Time>,
	mut timer: ResMut<EnemyLifeCountTimer>,
	mut query: Query<(&Enemy, &mut life_count::LifeCount)>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (_, mut enemy_life_count) in query.iter_mut() {
			enemy_life_count.count += 1;
		}
	}
}