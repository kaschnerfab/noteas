use iced::{Center, Renderer, Theme, mouse, Color};
use iced::widget::canvas::{Program, Path};
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

    fn view(&self) -> iced::Element<Message> {
        canvas(self)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let background = Path::rectangle(
            iced::Point::ORIGIN,
            frame.size(),
        );

        frame.fill(
            &background,
            Color::WHITE,
        );

        /*frame.stroke(
            &canvas::Path::line(
                iced::Point::new(10.0, 10.0),
                iced::Point::new(100.0, 100.0),
            ),
            canvas::Stroke::default(),
        );*/

        vec![frame.into_geometry()]
    }
}
