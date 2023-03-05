mod emulator;
use emulator::Emulator;
use relm4::{
    gtk::{
        traits::{BoxExt, GridExt, GtkWindowExt, WidgetExt},
        Box, Grid, Window,
    },
    Component, ComponentParts, RelmApp, RelmIterChildrenExt, RelmWidgetExt,
};

use std::{thread, time::Duration};

pub struct Application {
    display: [[u8; 128]; 64],
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
        let program: Vec<u8> = vec![
            // 0x63, 0x01, 0xF3, 0x29, 0xD0, 0x15, 0x70, 0x05, 0x63, 0x07, 0xF3, 0x29, 0xD0, 0x15,
            // 0x70, 0x05, 0x63, 0x03, 0xF3, 0x29, 0xD0, 0x15, 0x70, 0x05, 0x63, 0x08, 0xF3, 0x29,
            // 0xD0,
            // 0x15,
            0x70, 0x1, 0x70, 0x1, 0x70, 0x1, 0x63, 0x1, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63,
            0x8, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0x4, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5,
            0x63, 0x7, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xa, 0xf3, 0x29, 0xd0,
            0x15, 0x70, 0x5, 0x63, 0xf, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xb, 0xf3, 0x29,
            0xd0, 0x15, 0x70, 0x5, 0x63, 0xb, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63,
            0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xd, 0xf3, 0x29, 0xd0, 0x15,
            0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xa, 0xf3, 0x29, 0xd0,
            0x15, 0x70, 0x5, 0x63, 0xd, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xb,
            0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63,
            0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xf, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5,
            0x70, 0x1, 0x63, 0xf, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0,
            0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xb, 0xf3, 0x29,
            0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xd, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63,
            0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x71, 0x6,
            0x60, 0x0, 0x63, 0xb, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xb, 0xf3,
            0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xb,
            0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x70, 0x1, 0x63, 0xb, 0xf3, 0x29, 0xd0, 0x15, 0x70,
            0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15,
            0x70, 0x5, 0x63, 0xe, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5, 0x63, 0xb, 0xf3, 0x29, 0xd0,
            0x15, 0x70, 0x5, 0x63, 0xa, 0xf3, 0x29, 0xd0, 0x15, 0x70, 0x5,
        ];

        let mut emulator = Emulator::new();
        emulator.load_program(program);
        let mut model = Self {
            display: [[0; 128]; 64],
        };

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            (&mut emulator).start(&mut model.display, &sender.input_sender());
        });

        let frame = Box::new(relm4::gtk::Orientation::Horizontal, 0);
        frame.set_baseline_position(relm4::gtk::BaselinePosition::Top);
        frame.set_width_request(1480);
        frame.set_height_request(840);
        frame.set_margin_all(0);
        frame.inline_css("background-color: #F4E6D1; border-radius: 50px");

        let screen = Grid::builder()
            .width_request(1280)
            .height_request(640)
            .build();
        screen.set_margin_all(100);
        screen.inline_css("background-color: black; border-radius: 5px; padding: 5px");

        for y in 0..64 {
            for x in 0..128 {
                let pixel = Box::builder().width_request(10).height_request(10).build();
                screen.attach(&pixel, x * 10, y * 10, 10, 10);
            }
        }

        frame.append(&screen);
        root.set_child(Some(&frame));

        let widgets = Self::Widgets { screen };
        return ComponentParts { model, widgets };
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            Self::Input::Clr => {
                self.display = [[0; 128]; 64];
                widgets
                    .screen
                    .iter_children()
                    .for_each(|child| child.inline_css("background-color: black"));
            }
            Self::Input::Drw(x, y, bit) => {
                self.display[x as usize][y as usize] = bit;
                let pixel = widgets
                    .screen
                    .child_at(y as i32 * 10, x as i32 * 10)
                    .unwrap();
                if bit == 1 {
                    pixel.inline_css("background-color: white");
                } else {
                    pixel.inline_css("background-color: black");
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
