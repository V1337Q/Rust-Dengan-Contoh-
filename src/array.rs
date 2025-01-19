pub fn arayy() {
    let mut gitu: [i32; 3]= [1, 5, 7];
    let [a, b, c] = gitu;
    println!("{}, {}, {}", a, b, c);

    gitu[0] = 10;
    println!("{:?}", gitu);

    let panjang_array = gitu.len();
    println!("{:?}", panjang_array);
}

