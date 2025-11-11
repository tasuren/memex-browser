pub mod application {
    use std::{
        ffi::CStr,
        sync::{LazyLock, Mutex},
    };

    use objc2::{runtime::*, *};
    use objc2_app_kit::NSApplication;

    const GPUI_APP_NAME: &CStr = c"GPUIApplication";

    pub fn impl_cef_protocol_for_gpui_app() {
        let mtm =
            MainThreadMarker::new().expect("This function is must be called from main thread.");
        let app = NSApplication::sharedApplication(mtm).class();
        assert_eq!(app.name(), GPUI_APP_NAME);

        unsafe {
            let _ = objc2::ffi::class_addMethod(
                app as *const _ as *mut _,
                sel!(isHandlingSendEvent),
                std::mem::transmute(is_handling_send_event as *const ()),
                CStr::from_bytes_with_nul(b"B@:\0").unwrap().as_ptr(),
            );

            let _ = objc2::ffi::class_addMethod(
                app as *const _ as *mut _,
                sel!(setHandlingSendEvent:),
                std::mem::transmute(set_handling_send_event as *const ()),
                CStr::from_bytes_with_nul(b"v@:B\0").unwrap().as_ptr(),
            );
        }
    }

    static HANDLING_SEND_EVENT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

    extern "C" fn set_handling_send_event(
        _this: *mut AnyObject,
        _cmd: Sel,
        handling_send_event: Bool,
    ) {
        *HANDLING_SEND_EVENT.lock().unwrap() = handling_send_event.as_bool();
    }

    extern "C" fn is_handling_send_event(_this: *mut AnyObject, _cmd: Sel) -> Bool {
        Bool::new(*HANDLING_SEND_EVENT.lock().unwrap())
    }
}
