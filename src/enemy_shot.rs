use super::enemy::Enemy;
use super::play_area_descriptor::PlayAreaDescriptor;
use bevy::prelude::*;
use rand::prelude::*;

const BULLET_SIZE: f32 = 15.0;

pub struct EnemyShotPlugin;

impl Plugin for EnemyShotPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ShotBulletTimer::default())
		 	.add_system(randomly_shot_enemy_bullet_system)
			.add_system(move_enemy_bullet_system)
			.add_system(destroy_enemy_bullet_go_outside_system)
			.add_system(move_enemy_bullet_system);
	}
}
/*
 * Component
 */
#[derive(Component)]
pub struct Bullet {
	velocity: Vec3,
}

struct ShotBulletTimer(Timer);

impl Default for ShotBulletTimer {
	fn default() -> Self {
		ShotBulletTimer(Timer::from_seconds(2.0, true))
	}
}

/*
 * System
 */
fn randomly_shot_enemy_bullet_system(
	time: Res<Time>,
	mut timer: ResMut<ShotBulletTimer>,
	mut commands: Commands,
	enemy_query: Query<(&Enemy, &Transform)>,
) {
	if !timer.0.tick(time.delta()).just_finished() {
		return;
	}

	let mut rng = rand::thread_rng();
	let mut shot_count = rng.gen_range(0..=3);
	if shot_count == 0 {
		return;
	}
	for (_, enemy_transform) in enemy_query.iter() {
		if shot_count <= 0 {
			break;
		}

		let shot = rng.gen::<bool>();
		if shot {
			match rng.gen_range(0..=1) {
				0 => shot_strait_enemy_bullet(&mut commands, enemy_transform),
				1 => shot_triple_enemy_bullet(&mut commands, enemy_transform),
				_ => panic!("Unexpected bullet type"),
			}
			shot_count -= 1;
		}
	}
}

fn move_enemy_bullet_system(mut query: Query<(&Bullet, &mut Transform)>) {
	for (bullet, mut transform) in query.iter_mut() {
		transform.translation.x += bullet.velocity.x;
		transform.translation.y += bullet.velocity.y;
		transform.translation.z += bullet.velocity.z;
	}
}

fn destroy_enemy_bullet_go_outside_system(
	play_area: Res<PlayAreaDescriptor>,
	mut commands: Commands,
	enemy_bullet_query: Query<(Entity, &Bullet, &Transform)>,
) {
	for (enemy_bullet_entity, _, enemy_bullet_transform) in enemy_bullet_query.iter() {
		let enemy_bullet_top_y = enemy_bullet_transform.translation.y + BULLET_SIZE / 2.0;
		if enemy_bullet_top_y < play_area.min_y {
			commands.entity(enemy_bullet_entity).despawn();
		}
	}
}

/*
 * Util
 */
fn shot_strait_enemy_bullet(commands: &mut Commands, enemy_transform: &Transform) {
	commands
		.spawn_bundle(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(
					enemy_transform.translation.x,
					enemy_transform.translation.y,
					0.0,
				),
				scale: Vec3::new(BULLET_SIZE, BULLET_SIZE, BULLET_SIZE),
				..Default::default()
			},
			sprite: Sprite {
				color: Color::rgb(0.0, 153.0 / 255.0, 51.0 / 255.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Bullet {
			velocity: Vec3::new(0.0, -3.0, 0.0),
		});
}

fn shot_triple_enemy_bullet(commands: &mut Commands, enemy_transform: &Transform) {
	let bullet_vectors = vec![
		Vec3::new(-3.0 * 0.71, -3.0 * 0.71, 0.0),
		Vec3::new(0.0, -3.0 * 0.71, 0.0),
		Vec3::new(3.0 * 0.71, -3.0 * 0.71, 0.0),
	];
	for v in bullet_vectors.into_iter() {
		commands
			.spawn_bundle(SpriteBundle {
				transform: Transform {
					translation: Vec3::new(
						enemy_transform.translation.x,
						enemy_transform.translation.y,
						0.0,
					),
					scale: Vec3::new(BULLET_SIZE, BULLET_SIZE, BULLET_SIZE),
					..Default::default()
				},
				sprite: Sprite {
					color: Color::rgb(0.0, 153.0 / 255.0, 51.0 / 255.0),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Bullet { velocity: v });
	}
}
