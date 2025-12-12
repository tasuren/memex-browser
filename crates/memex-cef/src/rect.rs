#[derive(Clone, Copy)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy)]
pub struct WebViewBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub window_size: WindowSize,
}

impl From<WebViewBounds> for cef::Rect {
    fn from(value: WebViewBounds) -> Self {
        Self {
            x: value.x,
            y: value.window_size.height - (value.height + value.y),
            width: value.width,
            height: value.height,
        }
    }
}
