use std::collections::HashMap;

#[derive(PartialEq)]
enum VideoType {
    REGULAR,
    CHILDRENS,
}

pub struct Customer {
    title: String,
    days: i32,
    movie_registory: HashMap<String, VideoType>,
}

use crate::VideoType::*;

impl Customer {
    fn default() -> Self {
        Self {days: 3, title:String::from(""),
                movie_registory: HashMap::from ([
                ("RegularMovie".to_string(), REGULAR),
                ("ChildrenMovie".to_string(), CHILDRENS),
             ])}
    }

    fn add_rental(&mut self, title: &str, days: i32) {
        self.days = days;
        self.title = title.to_string();
    }

    fn get_rental_fee(&mut self) -> i32 {
        match self.movie_registory.get(&self.title)
        {
            Some(REGULAR) => self.apply_grace_period(150, 3),
            Some(CHILDRENS) => 100,
            None => 0,
        }
    }

    fn get_rental_point(&mut self) -> i32 {
        self.apply_grace_period(1, 3)
    }

    fn apply_grace_period(&mut self, amount: i32, grace: i32) -> i32 {
        if self.days > grace {
            return amount + amount * (self.days - grace);
        }
        amount
    }
}


#[cfg(test)]
mod customer_test {
    use super::*;

    fn assert_fee_and_points(c: &mut Customer, fee: i32, points: i32) {
        assert_eq!(fee, c.get_rental_fee());
        assert_eq!(points, c.get_rental_point());
    }

    #[test]
    pub fn regularmovie_oneday() {
        let mut c = Customer::default();

        c.add_rental("RegularMovie", 1);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_sec_and_third_day_free() {
        let mut c = Customer::default();

        c.add_rental("RegularMovie", 2);
        assert_fee_and_points(&mut c, 150, 1);
        c.add_rental("RegularMovie", 3);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_four_days() {
        let mut c = Customer::default();

        c.add_rental("RegularMovie", 4);
        assert_fee_and_points(&mut c, 300, 2);
    }

    #[test]
    pub fn childrens_movie_one_days() {
        let mut c = Customer::default();

        c.add_rental("ChildrenMovie", 1);
        assert_fee_and_points(&mut c, 100, 1);
    }

}
