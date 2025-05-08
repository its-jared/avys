use bevy::prelude::*;

pub enum AvysState {
    Dormant,
    Spreading, 
    Destroyed,
}

#[derive(Resource)]
pub struct AvysData {
    pub avys_destroyed: bool,
    pub avys_effect: f32,
    pub avys_state: AvysState,
}

fn update(
    mut data: ResMut<AvysData>,
) {
    if data.avys_destroyed { return; }

    if data.avys_effect < 0.0 { data.avys_effect = 0.0; }
    if data.avys_effect > 1.0 { data.avys_effect = 1.0; }

    if data.avys_effect == 0.0 {
        data.avys_destroyed = true;
        data.avys_state = AvysState::Destroyed;
    }
}

pub struct AvysMechanicPlugin; 

impl Plugin for AvysMechanicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            update
                .run_if(resource_exists::<AvysData>)
        );
    }
}