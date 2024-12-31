use rat::App;
mod rat;
mod ore;
mod shop;

fn main() {
    let terminal = ratatui::init();
    let _ = App::default().run(terminal);
    ratatui::restore();
}