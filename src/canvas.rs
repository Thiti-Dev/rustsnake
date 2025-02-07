// Importation
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};
// ────────────────────────────────────────────────────────────────────────────────

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap(); // get rendering context

        // how many size does it changed => results in 1.25x 1.50x .... bla bla bla
        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;
        // ─────────────────────────────────────────────────────────────────

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        //
        // ─── THESE SHOULDNT HAPPEN ───────────────────────────────────────
        // @desc assertion checking if outing the scope/logic
        assert!(x < self.width);
        assert!(y < self.height);
        // ────────────────────────────────────────────────────────────────────────────────

        self.ctx.set_fill_style_color(color);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("#e9f5f3");
        self.ctx.fill_rect(
            0.0,
            0.0,
            //
            // ─── REVERSING BACK TO THE ABSOLUTE WIDTH/HEIGHT ──────────────────────────────────────────────
            //
            f64::from(self.width * self.scaled_width),
            f64::from(self.height * self.scaled_height),
            // ─────────────────────────────────────────────────────────────────
        );
    }
}
