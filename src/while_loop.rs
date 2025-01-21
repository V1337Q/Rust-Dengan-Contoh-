pub fn while_loop() {
    let mut nomor = 0;
    while nomor <= 10 {
        if nomor % 2 == 0{
            println!("Nomor: {}", nomor);
        }
        nomor += 1;
    }
}
pub fn while_array() {
    let array = ["A", "B", "C"];
    let mut numbering = 0;
    let mut index = 0;
    while index < array.len() {
        println!("Nilai ke {} : {}", numbering, array[index]);
        index +=  1;
        numbering += 1;
    }

}

