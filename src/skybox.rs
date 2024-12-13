use fmc::{networking::Server, prelude::*, protocol::messages};

pub struct SkyPlugin;
impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, day_night_cycle);
    }
}

// time = 0, dawn
// time = 600, dusk
const DAY_LENGTH: f32 = 1200.0;

fn day_night_cycle(time: Res<Time>, net: Res<Server>) {
    let message = messages::Time {
        angle: time.elapsed_seconds() * std::f32::consts::TAU / DAY_LENGTH,
    };
    net.broadcast(message);
}
