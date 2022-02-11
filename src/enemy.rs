use super::life_count;
use super::life_count::LifeCount;
use bevy::prelude::*;
use rand::prelude::*;

const ENEMY_SIZE: f32 = 30.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(EnemyLifeCountTimer::default())
			.insert_resource(EnemyMoveTimer::default())
			.add_startup_system(setup)
			.add_system(count_up_enemy_life_count_system)
			.add_system(move_enemy_system)
			.add_system(update_enemy_velocity_system);
	}
}

#[derive(Component, Default)]
pub struct Enemy {
	pub velocity: Vec3,
}

struct EnemyLifeCountTimer(Timer);

impl Default for EnemyLifeCountTimer {
	fn default() -> Self {
		EnemyLifeCountTimer(Timer::from_seconds(1.0 / 40.0, true))
	}
}

struct EnemyMoveTimer(Timer);

impl Default for EnemyMoveTimer {
	fn default() -> Self {
		EnemyMoveTimer(Timer::from_seconds(1.0 / 40.0, true))
	}
}

#[derive(Component)]
enum MoveStrategy {
	DownStayUp,
	DownStayLeftBottom,
	DownStayRightBottom,
	FastDownLeft,
	FastDownRight,
	LeftBottom,
	RightBottom,
}

impl MoveStrategy {
	fn random() -> Self {
		let mut rng = rand::thread_rng();
		match rng.gen_range(0..=6) {
			0 => MoveStrategy::DownStayUp,
			1 => MoveStrategy::DownStayLeftBottom,
			2 => MoveStrategy::DownStayRightBottom,
			3 => MoveStrategy::FastDownLeft,
			4 => MoveStrategy::FastDownRight,
			5 => MoveStrategy::LeftBottom,
			6 => MoveStrategy::RightBottom,
			_ => panic!("Unexpected strategy")
		}
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
					}, transform: Transform { translation: brick_position,
						scale: enemy_size,
						..Default::default()
					},
					texture: alien_handles[row].clone().into(),
					..Default::default()
				})
				.insert(Enemy::default())
				.insert(LifeCount::default())
				.insert(MoveStrategy::random());
		}
	}
}

fn count_up_enemy_life_count_system(
	time: Res<Time>,
	mut timer: ResMut<EnemyLifeCountTimer>,
	mut query: Query<(&Enemy, &mut LifeCount)>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (_, mut enemy_life_count) in query.iter_mut() {
			enemy_life_count.count += 1;
		}
	}
}

fn move_enemy_system(
	time: Res<Time>,
	mut timer: ResMut<EnemyMoveTimer>,
	mut query: Query<(&Enemy, &mut Transform)>
) {
	if timer.0.tick(time.delta()).just_finished() {
		for (enemy, mut transform) in query.iter_mut() {
			transform.translation += enemy.velocity;
		}
	}
}

fn update_enemy_velocity_system(
	mut query: Query<(&mut Enemy, &LifeCount, &MoveStrategy)>,
) {
	for (mut enemy, life_count, move_strategy) in query.iter_mut() {
		match move_strategy {
			MoveStrategy::DownStayUp => {
				match life_count.count {
					0 => enemy.velocity = Vec3::new(0.0, -1.0, 0.0),
					40 => enemy.velocity = Vec3::new(0.0, 0.0, 0.0),
					80 => enemy.velocity = Vec3::new(0.0, -3.0, 0.0),
					_ => {}
				}
			},
			MoveStrategy::DownStayLeftBottom => {
				match life_count.count {
					0 => enemy.velocity = Vec3::new(0.0, -1.0, 0.0),
					40 => enemy.velocity = Vec3::new(0.0, 0.0, 0.0),
					80 => enemy.velocity = Vec3::new(-1.0, -2.0, 0.0),
					_ => {}
				}
			}
			MoveStrategy::DownStayRightBottom => {
				match life_count.count {
					0 => enemy.velocity = Vec3::new(0.0, -1.0, 0.0),
					40 => enemy.velocity = Vec3::new(0.0, 0.0, 0.0),
					80 => enemy.velocity = Vec3::new(1.0, -2.0, 0.0),
					_ => {}
				}
			}
			MoveStrategy::FastDownLeft => {
				if life_count.count == 0 {
					enemy.velocity = Vec3::new(0.0, -5.0, 0.0);
				}
				if life_count.count < 100 {
					enemy.velocity.x -= 5. / 100.;
					enemy.velocity.y -= 5. / 100.;
				}
			}
			MoveStrategy::FastDownRight => {
				if life_count.count == 0 {
					enemy.velocity = Vec3::new(0.0, -5.0, 0.0);
				}
				if life_count.count < 100 {
					enemy.velocity.x += 5. / 100.;
					enemy.velocity.y -= 5. / 100.;
				}
			}
			MoveStrategy::LeftBottom => {
				if life_count.count == 0 {
					enemy.velocity = Vec3::new(-1.0, -2.0, 0.0);
				}
			}
			MoveStrategy::RightBottom => {
				if life_count.count == 0 {
					enemy.velocity = Vec3::new(1.0, -2.0, 0.0);
				}
			}
		}
	}
}