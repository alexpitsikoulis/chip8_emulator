mod emulator;
use emulator::Emulator;
use relm4::{
    gtk::{
        traits::{BoxExt, GridExt, GtkWindowExt, WidgetExt},
        Box, Grid, Label, Window,
    },
    Component, ComponentParts, RelmApp, RelmIterChildrenExt, RelmWidgetExt,
};

use std::{thread, time::Duration};

pub struct Application {
    display: [[u8; 256]; 128],
}

#[derive(Debug)]
pub enum Message {
    Drw(u8, u8, u8),
    Clr,
    ShutDown,
}

pub struct AppWidgets {
    screen: Grid,
}

impl Component for Application {
    type Input = Message;

    type Output = ();

    type Init = ();

    type Root = Window;

    type Widgets = AppWidgets;

    type CommandOutput = ();

    fn init_root() -> Self::Root {
        let root = Window::builder()
            .width_request(1480)
            .height_request(840)
            .build();
        root.inline_css("background-color: black");
        return root;
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let print_leet: Vec<u8> = vec![
            0xA0, 0x05, 0xD0, 0x15, 0x70, 0x05, 0xA0, 0x23, 0xD0, 0x15, 0x70, 0x05, 0xA0, 0x0F,
            0xD0, 0x15, 0x70, 0x05, 0xA0, 0x28, 0xD0, 0x15,
            // 0x00, 0xE0,
        ];

        let mut emulator = Emulator::new();
        emulator.load_program(print_leet);
        let mut model = Self {
            display: [[0; 256]; 128],
        };

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            (&mut emulator).start(&mut model.display, &sender.input_sender());
        });

        let vBox = Box::new(relm4::gtk::Orientation::Horizontal, 0);
        vBox.set_baseline_position(relm4::gtk::BaselinePosition::Top);
        vBox.set_width_request(1480);
        vBox.set_height_request(840);
        vBox.set_margin_all(0);
        vBox.inline_css("background-color: #F4E6D1; border_radius: 50px");

        let screen = Grid::builder()
            .width_request(1380)
            .height_request(740)
            .margin_top(1)
            .margin_bottom(1)
            .margin_start(1)
            .margin_end(1)
            .build();
        screen.set_margin_all(100);
        screen.inline_css("background-color: black; border-radius: 5px");

        for y in 0..128 {
            for x in 0..256 {
                let pixel = Box::builder().width_request(5).height_request(5).build();
                if x == 0 && y == 0 {
                    pixel.inline_css("border-radius: 5px 0 0 0");
                }
                if x == 0 && y == 127 {
                    pixel.inline_css("border-radius: 0 0 0 5px");
                }
                if x == 255 && y == 0 {
                    pixel.inline_css("border-radius: 0 5px 0 0");
                }
                if x == 255 && y == 127 {
                    pixel.inline_css("border-radius: 0 0 5px 0");
                }
                screen.attach(&pixel, x * 5, y * 5, 5, 5);
            }
        }

        vBox.append(&screen);
        root.set_child(Some(&vBox));

        let widgets = Self::Widgets { screen };
        return ComponentParts { model, widgets };
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            Self::Input::Clr => {
                self.display = [[0; 256]; 128];
                widgets
                    .screen
                    .iter_children()
                    .for_each(|child| child.inline_css("background-color: black"));
            }
            Self::Input::Drw(x, y, bit) => {
                self.display[x as usize][y as usize] = bit;
                let pixel = widgets.screen.child_at(y as i32 * 5, x as i32 * 5).unwrap();
                if bit == 1 {
                    pixel.inline_css("background-color: white");
                } else {
                    pixel.inline_css("background-color: #black");
                }
            }
            Self::Input::ShutDown => {}
        };
    }
}

fn main() {
    let app = RelmApp::new("");
    app.run::<Application>(());
}
