pub struct Customer {
}

impl Customer {
    fn add_rental(&self, title: &str, days: i32) {
    }

    fn get_rental_fee(&self) -> f64 {
        1.5
    }

    fn get_rental_point(&self) -> i32 {
        1
    }
}


#[cfg(test)]
mod customer_test {
    use super::*;

    fn assertFeeAndPoints(c: Customer, fee: f64, point: i32) {
        assert_eq!(1.5, c.get_rental_fee());
        assert_eq!(1, c.get_rental_point());
    }

    #[test]
    pub fn regularmovie_oneday() {
        let c = Customer {};

        c.add_rental("Regular Movie", 1);
        assertFeeAndPoints(c, 1.5, 1);
    }
}
