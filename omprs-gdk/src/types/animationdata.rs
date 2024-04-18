#[derive(Clone, Debug, Default)]
pub struct AnimationData {
    pub delta: f32,
    pub looping: bool,
    pub lockX: bool,
    pub lockY: bool,
    pub freeze: bool,
    pub time: usize,
    lib: String,
    name: String,
}

impl AnimationData {
    pub fn new(
        delta: f32,
        looping: bool,
        lockX: bool,
        lockY: bool,
        freeze: bool,
        time: usize,
        lib: &str,
        name: &str,
    ) -> Self {
        AnimationData {
            delta,
            looping,
            lockX,
            lockY,
            freeze,
            time,
            lib: lib.into(),
            name: name.into(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_animation_library(&self) -> String {
        self.lib.to_string()
    }
}
