pub fn while_loop() {
    let mut nomor = 0;
    while nomor <= 10 {
        if nomor % 2 == 0{
            println!("Nomor: {}", nomor);
        }
        nomor += 1;
    }
}

