use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, BALL_SLOWDOWN_FACTOR};

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut locals, time): Self::SystemData) {
        for (ball, local) in (&mut balls, &mut locals).join() {
            let mut scaled_velocity_x = ball.velocity[0];
            let mut scaled_velocity_y = ball.velocity[1];
            if let Some(mut timer) = ball.slowdown_timer.take() {
                timer -= time.delta_seconds();
                if timer > 0.0 {
                    scaled_velocity_x /= BALL_SLOWDOWN_FACTOR;
                    scaled_velocity_y /= BALL_SLOWDOWN_FACTOR;
                    ball.slowdown_timer.replace(timer);
                }
            }
            local.prepend_translation_x(scaled_velocity_x * time.delta_seconds());
            local.prepend_translation_y(scaled_velocity_y * time.delta_seconds());
        }
    }
}