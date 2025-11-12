pub mod view {
    use memex_cef::{Browser, UIThreadMarker};
    use objc2::rc::Retained;
    use objc2_app_kit::NSView;

    pub fn set_hidden(utm: UIThreadMarker, browser: &Browser, hidden: bool) {
        // Operation with `NSView` should be done on main thread (UI thread).
        let ptr = browser.view_handle(utm).unwrap() as *mut NSView;

        let nsview = unsafe { Retained::retain(ptr) }.unwrap();
        nsview.setHidden(hidden);
    }

    pub fn close_view(utm: UIThreadMarker, browser: &Browser) {
        let ptr = browser.view_handle(utm).unwrap() as *mut NSView;
        unsafe {
            let ns_view = Retained::retain(ptr).unwrap();
            ns_view.removeFromSuperview();
        }
    }
}
