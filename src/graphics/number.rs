/// Any number of type Integer(i32), Float(f32) and UsInteger(u32)
#[derive(Copy, Clone)]
pub enum Number {
    /// i32
    Integer(i32),
    /// f32
    Float(f32),
    /// u32
    UsInteger(u32),
}

/// An array of type Integer(&'static [[i32]]), Float(&'static [[f32]]), UsInteger(&'static [[u32]])
#[derive(Copy, Clone)]
pub enum Array {
    /// Integer(&'static [[i32]])
    Integer(&'static [i32]),
    /// Float(&'static [[f32]])
    Float(&'static [f32]),
    /// UsInteger(&'static [[u32]])
    UsInteger(&'static [u32]),
}

/// MultiSingularNumber can be either [Array] or [Number]
#[derive(Copy, Clone)]
pub enum MultiSingularNumber {
    /// Any number of type Integer(i32), Float(f32) and UsInteger(u32)
    Number(Number),
    /// An array of type Integer(&'static [[i32]]), Float(&'static [[f32]]), UsInteger(&'static [[u32]])
    Array(Array),
    /// Default value, not an Array or Number
    None,
}
