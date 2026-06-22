mod drawnote;

use drawnote::drawnote::DrawNote;


#[derive(Default)]
struct App {
    draw_note: DrawNote,
}


impl App {
    fn view(&self) -> iced::Element<Message>{
        self.draw_note
            .draw_view()
            .map(Message::DrawMessage)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DrawMessage(draw_message) => {
                self.draw_note.update(draw_message);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DrawMessage(drawnote::drawnote::DrawMessage),
}

pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}