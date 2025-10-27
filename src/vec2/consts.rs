/// f32 constants
pub mod f32 {
    use crate::vec2::Vec2;

    /// Vector all of whose components are equal to zero
    pub const ZERO: Vec2<f32> = Vec2 { x: 0.0, y: 0.0 };

    /// Vector all of whose components are equal to one
    pub const ONE: Vec2<f32> = Vec2 { x: 1.0, y: 1.0 };

    /// x = 1; y = 0;
    pub const X: Vec2<f32> = Vec2 { x: 1.0, y: 0.0 };

    /// x = 0; y = 1;
    pub const Y: Vec2<f32> = Vec2 { x: 0.0, y: 1.0 };

    /// x = -1; y = 0;
    pub const NEG_X: Vec2<f32> = Vec2 { x: -1.0, y: 0.0 };

    /// x = 0; y = -1;
    pub const NEG_Y: Vec2<f32> = Vec2 { x: 0.0, y: -1.0 };
}

/// i32 constants
pub mod i32 {
    use crate::vec2::Vec2;

    /// Vector all of whose components are equal to zero
    pub const ZERO: Vec2<i32> = Vec2 { x: 0, y: 0 };

    /// Vector all of whose components are equal to one
    pub const ONE: Vec2<i32> = Vec2 { x: 1, y: 1 };

    /// x = 1; y = 0;
    pub const X: Vec2<i32> = Vec2 { x: 1, y: 0 };

    /// x = 0; y = 1;
    pub const Y: Vec2<i32> = Vec2 { x: 0, y: 1 };

    /// x = -1; y = 0;
    pub const NEG_X: Vec2<i32> = Vec2 { x: -1, y: 0 };

    /// x = 0; y = -1;
    pub const NEG_Y: Vec2<i32> = Vec2 { x: 0, y: -1 };
}
