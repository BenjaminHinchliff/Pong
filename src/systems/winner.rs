use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, Write, WriteStorage, Read},
    ui::UiText
};

use crate::audio::{play_score_sound, Sounds};

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH, BALL_SLOWDOWN_TIME};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                ball.slowdown_timer.replace(BALL_SLOWDOWN_TIME);
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}