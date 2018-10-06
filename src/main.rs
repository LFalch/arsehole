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
use arsehole::game::Game;

const CARD_WIDTH: f32 = 72.;
const CARD_HEIGHT: f32 = 96.;

pub struct MainState {
    text: graphics::Text,
    cards: graphics::Image,
    game: Game,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/FiraMono.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;
        let cards = graphics::Image::new(ctx, "/cards.png")?;

        let mut game = Game::with_jokers();
        game.add_player("Falch");
        game.add_player("Dummer");
        game.shuffle();
        game.deal_to_all(5);

        Ok(MainState {
            text,
            cards,
            game
        })
    }
}

impl event::EventHandler for MainState {
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        let x = x as f32;
        let y = y as f32;

        if let event::MouseButton::Left = button {
            if x >= 350. && x < 350. + CARD_WIDTH && y >= 100. && y < 100. + CARD_HEIGHT {
                self.game.draw(0);
            }

            if x > 200. {
                let p = (y - (440. - 99. * (self.game.players.len() - 1) as f32)) as usize / 99;
                if p < self.game.players.len() {
                    let i = (x - 200.) as usize / 50;

                    if i < self.game.players[p].hand.len() {
                        self.game.play(p, i);
                    }
                }
            }
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;

        let mut deck_size = self.game.deck.len()/5;
        if deck_size == 0 && self.game.deck.len() != 0 {
            deck_size = 1;
        }

        for i in 0..deck_size {
            draw_card(BLUE_BACK, ctx, self, 350.+(i as f32 * 5.), 100.)?;
        }
        for (i, card) in self.game.pile.iter().enumerate() {
            card.draw(ctx, self, 150.+(i as f32 * 0.5), 100.)?;
        }
        for (p, player) in self.game.players.iter().enumerate() {
            for (i, card) in player.hand.iter().enumerate() {
                card.draw(ctx, self, 200.+(i as f32 * 50.), 440.-(p as f32 * 99.))?;
            }
        }

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

pub trait Draw {
    fn draw(&self, &mut Context, &MainState, f32, f32) -> GameResult<()>;
}

pub const RED_BACK: (u8, u8) = (13, 0);
pub const BLUE_BACK: (u8, u8) = (13, 1);

pub fn draw_card((cx, cy): (u8, u8), ctx: &mut Context, ms: &MainState, x: f32, y: f32) -> GameResult<()> {
    const SRC_W: f32 = 1. / 14.;
    const SRC_H: f32 = 1. / 4.;

    let src_x = cx as f32 / 14.;
    let src_y = cy as f32 / 4.;
    ms.cards.draw_ex(ctx, graphics::DrawParam {
        src: graphics::Rect{x: src_x, y: src_y, w: SRC_W, h: SRC_H},
        dest: Point2::new(x, y),
        .. graphics::DrawParam::default()
    })
}

impl Draw for Card {
    fn draw(&self, ctx: &mut Context, ms: &MainState, x: f32, y: f32) -> GameResult<()> {
        let suit;

        if let Rank::Joker = self.rank {
            if self.suit.is_black() {
                suit = 2
            } else {
                suit = 3;
            }
        } else {
            suit = self.suit as u8;
        }

        draw_card((self.rank as u8, suit), ctx, ms, x, y)
    }
}
