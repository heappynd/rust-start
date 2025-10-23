use csv_to_json::{functions::read_csv, models::structs::HousePrice};

fn main() {
    // crate::m1::m2::f();
    // m1::m2::f();
    let y = csv_to_json::models::enums::YesNo::Yes;
    let house_price = HousePrice {
        price: 250000,
        area: String::from("Downtown"),
        bed_rooms: 3,
        main_road: y,
    };
    println!("House Price: {}", house_price.price);

    read_csv(String::from(r"C:\a\c.csv"));
}

// mod models {}

mod m1 {
    pub mod m2 {
        pub fn f() {
            println!("Hello from m2::f");
        }
    }
}

mod x1 {
    fn method3() {
        // x2::g();
        self::x2::g();
    }

    mod x2 {
        pub fn g() {
            super::super::m1::m2::f();
        }
    }
}
