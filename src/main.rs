use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Reset, // リセット用のメッセージを追加
}

impl Counter {
    pub fn view(&self) -> Element<Message> {
        // 新しい説明テキストを含むカウンターとボタンを垂直に配置
        let content = column![
            text("This is a simple counter:").size(30), // 説明用のテキストを追加
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset), // リセットボタンを追加
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        // コンテナでラップし、画面の中央に配置
        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Reset => {
                self.value = 0; // リセットボタンでカウンターをゼロにする
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
