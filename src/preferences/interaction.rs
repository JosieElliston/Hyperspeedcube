use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct InteractionPreferences {
    pub confirm_discard_only_when_scrambled: bool,

    pub drag_sensitivity: f32,
    pub realign_on_release: bool,
    pub realign_on_keypress: bool,
    pub smart_realign: bool,
    pub timer_blind_mode: bool,

    pub dynamic_twist_speed: bool,
    pub twist_duration: f32,
    pub other_anim_duration: f32,
}
