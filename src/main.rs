use iced::{Center, Color, Event, Rectangle, Renderer, Theme, mouse, Point};
use iced::widget::canvas::{Program, Path, Action};
use iced::widget::{canvas};
use iced::mouse::{Cursor};

pub fn main() -> iced::Result {
    iced::run(DrawNote::update, DrawNote::view)
}

struct Stroke {
    points: Vec<Point<f32>>,
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

impl Stroke {
    fn new() -> Self {
        Stroke { points: Vec::new() }
    }
}

impl DrawNote {
    fn update(&mut self, message: Message) {
        match message {
            Message::Start(point) => {
                let mut new_stroke = Stroke::new(); 
                new_stroke.points.push(point);
                self.strokes.push(new_stroke);
            }
            Message::Move(point) => {
                if let Some(stroke) = self.strokes.last_mut() {
                    stroke.points.push(point);
                }
            }
            Message::End => {
                if let Some(stroke) = self.strokes.last() {
                    println!("Stroke length: {:?}", stroke.points.len());
                }
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        canvas(self)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }
}

impl Program<Message> for DrawNote{
    type State = NoteState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
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

        for stroke in &self.strokes {
            if stroke.points.len() < 1 {
                continue;
            }
            for i in 1..stroke.points.len() {
                frame.stroke(
                    &canvas::Path::line(
                        stroke.points[i - 1],
                        stroke.points[i]
                    ),
                    canvas::Stroke::default(),
                );   
            }
        }

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
                return Some(canvas::Action::publish(Message::Start(cursor_position)))
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.is_drawing = false;
                return Some(canvas::Action::publish(Message::End))
            }

            Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                if !state.is_drawing {
                    return None
                }
                return Some(canvas::Action::publish(Message::Move(cursor_position)))
            }

            _ => {}
        }

        None
    }
}
