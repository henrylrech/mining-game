use rat::App;
mod rat;
mod ore;

fn main() {
    // let mut money = 0;

    // let mut ores = get_ores();

    // loop {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("error: unable to read user input");

    //     match input.as_str().trim() {
    //         "1" => {
    //             let coal = &mut ores[0];
    //             coal.mine(&mut money);

    //             println!("{} {} {}", coal.name, coal.count, money);
    //         },
    //         "2" => {
    //             let iron = &mut ores[1];
    //             iron.mine(&mut money);

    //             println!("{} {} {}", iron.name, iron.count, money);
    //         },
    //         "3" => {
    //             let diamond = &mut ores[2];
    //             diamond.mine(&mut money);

    //             println!("{} {} {}", diamond.name, diamond.count, money);
    //         },
    //         _ => println!("unknown")
    //     }
    // }

    let terminal = ratatui::init();
    let _ = App::default().run(terminal);
    ratatui::restore();
}