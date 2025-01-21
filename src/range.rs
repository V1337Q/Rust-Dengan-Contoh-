pub fn range() {

    let range = 0..5;
    println!("Mulai (inclusive): {}", range.start);
    println!("Akhir (exvlusive/exlude): {}", range.end);
    
    for x in range {
        println!("{}", x * 2);
    }
}

