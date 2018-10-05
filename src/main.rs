extern crate ggez;
extern crate arsehole;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Drawable, Point2};
use ggez::{Context, GameResult};
use std::env;
use std::path;
use arsehole::*;

struct MainState {
    text: graphics::Text,
    cards: graphics::Image,
    deck: Vec<Card>,
    frame: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/FiraMono.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;
        let cards = graphics::Image::new(ctx, "/cards.png")?;

        let deck = (0..54).map(|n| Card::from(n)).collect::<Vec<_>>();

        let s = MainState { text, cards, deck, frame: 0};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;

        let card = self.deck[self.frame / 25 % self.deck.len()];

        card.draw(ctx, self, 10., 150.)?;

        self.frame += 1;

        graphics::present(ctx);

        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = "Arsehole".to_owned();
    let ctx = &mut Context::load_from_conf("arsehole", "LFalch", c).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        eprintln!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

trait Draw {
    fn draw(&self, &mut Context, &MainState, f32, f32) -> GameResult<()>;
}

impl Draw for Card {
    fn draw(&self, ctx: &mut Context, ms: &MainState, x: f32, y: f32) -> GameResult<()> {
        const SRC_W: f32 = 1. / 14.;
        const SRC_H: f32 = 1. / 4.;
        let src_x = self.rank as u8 as f32 / 14.;
        let src_y = self.suit as u8 as f32 / 4.;
        ms.cards.draw_ex(ctx, graphics::DrawParam {
            src: graphics::Rect{x: src_x, y: src_y, w: SRC_W, h: SRC_H},
            dest: Point2::new(x, y),
            .. graphics::DrawParam::default()
        })
    }
}
