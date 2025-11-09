use gpui::{Entity, prelude::*, px};
use gpui_component::*;

pub struct Exproler {
    tree: Entity<TreeState>,
}

impl Exproler {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            tree: cx.new(|cx| {
                TreeState::new(cx).items(vec![
                    TreeItem::new("cef-github", "tauri-apps/cef-rs"),
                    TreeItem::new("cef-forum", "CEF Forum . aaa"),
                    TreeItem::new("cef-forum-2", "Object reference"),
                    TreeItem::new("note", "note.md"),
                ])
            }),
        }
    }
}

impl Render for Exproler {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        tree(&self.tree, |ix, entry, _selected, _window, _cx| {
            let icon = if entry.is_folder() {
                if entry.is_expanded() {
                    IconName::FolderOpen
                } else {
                    IconName::Folder
                }
            } else {
                IconName::File
            };

            ListItem::new(ix)
                .pl(px(16.) * entry.depth() + px(12.))
                .child(
                    h_flex()
                        .gap_2()
                        .w_full()
                        .child(icon)
                        .child(entry.item().label.clone()),
                )
        })
    }
}
