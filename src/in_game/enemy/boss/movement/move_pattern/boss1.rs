use std::ops::RangeInclusive;

use crate::{
    in_game::{
        enemy::boss::movement::easing::easing_vec3_ease_in_out_interpolation, life_count::LifeCount,
    },
    FPS,
};

use bevy::prelude::*;

use super::{ActionCalculater, BossAction};

const FAST_MOVE_FRAMES: i128 = FPS as i128 * 1;
const SLOW_MOVE_FRAMES: i128 = FPS as i128 * 1;

pub struct Boss1ActionCalculater {
    main_action_scenario: Boss1ActionScenario,
}

impl Boss1ActionCalculater {
    fn initial_position() -> Vec3 {
        Vec3::new(0.0, 370.0, 0.0)
    }

    fn slow_move_frames() -> i128 {
        SLOW_MOVE_FRAMES
    }

    fn fast_move_frames() -> i128 {
        FAST_MOVE_FRAMES
    }

    fn action_slack_frames() -> i128 {
        FPS as i128 * 2
    }

    fn enter_area_frames() -> i128 {
        Self::slow_move_frames()
    }

    fn enter_area_target_position() -> Vec3 {
        Vec3::new(0.0, 300.0, 0.0)
    }

    pub fn new() -> Self {
        let _initial_position = Self::initial_position();

        //登場後は弾幕を打ちながらループ
        let wave1_barrage_frames = 205i128;
        let wave2_barrage_frames = 400i128;
        let step_commands: Vec<Boss1ActionScenarioStepCommand> = vec![
            Boss1ActionScenarioStepCommand::StartBarrage {
                barrage_name: "boss1_first_wave".to_string(),
                barrage_duration: wave1_barrage_frames,
            },
            Boss1ActionScenarioStepCommand::Stay {
                frames: Self::action_slack_frames(),
            },
            Boss1ActionScenarioStepCommand::MoveTo {
                duration_frames: Self::fast_move_frames(),
                target_position: Vec3::new(100.0, 300.0, 0.0),
            },
            Boss1ActionScenarioStepCommand::StartBarrage {
                barrage_name: "boss1_second_wave".to_string(),
                barrage_duration: wave2_barrage_frames,
            },
            Boss1ActionScenarioStepCommand::Stay {
                frames: Self::action_slack_frames(),
            },
            Boss1ActionScenarioStepCommand::MoveTo {
                duration_frames: Self::fast_move_frames(),
                target_position: Vec3::new(-100.0, 200.0, 0.0),
            },
            Boss1ActionScenarioStepCommand::StartBarrage {
                barrage_name: "boss1_first_wave".to_string(),
                barrage_duration: wave1_barrage_frames,
            },
            Boss1ActionScenarioStepCommand::Stay {
                frames: Self::action_slack_frames(),
            },
            Boss1ActionScenarioStepCommand::MoveTo {
                duration_frames: Self::slow_move_frames(),
                target_position: Self::enter_area_target_position(),
            },
        ];
        let scenario = Boss1ActionScenario::build_from_step_commands(
            Self::enter_area_target_position(),
            step_commands,
        );

        Self {
            main_action_scenario: scenario,
        }
    }
}

impl ActionCalculater for Boss1ActionCalculater {
    fn action_for_life_count(&self, life_count: &LifeCount) -> BossAction {
        // 登場シーン
        if (0..=(Self::enter_area_frames())).contains(&life_count.count) {
            let start = Self::initial_position();
            let end = Vec3::new(0.0, 300.0, 0.0);
            BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                life_count.count as f32,
                &start,
                &(end - start),
                Self::enter_area_frames() as f32,
            ))
        } else {
            //登場後は弾幕を打ちながらループ
            self.main_action_scenario
                .run(
                    (life_count.count - Self::enter_area_frames() - 1)
                        % self.main_action_scenario.total_frames,
                )
                .unwrap()
        }
    }
}

/* 次に何をしてほしい行動を指示するコマンド */
enum Boss1ActionScenarioStepCommand {
    MoveTo {
        duration_frames: i128,
        target_position: Vec3,
    },
    StartBarrage {
        barrage_name: String,
        barrage_duration: i128,
    },
    Stay {
        frames: i128,
    },
}

/* シナリオとして実際に動作できるようなコマンド*/
enum Boss1ActionScenarioCommand {
    Move {
        start_position: Vec3,
        end_position: Vec3,
    },
    StartBarrrage {
        barrage_name: String,
    },
    Stay,
}

struct Boss1ActionScenario {
    pub total_frames: i128,
    commands: Vec<(RangeInclusive<i128>, Boss1ActionScenarioCommand)>,
}

impl Boss1ActionScenario {
    fn build_from_step_commands(
        initial_position: Vec3,
        step_commands: Vec<Boss1ActionScenarioStepCommand>,
    ) -> Self {
        let mut commands = Vec::new();
        let mut current_position = initial_position;
        let mut current_frame = 0;
        for s in step_commands.iter() {
            match s {
                Boss1ActionScenarioStepCommand::MoveTo {
                    duration_frames,
                    target_position,
                } => {
                    commands.push((
                        RangeInclusive::new(current_frame, current_frame + duration_frames),
                        Boss1ActionScenarioCommand::Move {
                            start_position: current_position,
                            end_position: *target_position,
                        },
                    ));
                    current_frame += duration_frames + 1;
                    current_position = *target_position;
                }
                Boss1ActionScenarioStepCommand::StartBarrage {
                    barrage_duration,
                    barrage_name,
                } => {
                    commands.push((
                        RangeInclusive::new(current_frame, current_frame),
                        Boss1ActionScenarioCommand::StartBarrrage {
                            barrage_name: barrage_name.clone(),
                        },
                    ));
                    current_frame += barrage_duration + 1;
                }
                Boss1ActionScenarioStepCommand::Stay { frames } => {
                    current_frame += frames + 1;
                }
            }
        }

        Self {
            total_frames: current_frame,
            commands,
        }
    }

    fn run(&self, frame: i128) -> Option<BossAction> {
        let matched_command = self
            .commands
            .iter()
            .find(|(range, _)| range.contains(&frame));
        if let Some((range, command)) = matched_command {
            match command {
                Boss1ActionScenarioCommand::Move {
                    start_position,
                    end_position,
                } => Some(BossAction::MoveTo(easing_vec3_ease_in_out_interpolation(
                    (frame - range.start()) as f32,
                    start_position,
                    &(*end_position - *start_position),
                    (range.end() - range.start()) as f32,
                ))),
                Boss1ActionScenarioCommand::StartBarrrage { barrage_name } => {
                    Some(BossAction::StartBarrrage(barrage_name.clone()))
                }
                Boss1ActionScenarioCommand::Stay => Some(BossAction::Stay),
            }
        } else {
            Some(BossAction::Stay)
        }
    }
}
