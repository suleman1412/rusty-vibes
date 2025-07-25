// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    pub const OUTER_SULEMAN:&str = self::fruits::INNER_SULEMAN;

    mod fruits {
        pub const INNER_SULEMAN:&str = "suleman";
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {} \n Name: {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
        delicious_snacks::OUTER_SULEMAN
    );
}
