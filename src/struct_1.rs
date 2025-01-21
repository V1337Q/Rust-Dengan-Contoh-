pub struct IdentitasSaya {
    nama : String,
    alamat: String,
    asal_sekolah: String,
    usia: u8,
}

pub impl IdentitasSaya { //self mengacu kesini. Struct IdentitasSaya.
    pub fn identitas(&self, nama: &str) { //merupakan method. Method: identitas. &self memungkinkan
        //akses ke struct IdentitasSaya. Struct dinyatakan setelah impl.
        println!("Hallo {}, alamat mu di {}", nama, self.alamat);//self.alamat mengacu pada field
        //alamat pada struct IdentitasSaya.
    }
}

pub fn cobaidentitas() {
    let user = IdentitasSaya {
    nama : String::from("Caesar"),
    alamat: String::from("StAve"),
    asal_sekolah: String::from("SD108"),
    usia: 20,
    };

    user.identitas("Caesar");
} 


