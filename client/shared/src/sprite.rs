use rask_game_engine::math;

pub type AnimationId = u32;
pub type FrameId = u32;

#[derive(Clone, Copy)]
pub struct Sprite {
    pos: math::Vec2,
    animation_id: AnimationId,
    frame_id: FrameId,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            pos: math::Vec2::new(0.0, 0.0),
            animation_id: 0,
            frame_id: 0
        }
    }
}

pub struct Animation {
    frames: Vec<Sprite>
}

pub struct Frame {
    transformations: Vec<math::Mat3>,
}
