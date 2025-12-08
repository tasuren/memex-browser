#[derive(Clone)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub window_size: WindowSize,
}

impl From<Rect> for cef::Rect {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.window_size.height - (value.height + value.y),
            width: value.width,
            height: value.height,
        }
    }
}

