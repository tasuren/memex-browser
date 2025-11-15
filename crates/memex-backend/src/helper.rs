use gpui::{App, Context, Window};
use memex_cef::UIThreadMarker;

pub trait OnlyUIThread {
    fn utm(self) -> UIThreadMarker;
}

impl OnlyUIThread for &mut Window {
    fn utm(self) -> UIThreadMarker {
        // gpuiのWindowはSyncとSendを実装しておらず、メインスレッドで稼働する。
        // このため、UI Threadで稼働しているといえる。
        UIThreadMarker::new().unwrap()
    }
}

impl OnlyUIThread for &mut App {
    fn utm(self) -> UIThreadMarker {
        // Appはメインスレッドが所有する。
        // このため、それを参照しているのは通常、UI Threadで稼働しているといえる。
        UIThreadMarker::new().unwrap()
    }
}

impl<T> OnlyUIThread for &mut Context<'_, T> {
    fn utm(self) -> UIThreadMarker {
        // gpuiのContextはAppへの参照を持ち、メインスレッドで扱う。
        // このため、UI Threadで稼働しているといえる。
        UIThreadMarker::new().unwrap()
    }
}
