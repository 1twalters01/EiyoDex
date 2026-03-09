mod sealed {
    pub trait Sealed {}
}

pub trait IntoF64Safe: Sealed {
    fn into_f64(self) -> f64;
}

use sealed::Sealed;

impl Sealed for f64 {}
impl IntoF64Safe for f64 {
    fn into_f64(self) -> f64 {
        self
    }
}

impl Sealed for f32 {}
impl IntoF64Safe for f32 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}

impl Sealed for i64 {}
impl IntoF64Safe for i64 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}

impl Sealed for i32 {}
impl IntoF64Safe for i32 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}

impl Sealed for u64 {}
impl IntoF64Safe for u64 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}

impl Sealed for u32 {}
impl IntoF64Safe for u32 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}
