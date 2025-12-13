CREATE TABLE workspace (
    id CHAR(36) NOT NULL PRIMARY KEY,
    icon_type TEXT NOT NULL DEFAULT 'Default',
    icon_source TEXT,
    selected_tab CHAR(36),

    CONSTRAINT fk_selected_tab
          FOREIGN KEY (selected_tab)
          REFERENCES tab(id)
          ON DELETE SET NULL
);

CREATE TABLE tab (
    id CHAR(36) NOT NULL PRIMARY KEY,
    workspace_id CHAR(36) NOT NULL,
    location_type TEXT NOT NULL,
    location_source TEXT,

    FOREIGN KEY (workspace_id)
        REFERENCES workspace(id)
        ON DELETE CASCADE
);
