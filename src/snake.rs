use crate::canvas::Canvas;
use crate::direction::Direction;
use crate::food::Food;
use crate::pixel::Pixel;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;

pub struct Snake {
    head: Pixel,
    tails: Vec<Pixel>,
    food: Food,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    previous_direction: Direction,
    score: u32,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x: u32 = js! {return Math.floor(Math.random() * @{width})}
            .try_into()
            .unwrap();

        let head_y: u32 = js! {return Math.floor(Math.random() * @{height})}
            .try_into()
            .unwrap();

        let head = Pixel(head_x, head_y);

        //Create food entity and gen atleast 1 food inside of the vec
        let mut food = Food::new();
        food.gen_new_food(width, height);
        food.gen_new_food(width, height);
        // ────────────────────────────────────────────────────────────────────────────────

        let tails = Vec::new();

        Snake {
            head,
            tails,
            food: food,
            height,
            width,
            direction: None,
            next_direction: None,
            previous_direction: Direction::Right,
            score: 0,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.previous_direction.is_opposite(direction) && self.direction.is_none() {
            self.direction = Some(direction)
        } else if self.direction.iter().any(|d| !d.is_opposite(direction)) {
            self.next_direction = Some(direction)
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.previous_direction);
        self.previous_direction = direction;

        let new_head = match direction {
            Direction::Up => Pixel(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            Direction::Down => Pixel((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Right => Pixel((self.head.0 + 1) % self.width, (self.head.1) % self.height),
            Direction::Left => Pixel(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
        };

        self.tails.insert(0, self.head);
        let last_end = self.tails.pop();

        if self.tails.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }
        self.head = new_head;
        self.check_if_is_eating(last_end);

        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "red");
        for &Pixel(x, y) in &self.tails {
            canvas.draw(x, y, "yellow");
        }
        for food in self.food.foods.iter() {
            canvas.draw(food.0, food.1, "green");
        }
        //canvas.draw_text(1f64, 1f64, "Thiti");
        let score_txt = document().query_selector("#score_txt").unwrap().unwrap();
        // Using js-macro
        js! {
            @{score_txt}.innerHTML = "<h2>Score: " + @{self.score.to_string()}+ " </h2>";
        };
    }

    //
    // ─── PRIVATE ────────────────────────────────────────────────────────────────────
    //
    fn check_if_is_eating(&mut self, last_end: Option<Pixel>) {
        for (index, food) in self.food.foods.clone().iter().enumerate() {
            if self.head == *food {
                while *food == self.head || self.tails.contains(&food) {
                    js! {console.log("Removing food at index", @{index.to_string()})};
                    if index < self.food.foods.len() {
                        self.food.remove_food_at(index);
                        js! {console.log("Food removed, generating new food")};
                        self.food.gen_new_food(self.width, self.height);
                        self.gain_score(); // gain the score
                        break; // Was Missing this <= it messed me up for 2hrs lollllllllllllllll!!
                    } else {
                        // @TODO Handling the error
                        // @desc this will never happen
                    }
                }
                last_end.map(|x| self.tails.push(x));
            }
        }
    }

    fn gain_score(&mut self) {
        self.score += 10;
        js! {console.log("Current Score: " , @{self.score})}
    }
    // ────────────────────────────────────────────────────────────────────────────────
}
