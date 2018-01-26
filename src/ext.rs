pub trait BoolExt {
    fn toggle(&mut self);
}

impl BoolExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

#[derive(Debug, Default)]
pub struct InputState {
    pub hor: i8,
    pub ver: i8,
}

impl InputState {
    #[inline]
    pub fn hor(&self) -> f32 {
        self.hor.signum() as f32
    }
    #[inline]
    pub fn ver(&self) -> f32 {
        self.ver.signum() as f32
    }
}
