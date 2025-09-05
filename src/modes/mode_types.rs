#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Normal,
    Editing,
    Naming,
    Renaming,
    ChangingDirectory,
    SelectingTemplateFolder,
    SelectingTemplate,
    Search,
    ConfirmingDelete,
    SelectingMoveDestination,
    Settings,
}

impl Mode {
    pub fn to_string(&self) -> &str {
        match self {
            Mode::Normal => "NAVIGATE",
            Mode::Editing => "EDITING",
            Mode::Naming => "NAMING",
            Mode::Renaming => "RENAMING",
            Mode::ChangingDirectory => "CHANGE DIR",
            Mode::SelectingTemplateFolder => "SELECT TMPL DIR",
            Mode::SelectingTemplate => "SELECT TMPL",
            Mode::Search => "SEARCH",
            Mode::ConfirmingDelete => "CONFIRM DELETE",
            Mode::SelectingMoveDestination => "SELECT MOVE DEST",
            Mode::Settings => "SETTINGS",
        }
    }
}