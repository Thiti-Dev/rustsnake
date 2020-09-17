use crate::pixel::Pixel;

use stdweb::unstable::TryInto;

#[derive(Debug, Eq, PartialEq)]
pub struct Food {
    pub foods: Vec<Pixel>,
}

impl Food {
    pub fn new() -> Self {
        Food { foods: Vec::new() }
    }
    pub fn gen_new_food(&mut self, width: u32, height: u32) {
        let food_x: u32 = js! { return Math.floor(Math.random() * @{width}) }
            .try_into()
            .unwrap();
        let food_y: u32 = js! { return Math.floor(Math.random() * @{height}) }
            .try_into()
            .unwrap();

        let food = Pixel(food_x, food_y);
        self.foods.push(food);
    }
    pub fn remove_food_at(&mut self, index: usize) {
        self.foods.remove(index);
    }
}
