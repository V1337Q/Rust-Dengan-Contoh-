pub fn rekursiv(value: String, times: u32) {
    if times == 0 {
        return;
    }else {
        println!("{}", value);
    }
    
    rekursiv(value, times -1);
}

pub fn coba_rekursi() {
    rekursiv(String::from("Kai"), 10);
}

pub fn faktorial_rekursiv(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * faktorial_rekursiv(n - 1)
}
pub fn faktorekur() {
    let hasil = faktorial_rekursiv(5);

    println!("{}", hasil);
}

