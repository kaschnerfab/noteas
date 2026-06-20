use iced::{Center, Color, Event, Rectangle, Renderer, Theme, mouse, Point};
use iced::widget::canvas::{Program, Path};
use iced::widget::{Action, canvas};
use iced::mouse::{Cursor};

pub fn main() -> iced::Result {
    iced::run(DrawNote::update, DrawNote::view)
}

struct Stroke {
    lines: Vec<Point<f32>>,
}

#[derive(Default)]
struct DrawNote {
    strokes: Vec<Stroke>
}

#[derive(Default)]
struct NoteState {
    is_drawing: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Start(Point<f32>),
    End,
    Move(Point<f32>),
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
    type State = NoteState;

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
                Point::new(10.0, 10.0),
                Point::new(100.0, 100.0),
            ),
            canvas::Stroke::default(),
        );*/

        vec![frame.into_geometry()]
    }
    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<Message>>{
        let cursor_position = cursor.position_in(bounds)?;
        
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.is_drawing = true;
                println!("mouse down at {:?}", cursor_position);
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.is_drawing = false;
                println!("mouse up at {:?}", cursor_position);
            }

            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if !state.is_drawing {
                    return None
                }
                println!("mouse moved at position {:?}", cursor_position);
                println!("mouse moved at point {:?}", position);
            }

            _ => {}
        }

        None
    }
}
