pub struct Titik<Y> {
    a: Y,
    b: Y,
}

pub fn coba_generic_struct() {
    let integer = Titik::<i32> { a: 5, b: 2 };
    let float = Titik::<f32> { a: 1.2, b: 3.1 };

    println!("Integer: {}, {}", integer.a, integer.b);
    println!("Float: {}, {}", float.a, float.b);
}
