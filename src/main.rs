use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

#[derive(Default)]
struct Counter {
    left_value: i64,  // 左カラム用のカウンター
    right_value: i64, // 右カラム用のカウンター
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementLeft,
    DecrementLeft,
    IncrementRight,
    DecrementRight,
}

impl Counter {
    pub fn view(&self) -> Element<Message> {
        // 左カラム: 左側のカウンターとボタン
        let left_column = column![
            text("Left Counter").size(30),
            button("+").on_press(Message::IncrementLeft),
            text(self.left_value).size(50),
            button("-").on_press(Message::DecrementLeft),
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        // 右カラム: 右側のカウンターとボタン
        let right_column = column![
            text("Right Counter").size(30),
            button("+").on_press(Message::IncrementRight),
            text(self.right_value).size(50),
            button("-").on_press(Message::DecrementRight),
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        // 左カラムと右カラムを水平に並べる
        let content = row![
            left_column,
            right_column,
        ]
        .spacing(50) // 左右のカラム間のスペースを設定
        .align_y(Alignment::Center); // align_items を align_y に修正

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
            Message::IncrementLeft => {
                self.left_value += 1;
            }
            Message::DecrementLeft => {
                self.left_value -= 1;
            }
            Message::IncrementRight => {
                self.right_value += 1;
            }
            Message::DecrementRight => {
                self.right_value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Two Counters", Counter::update, Counter::view)
}
