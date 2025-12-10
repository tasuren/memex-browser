pub trait EventHandler {
    fn on_title_change(&self, title: String);
}
