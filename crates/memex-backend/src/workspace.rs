use std::collections::HashMap;

use anyhow::Context as _;
use gpui::{App, Context, Entity, WeakEntity, Window, prelude::*};
use memex_cef::{Profile, UIThreadMarker, utils::Rect};
use raw_window_handle::HasWindowHandle;
use uuid::Uuid;

use crate::{
    OnlyUIThread, WorkspaceListState,
    data::{AppPath, TabLocationData, WorkspaceData, WorkspaceIconData, delete_workspace},
    os::file_system::FileSystemItem,
    tab::Tab,
};

pub struct WorkspaceState {
    workspace_list: WeakEntity<WorkspaceListState>,
    profile: Profile,

    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) icon: WorkspaceIconData,
    pub(crate) tabs: HashMap<Uuid, Entity<Tab>>,
    pub(crate) tab_order: Vec<Uuid>,
    pub(crate) selected: Option<Uuid>,

    files: Vec<FileSystemItem>,
}

impl WorkspaceState {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<'_, WorkspaceListState>,
        rect: Rect,
        data: WorkspaceData,
        files: Vec<FileSystemItem>,
    ) -> anyhow::Result<Entity<Self>> {
        let workspace_list = cx.weak_entity();
        let profile = Profile::new().unwrap();

        let mut tabs = HashMap::new();
        let window_handle = window.window_handle().unwrap().as_raw();
        let utm = window.utm();

        // タブの復元。
        for (id, tab_data) in data.tabs.into_iter() {
            let tab = Tab::new(
                cx,
                window_handle,
                rect.clone(),
                id,
                profile.clone(),
                tab_data.location,
            )
            .with_context(|| format!("{}のタブを復元するのに失敗しました。", id))?;

            tab.update(cx, |tab, _cx| {
                let show = data.selected.is_some_and(|selected| selected == tab.id);
                tab.set_hidden(utm, !show)
            });
            tabs.insert(id, tab);
        }

        Ok(cx.new(|_cx| Self {
            workspace_list,
            profile,
            id: data.id,
            name: data.name,
            icon: data.icon,
            tabs,
            tab_order: data.tab_order,
            selected: data.selected,
            files,
        }))
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn icon(&self) -> &WorkspaceIconData {
        &self.icon
    }

    pub fn selected_tab(&self) -> Option<Uuid> {
        self.selected
    }

    pub fn select(&mut self, cx: &mut App, id: Uuid) {
        for (tab_id, tab) in self.tabs.iter() {
            tab.update(cx, |tab, cx| {
                if *tab_id == id {
                    tab.set_hidden(cx.utm(), false);
                } else {
                    tab.set_hidden(cx.utm(), true);
                }
            });
        }

        self.selected = Some(id);
    }

    pub fn tab_order(&self) -> &Vec<Uuid> {
        &self.tab_order
    }

    pub fn get_tab(&self, id: Uuid) -> Option<&Entity<Tab>> {
        self.tabs.get(&id)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn create_tab(
        &mut self,
        window: &mut Window,
        cx: &mut App,
        with_open: bool,
    ) -> anyhow::Result<()> {
        let window_handle = window.window_handle().unwrap().as_raw();
        let rect = self
            .workspace_list
            .read_with(cx, |list, cx| list.layout_state.read(cx).view_rect(window))
            .unwrap();
        let location = TabLocationData::WebPage {
            url: "https://www.google.com".to_owned(),
        };

        let tab = Tab::new(
            cx,
            window_handle,
            rect,
            Uuid::new_v4(),
            self.profile.clone(),
            location,
        )?;

        let id = tab.read(cx).id;
        if with_open {
            self.select(cx, id);
        } else {
            tab.update(cx, |tab, cx| {
                tab.set_hidden(cx.utm(), true);
            });
        }

        self.tabs.insert(id, tab);
        self.tab_order.push(id);

        Ok(())
    }

    pub fn close_tab(&mut self, cx: &mut App, utm: UIThreadMarker, id: Uuid) -> anyhow::Result<()> {
        let tab = self.tabs.remove(&id).context("タブがありませんでした。")?;

        tab.update(cx, |tab, _cx| {
            tab.close(utm)
                .context("Failed to properly close the tab.")?;

            anyhow::Ok(())
        })?;

        Ok(())
    }

    pub async fn destroy(mut self, cx: &mut App, utm: UIThreadMarker) -> anyhow::Result<()> {
        for (id, tab) in self.tabs.drain() {
            tab.update(cx, |tab: &mut Tab, _cx| tab.close(utm))
                .with_context(|| format!("Failed to close the tab {}.", id))?;
        }

        delete_workspace(cx.global::<AppPath>(), self.id)
            .await
            .context("Failed to delete the workspace data.")?;

        Ok(())
    }

    pub fn files(&self) -> &[FileSystemItem] {
        &self.files
    }
}
