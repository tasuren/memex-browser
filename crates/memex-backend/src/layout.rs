use gpui::{Pixels, Window};
use memex_cef::utils::{Rect, WindowSize};

/// UIの大きさから、適切なレイアウトを決定するのに使う、状態。
pub struct LayoutState {
    pub top_tab_bar_height: Pixels,
    pub url_bar_height: Pixels,
    pub workspace_list_width: Pixels,
    pub exproler_width: Pixels,
}

impl LayoutState {
    pub fn view_rect(&self, window: &mut Window) -> Rect {
        let view_offset_x = self.workspace_list_width + self.exproler_width;
        let view_offset_y = self.top_tab_bar_height + self.url_bar_height;
        let window_size = window.bounds().size;

        let x = view_offset_x.to_f64() as i32;
        let y = view_offset_y.to_f64() as i32;

        let window_width = window_size.width.to_f64() as i32;
        let window_height = window_size.height.to_f64() as i32;

        Rect {
            x,
            y,
            width: window_width - x,
            height: window_height - y,
            window_size: WindowSize {
                width: window_width,
                height: window_height,
            },
        }
    }
}
