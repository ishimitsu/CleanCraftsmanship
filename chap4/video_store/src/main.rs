pub struct Customer {
    days: i32,
}

impl Customer {
    fn add_rental(&mut self, title: &str, days: i32) {
        self.days = days;
    }

    fn get_rental_fee(&mut self) -> i32 {
        self.apply_grace_period(150, 3)
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
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 1);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_sec_and_third_day_free() {
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 2);
        assert_fee_and_points(&mut c, 150, 1);
        c.add_rental("Regular Movie", 3);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_four_days() {
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 4);
        assert_fee_and_points(&mut c, 300, 2);
    }

}
