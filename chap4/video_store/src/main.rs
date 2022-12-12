use std::collections::HashMap;

#[derive(PartialEq)]
enum VideoType {
    REGULAR,
    CHILDRENS,
}

use crate::VideoType::*;

pub struct VideoRegistory {
    video_registory: HashMap<String, VideoType>,
}

impl VideoRegistory {
    fn default() -> Self {
        Self {video_registory: HashMap::from ([
                ("RegularMovie".to_string(), REGULAR),
                ("ChildrenMovie".to_string(), CHILDRENS),
             ])}
    }

    fn get_type(&mut self, title: &String) -> VideoType {
        match self.video_registory.get(title) {
            Some(REGULAR) => REGULAR,
            Some(CHILDRENS) => CHILDRENS,
            None => CHILDRENS,
        }
    }

    fn add_movie(&mut self, title: String, video_type: VideoType) {
        self.video_registory.insert(title, video_type);
    }
}

pub struct Rental {
    title: String,
    days: i32,
    video_type: VideoType,
}

impl Rental {
    fn default(title: &str, days: i32) {
        let mut video_registory = VideoRegistory::default();
        Self {title: title.to_string(), days: days,
              video_type: video_registory.get_type(&title.to_string())};
    }

    /*
    fn get_title(&mut self) -> String {
        return self.title;
    }

    fn get_type(&mut self) -> VideoType {
        return self.video_type;
    }
     */

    fn get_fee(&mut self) -> i32 {
        let mut fee: i32 = 0;
        let mut video_registory = VideoRegistory::default();

        match video_registory.get_type(&self.title)
        {
            REGULAR => fee = fee + Rental::apply_grace_period(150, self.days, 3),
            CHILDRENS => fee = fee + self.days * 100,
        }
        fee
    }

    fn get_point(&mut self) -> i32 {
        let mut point: i32 = 0;
        let mut video_registory = VideoRegistory::default();

        match video_registory.get_type(&self.title)
        {
            REGULAR => point = point + Rental::apply_grace_period(1, self.days, 3),
            CHILDRENS => point = point + 1,
        }
        point
    }

    fn apply_grace_period(amount: i32, days: i32, grace: i32) -> i32 {
        if days > grace {
            return amount + amount * (days - grace);
        }
        amount
    }
}

pub struct Customer {
    rentals: Vec<Rental>,
}

impl Customer {
    fn default() -> Self {
        Self {rentals: Vec::new()}
    }

    fn add_rental(&mut self, title: &str, days: i32) {
        let mut video_registory = VideoRegistory::default();
        self.rentals.push(Rental {title: title.to_string(), days: days,
                                  video_type: video_registory.get_type(&title.to_string())});
    }

    fn get_rental_fee(&mut self) -> i32 {
        let mut fee: i32 = 0;
        let mut video_registory = VideoRegistory::default();

        for rental in &self.rentals {
            match video_registory.get_type(&rental.title)
            {
                REGULAR => fee = fee + Customer::apply_grace_period(150, rental.days, 3),
                CHILDRENS => fee = fee + rental.days * 100,
            }
        }
        fee
    }

    fn get_rental_point(&mut self) -> i32 {
        let mut point: i32 = 0;
        let mut video_registory = VideoRegistory::default();

        for rental in &self.rentals {
            match video_registory.get_type(&rental.title)
            {
                REGULAR => point = point + Customer::apply_grace_period(1, rental.days, 3),
                CHILDRENS => point = point + 1,
            }
        }
        point
    }

    fn apply_grace_period(amount: i32, days: i32, grace: i32) -> i32 {
        if days > grace {
            return amount + amount * (days - grace);
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
    pub fn regularmovie_sec_day_free() {
        let mut c = Customer::default();
        c.add_rental("RegularMovie", 2);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_third_day_free() {
        let mut c = Customer::default();
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

    #[test]
    pub fn childrens_movie_four_days() {
        let mut c = Customer::default();
        c.add_rental("ChildrenMovie", 4);
        assert_fee_and_points(&mut c, 400, 1); // 400 is for using 3 at get_rental_point
    }

    #[test]
    pub fn one_regular_one_childrens_four_days() {
        let mut c = Customer::default();
        c.add_rental("RegularMovie", 4); // $3 + 2p
        c.add_rental("ChildrenMovie", 4); // $4 + 1p
        assert_fee_and_points(&mut c, 700, 3);
    }

}
