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

pub struct Customer {
    title: String,
    days: i32,
}

impl Customer {
    //fn default() -> Self { Self {days: 3, title:String::from("")} }

    fn add_rental(&mut self, title: &str, days: i32) {
        self.days = days;
        self.title = title.to_string();
    }

    fn get_rental_fee(&mut self) -> i32 {
        let mut video_registory = VideoRegistory::default();
        match video_registory.get_type(&self.title)
        {
            REGULAR => self.apply_grace_period(150, 3),
            CHILDRENS => 100,
        }
    }

    fn get_rental_point(&mut self) -> i32 {
        let mut video_registory = VideoRegistory::default();
        match video_registory.get_type(&self.title)
        {
            REGULAR => self.apply_grace_period(1, 3),
            CHILDRENS => 1,
        }
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
        let mut c = Customer{days: 3, title:String::from("")};

        c.add_rental("RegularMovie", 1);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_sec_and_third_day_free() {
        let mut c = Customer{days: 3, title:String::from("")};

        c.add_rental("RegularMovie", 2);
        assert_fee_and_points(&mut c, 150, 1);
        c.add_rental("RegularMovie", 3);
        assert_fee_and_points(&mut c, 150, 1);
    }

    #[test]
    pub fn regularmovie_four_days() {
        let mut c = Customer{days: 3, title:String::from("")};

        c.add_rental("RegularMovie", 4);
        assert_fee_and_points(&mut c, 300, 2);
    }

    #[test]
    pub fn childrens_movie_one_days() {
        let mut c = Customer{days: 3, title:String::from("")};

        c.add_rental("ChildrenMovie", 1);
        assert_fee_and_points(&mut c, 100, 1);
    }

    #[test]
    pub fn childrens_movie_four_days() {
        let mut c = Customer{days: 3, title:String::from("")};

        c.add_rental("ChildrenMovie", 4);
        assert_fee_and_points(&mut c, 400, 1); // 400 is for using 3 at get_rental_point
    }

}
