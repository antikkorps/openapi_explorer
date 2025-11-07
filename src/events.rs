use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    Quit,
    NextPanel,
    PreviousPanel,
    Search(String),
    ClearSearch,
    Select,
    Back,
    ChangeView(View),
    Reload,
    ToggleHelp,
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    ShowEndpointDetails,
    HideEndpointDetails,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Fields,
    Schemas,
    Endpoints,
    Graph,
    Stats,
}

pub fn handle_key_event(key: KeyEvent) -> Option<AppEvent> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('c')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            Some(AppEvent::Quit)
        }
        KeyCode::Char('q') => Some(AppEvent::Quit),
        KeyCode::Tab => Some(AppEvent::NextPanel),
        KeyCode::BackTab => Some(AppEvent::PreviousPanel),
        KeyCode::Char('/') => Some(AppEvent::ClearSearch),
        KeyCode::Enter => Some(AppEvent::Select),
        KeyCode::Esc => Some(AppEvent::Back),
        KeyCode::Char('h') => Some(AppEvent::ToggleHelp),
        KeyCode::Char('r') => Some(AppEvent::Reload),
        KeyCode::Char('1') => Some(AppEvent::ChangeView(View::Fields)),
        KeyCode::Char('2') => Some(AppEvent::ChangeView(View::Schemas)),
        KeyCode::Char('3') => Some(AppEvent::ChangeView(View::Endpoints)),
        KeyCode::Char('4') => Some(AppEvent::ChangeView(View::Graph)),
        KeyCode::Char('5') => Some(AppEvent::ChangeView(View::Stats)),
        KeyCode::Up => Some(AppEvent::NavigateUp),
        KeyCode::Down => Some(AppEvent::NavigateDown),
        KeyCode::Left => Some(AppEvent::NavigateLeft),
        KeyCode::Right => Some(AppEvent::NavigateRight),
        KeyCode::Char(ch) => Some(AppEvent::Search(ch.to_string())),
        KeyCode::Backspace => Some(AppEvent::Search(String::new())), // Will be handled as backspace
        _ => None,
    }
}
