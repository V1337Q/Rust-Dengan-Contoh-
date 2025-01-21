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


