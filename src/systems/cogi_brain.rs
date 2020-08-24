use crate::cogi::Cogi;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use nalgebra::DVector;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct CogiBrainSystem;

impl<'s> System<'s> for CogiBrainSystem {
    type SystemData = (
        WriteStorage<'s, Cogi>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cogis, transforms, time): Self::SystemData) {
        for (mut cogi, transform) in (&mut cogis, &transforms).join() {
            let input = |_c, r| match r {
                0 => transform.translation().x,
                1 => transform.translation().x,
                2 => cogi.velocity[0],
                3 => cogi.velocity[1],
                4 => time.absolute_time().as_secs_f32(),
                _ => unreachable!(),
            };
            let input: DVector<f32> = DVector::from_fn(5, input);
            let output = cogi.brain.neural_network.feed(input);
            if output[0].max(output[1]) > 2.5 {
                // BUG: values should always be in 0.0..1.0
                dbg!(&output);
            }
            cogi.force[0] = output[0] - 0.5;
            cogi.force[1] = output[1] - 0.5;
        }
    }
}