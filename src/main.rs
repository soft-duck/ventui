mod config;
mod tui;

fn main() {
    let mut terminal = ratatui::init();
    let app_result = tui::TUI::default().run(&mut terminal);
    ratatui::restore();
    if let Err(e) = app_result {
        print!("{e}")
    }
}
