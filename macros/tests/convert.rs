use web3_rush_macros::{struct_original_mapping, tuple_struct_original_mapping};

#[derive(Debug)]
pub struct Fuga {
    pub a: String,
    pub b: FugaChild
}

#[derive(Debug)]
pub struct FugaChild {
    c: Option<String>,
    d: u64
}

#[struct_original_mapping(FugaChild)]
#[derive(Debug)]
pub struct HogeChild {
    c: Option<String>,
    d: u64
}

#[struct_original_mapping(Fuga)]
#[derive(Debug)]
pub struct Hoge {
    a: String,
    b: HogeChild
}

#[derive(Debug)]
pub struct Fugo {
    a: String,
    b: u64
}

pub type Fugo2 = Fugo;

#[tuple_struct_original_mapping(Fugo)]
pub struct Hage(pub Fugo);


fn main(){
    let f = Fuga{ a: "test".to_owned(), b: FugaChild { c: Some("test".to_owned()), d: 0 } };
    let h: Hoge = f.into();
    let f2: Fuga = h.into();
    println!("{}", f2.b.d);


    // let fugo: Fugo2 = Fugo2{a: "fugo".to_owned(), b: 0};
    // let hage: Hage = fugo.into();
    // let fugo2: Fugo2 = hage.into();
    // println!("{}", fugo2.a);
}