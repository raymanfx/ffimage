use std::{env, io, io::Read};

use iced::{
    executor,
    widget::{column, container, image, text::Text},
    Application, Command, Length, Renderer, Settings, Subscription, Theme,
};

use ffimage::{
    color::Rgb,
    iter::{BytesExt, ColorConvertExt, PixelsExt},
};

mod ppm;

mod rgba;
use rgba::Rgba;

#[derive(Debug)]
enum App {
    Empty,
    Loading,
    Loaded { image: Image, handle: image::Handle },
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Image, &'static str>),
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let args: Vec<String> = env::args().collect();

        if let Some(last) = args.last() {
            if last == "-" {
                return (
                    App::Loading,
                    Command::perform(load_from_stdin(), Message::Loaded),
                );
            }
        }

        (App::Empty, Command::none())
    }

    fn title(&self) -> String {
        match self {
            App::Empty => String::from("ffimage"),
            App::Loading => String::from("ffimage - Loading"),
            App::Loaded { image, handle: _ } => {
                format!("ffimage - {} x {}", image.width, image.height)
            }
            App::Error(_) => String::from("ffimage - Error"),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            App::Loading => match message {
                Message::Loaded(res) => {
                    match res {
                        Ok(image) => {
                            let rgba: Vec<u8> = image
                                .rgb
                                .iter()
                                .copied()
                                .pixels::<Rgb<u8>>()
                                .colorconvert::<Rgba<u8>>()
                                .bytes()
                                .flatten()
                                .collect();

                            let handle =
                                image::Handle::from_pixels(image.width, image.height, rgba);

                            *self = App::Loaded { image, handle }
                        }
                        Err(reason) => *self = App::Error(String::from(reason)),
                    }
                    Command::none()
                }
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        let content = match self {
            App::Empty => column!(Text::new("No data")),
            App::Loading => column![Text::new("Loading ..")],
            App::Loaded { image: _, handle } => column![image::Viewer::new(handle.clone())],
            App::Error(reason) => column![container(Text::new(format!("Error: {reason}")))],
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    rgb: Vec<u8>,
}

async fn load_from_stdin() -> Result<Image, &'static str> {
    if atty::isnt(atty::Stream::Stdin) {
        return Err("stdin is no tty");
    }

    // read bytes from stdin
    let stdin = io::stdin().lock();
    let bytes = io::BufReader::new(stdin).bytes();
    let bytes = bytes.filter_map(|res| match res {
        Ok(byte) => Some(byte),
        Err(_) => None,
    });

    let res = ppm::read(bytes);
    match res {
        Ok(ppm) => Ok(Image {
            width: ppm.width,
            height: ppm.height,
            rgb: ppm.bytes,
        }),
        Err(e) => Err(e),
    }
}
