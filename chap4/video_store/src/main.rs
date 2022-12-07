pub struct Customer {
    days: i32,
}

impl Customer {
    fn add_rental(&mut self, title: &str, days: i32) {
        self.days = days;
    }

    fn get_rental_fee(&mut self) -> f64 {
        let mut fee: f64 = 1.5;
        if self.days > 3 {
            fee = fee + 1.5 * (self.days as f64 - 3.0);
        }

        fee
    }

    fn get_rental_point(&mut self) -> i32 {
        let mut points: i32 = 1;
        if self.days > 3 {
            points = points + (self.days - 3)
        }

        points
    }
}


#[cfg(test)]
mod customer_test {
    use super::*;

    fn assertFeeAndPoints(c: &mut Customer, fee: f64, points: i32) {
        assert_eq!(fee, c.get_rental_fee());
        assert_eq!(points, c.get_rental_point());
    }

    #[test]
    pub fn regularmovie_oneday() {
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 1);
        assertFeeAndPoints(&mut c, 1.5, 1);
    }

    #[test]
    pub fn regularmovie_sec_and_third_day_free() {
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 2);
        assertFeeAndPoints(&mut c, 1.5, 1);
        c.add_rental("Regular Movie", 3);
        assertFeeAndPoints(&mut c, 1.5, 1);
    }

    #[test]
    pub fn regularmovie_four_days() {
        let mut c = Customer {days: 0};

        c.add_rental("Regular Movie", 4);
        assertFeeAndPoints(&mut c, 3.0, 2);
    }

}
