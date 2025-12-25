pub struct Breakfast {
    pub toast: String,
    season_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}

pub enum Appetizer {
    Soup,
    Salad,
}
