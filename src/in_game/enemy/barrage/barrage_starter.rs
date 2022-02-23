use bevy::prelude::*;
use bevy_bulletml::{BulletMLServer, Runner};

use super::{
    bullet::{Bullet, BulletType},
    bulletml_runner::{BulletMLRunner, BulletMLRunnerData},
};

pub trait BarrageStarter {
    fn start_barrage(
        &mut self,
        transform: &Transform,
        bulletml_server: &BulletMLServer,
        barrage_type: &str,
    ) -> Result<(), anyhow::Error>;
}

impl BarrageStarter for Commands<'_, '_> {
    fn start_barrage(
        &mut self,
        transform: &Transform,
        bulletml_server: &BulletMLServer,
        barrage_type: &str,
    ) -> Result<(), anyhow::Error> {
        let bml = bulletml_server.get(barrage_type);
        if let Some(bml) = bml {
            self.spawn()
                .insert(Bullet {
                    vanished: true,
                    ..Default::default()
                })
                .insert(Transform {
                    translation: transform.translation,
                    ..Default::default()
                })
                .insert(BulletType::WithRunner {
                    data: BulletMLRunnerData::default(),
                    runner: Runner::new(BulletMLRunner, bml.clone()),
                });
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to load barrage: {}", barrage_type))
        }
    }
}
