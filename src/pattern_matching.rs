pub enum Level {
    A1,
    A2,
    B1,
    B2,
}

pub fn match_level() {
    let level = Level::B2;
    match level {
        Level::A1 => {
            println!("A Eins");
        }
        Level::A2 => {
            println!("A Zwei");
        }
        Level::B1 => {
            println!("B Eins");
        }
        Level::B2 => {
            println!("B Zwei");
        }
    }
}

