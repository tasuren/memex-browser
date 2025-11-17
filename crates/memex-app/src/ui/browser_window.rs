use gpui::{App, Entity, ReadGlobal, Window, prelude::*};
use memex_backend::{
    LayoutState, WorkspaceListState,
    data::{
        AppPath, WorkspaceMetadata, create_workspace, get_workspace_list, list_workspaces,
        load_workspace,
    },
};

use crate::ui::{
    EXPROLER_WIDTH, Start, TOP_TAB_BAR_HEIGHT, TitleBar, URL_BAR_HEIGHT, WORKSPACE_LIST_WIDTH,
    Workbench, WorkspaceList,
};

#[derive(Clone)]
pub enum CurrentView {
    Start(Entity<Start>),
    Workbench(Entity<Workbench>),
}

/// ブラウザのウィンドウのUIの根本。
/// 起動時にワークスペース一覧を読み込んで、読み込めればそれをUIに反映する。
pub struct BrowserWindow {
    current: CurrentView,
}

impl BrowserWindow {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let entity = cx.new(|cx| Self {
            current: CurrentView::Start(Start::new(cx)),
        });

        // ワークベンチの読み込みを行う。
        window
            .spawn(cx, {
                let entity = entity.clone();
                let path = AppPath::global(cx).clone();

                async move |cx| {
                    let workspace_list_data = get_workspace_list(&path)
                        .await
                        .expect("ワークスペースリストの状態の復元に失敗しました。");
                    let mut workspaces = list_workspaces(&path)
                        .await
                        .expect("ワークスペースのリストの読み込みに失敗しました。");

                    // もしワークスペースがまだ一つもないなら、最低限ホームが必要なので作成する。
                    let (home, home_files) = if workspaces.is_empty() {
                        let home =
                            create_workspace(&path, workspace_list_data.home, "Home".to_owned())
                                .await
                                .expect("ホームワークスペースの作成に失敗しました。");

                        workspaces.insert(workspace_list_data.home, WorkspaceMetadata::from(&home));

                        (home, Vec::new())
                    } else {
                        load_workspace(&path, workspace_list_data.home)
                            .await
                            .expect("ホームワークスペースの読み込みに失敗しました。")
                    };

                    entity
                        .update_in(cx, move |browser_window, window, cx| {
                            let layout_state = cx.new(|_| LayoutState {
                                top_tab_bar_height: TOP_TAB_BAR_HEIGHT,
                                url_bar_height: URL_BAR_HEIGHT,
                                workspace_list_width: WORKSPACE_LIST_WIDTH,
                                exproler_width: EXPROLER_WIDTH,
                            });

                            let workspace_list_state = WorkspaceListState::new(
                                window,
                                cx,
                                layout_state.clone(),
                                workspace_list_data,
                                workspaces,
                                home,
                                home_files,
                            )
                            .unwrap();
                            let workspace_list = WorkspaceList::new(
                                cx,
                                workspace_list_state.clone(),
                                layout_state.clone(),
                            );

                            let current = workspace_list_state.read(cx).current().clone();
                            let title_bar =
                                TitleBar::new(window, cx, layout_state.clone(), current);

                            let workbench =
                                Workbench::new(cx, layout_state, workspace_list, title_bar);
                            browser_window.current = CurrentView::Workbench(workbench);

                            cx.notify();
                        })
                        .unwrap();
                }
            })
            .detach();

        entity
    }
}

impl Render for BrowserWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.current.clone() {
            CurrentView::Start(start) => start.into_any_element(),
            CurrentView::Workbench(workbench) => workbench.into_any_element(),
        }
    }
}
