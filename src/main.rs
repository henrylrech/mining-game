use rat::App;
mod rat;
mod ore;
mod shop;

fn main() {
    let terminal = ratatui::init();
    let app = App::default();
    let _ = app.run(terminal);
    ratatui::restore();
}