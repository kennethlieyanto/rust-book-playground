mod back_of_house;
mod customer;
mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
