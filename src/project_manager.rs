// define a module by "mod" keyword
pub mod front_of_house {
    // # modules inside modules
    pub mod hosting {
        const HAMBURGER_PRICE: u8 = 20;
        const SANDWICH_PRICE: u8 = 15;

        #[derive(Debug)]
        pub enum Menu {
            Hamburger,
            Sandwich,
            New(String),
        }

        impl Menu {
            fn price(name: Menu) -> u8 {
                match name {
                    Menu::Hamburger => HAMBURGER_PRICE,
                    Menu::Sandwich => SANDWICH_PRICE,
                    _ => 0,
                }
            }
        }

        #[derive(Debug)]
        pub struct Dishes {
            pub name: Menu,
            pub price: u8,
        }

        pub fn add_to_list() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            serve_order();
            super::hosting::add_to_list();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

use front_of_house::hosting as fh;
use fh::{Menu as fhm, Dishes};

pub(crate) fn eat_at_restaurant() {
    // Absolute path
    crate::project_manager::front_of_house::hosting::add_to_list();

    // Relative path
    front_of_house::hosting::seat_at_table();

    let dishes = front_of_house::hosting::Dishes {
        name: front_of_house::hosting::Menu::New(String::from("soup")),
        price: 3,
    };
    println!("The new dishes is: {:#?}", dishes);

    println!("Hamburger in menu: {:#?}", fh::Menu::Hamburger);
}
