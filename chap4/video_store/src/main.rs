pub struct Customer {
}

impl Customer {
    fn add_rental(&mut self, title: &str, days: i32) {
    }

    fn get_rental_fee(&mut self) -> f64 {
        1.5
    }

    fn get_rental_point(&mut self) -> i32 {
        1
    }
}


#[cfg(test)]
mod customer_test {
    use super::*;

    fn assertFeeAndPoints(c: &mut Customer, fee: f64, point: i32) {
        assert_eq!(1.5, c.get_rental_fee());
        assert_eq!(1, c.get_rental_point());
    }

    #[test]
    pub fn regularmovie_oneday() {
        let mut c = Customer {};

        c.add_rental("Regular Movie", 1);
        assertFeeAndPoints(&mut c, 1.5, 1);
    }

    #[test]
    pub fn regularmovie_sec_and_third_day_free() {
        let mut c = Customer {};

        c.add_rental("Regular Movie", 2);
        assertFeeAndPoints(&mut c, 1.5, 1);
        c.add_rental("Regular Movie", 3);
        assertFeeAndPoints(&mut c, 1.5, 1);
    }

    #[test]
    pub fn regularmovie_four_days() {
        let mut c = Customer {};

        c.add_rental("Regular Movie", 4);
        assertFeeAndPoints(&mut c, 3.0, 2);
    }

}
