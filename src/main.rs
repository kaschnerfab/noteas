mod drawnote;

use drawnote::drawnote::DrawNote;

pub fn main() -> iced::Result {
    iced::run(DrawNote::update, DrawNote::view)
}