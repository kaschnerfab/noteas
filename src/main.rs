use iced::{Center, Renderer, Theme, mouse};
use iced::widget::canvas::Program;
use iced::widget::{canvas};

pub fn main() -> iced::Result {
    iced::run(DrawNote::update, DrawNote::view)
}

#[derive(Default)]
struct Point {
    x: i64,
    y: i64
}

struct Line{
    start: Point,
    end: Point,
}

struct Stroke {
    lines: Vec<Line>,
}

#[derive(Default)]
struct DrawNote {
    strokes: Vec<Stroke>
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Start,
    End,
    Move,
}

impl DrawNote {
    fn update(&mut self, message: Message) {
    }

    fn view(&self) {
    }
}

impl<Message> Program<Message> for DrawNote{
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>>
    {
        let mut gem: Vec<canvas::Geometry<Renderer>> = Vec::new();
        gem
    }
}
