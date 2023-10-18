use druid::widget::{Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, Color, WidgetExt,
};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Four Text Boxes Example");

#[derive(Clone, Data, Lens)]
struct AppState {
    text1: String,
    text2: String,
    text3: String,
    text4: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 200.0));

    // create the initial app state
    let initial_state = AppState {
        text1: "".to_string(),
        text2: "".to_string(),
        text3: "".to_string(),
        text4: "".to_string(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let label1 = Label::new("Text Box 1:");
    let label2 = Label::new("Text Box 2:");
    let label3 = Label::new("Text Box 3:");
    let label4 = Label::new("Text Box 4:");

    let text_box1 = TextBox::new()
        .lens(AppState::text1)
        .expand_width()
        .padding(5.0)
        .background(Color::WHITE);

    let text_box2 = TextBox::new()
        .lens(AppState::text2)
        .expand_width()
        .padding(5.0)
        .background(Color::WHITE);

    let text_box3 = TextBox::new()
        .lens(AppState::text3)
        .expand_width()
        .padding(5.0)
        .background(Color::WHITE);

    let text_box4 = TextBox::new()
        .lens(AppState::text4)
        .expand_width()
        .padding(5.0)
        .background(Color::WHITE);

    let col1 = Flex::column()
        .with_child(label1)
        .with_flex_child(text_box1, 1.0);

    let col2 = Flex::column()
        .with_child(label2)
        .with_flex_child(text_box2, 1.0);

    let col3 = Flex::column()
        .with_child(label3)
        .with_flex_child(text_box3, 1.0);

    let col4 = Flex::column()
        .with_child(label4)
        .with_flex_child(text_box4, 1.0);

    Flex::row()
        .with_child(col1)
        .with_spacer(10.0)
        .with_child(col2)
        .with_spacer(10.0)
        .with_child(col3)
        .with_spacer(10.0)
        .with_child(col4)
}