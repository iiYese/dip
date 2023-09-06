use bevy::{
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
};

// For quick migration
#[derive(Resource, Deref, DerefMut)]
pub struct DipRes<T>(pub T);
