//Parameter: Variable yang menjadi bagian dari definisi functionnya. Wajib kita sebutkan tipe data.
//Wajib juga kita beri value untuk parameter tersebut ketika dipanggil.
pub fn katakan_cis(kata_pertama: &str, kata_kedua: &str) {
    println!("{} {}", kata_pertama, kata_kedua);
}

pub fn test_parameter() {
    katakan_cis("Katakan", "CISS");
    katakan_cis("Aku", "gay");
    katakan_cis("SUka", "Kholi");
    katakan_cis("Mimpi", "Oca");
}


