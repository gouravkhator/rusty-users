use eframe::{
    egui::{
        self, CentralPanel, Context, Image, Label, Layout, Resize, RichText, ScrollArea,
        TopBottomPanel, Ui,
    },
    emath::{vec2, Align},
    epaint::{Fonts, ImageData},
    App,
};

use crate::api_handlers::{self, UsersResponse};
use egui_extras::RetainedImage;

#[derive(thiserror::Error, Debug)]
pub enum UsersAppError {
    #[error("Failed fetching users' details")]
    FetchFailedErr(#[from] reqwest::Error),
}

#[derive(Default)]
pub struct UsersApp {
    users_response: UsersResponse,
    // TODO: other configs of this app goes here as another struct
}

impl UsersApp {
    pub async fn new() -> Self {
        let users_response = api_handlers::fetch_users()
            .await
            .expect("Failed fetching users");

        Self { users_response }
    }

    fn render_top_panel(self: &Self, ctx: &Context) {
        TopBottomPanel::top("top-panel").show(ctx, |ui| {
            let image = RetainedImage::from_image_bytes(
                "sample-image.png",
                include_bytes!("sample-image.png"),
            )
            .expect("Failed loading the logo");

            // image.show_max_size(ui, vec2(30.0, 30.0));

            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(Image::new(image.texture_id(ctx), vec2(30.0, 30.0)));
            });

            // add buttons in right to left layout, to add them to the right most side
            ui.with_layout(Layout::right_to_left(), |ui| {});
        });
    }

    fn render_header(self: &Self, ctx: &Context) {
        TopBottomPanel::top("users-app-header").show(ctx, |ui| {
            ui.add_space(40.);

            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Random Users Data").size(30.));
            });
        });
    }

    fn render_users_list(self: &Self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for user in &(self.users_response.users) {
                    ui.label(RichText::new(format!("User Id: {}", user.id)).size(21.0));

                    ui.label(
                        RichText::new(format!("Full Name: {}", user.get_full_name())).size(18.0),
                    );

                    ui.with_layout(
                        Layout::right_to_left().with_cross_align(Align::LEFT),
                        |ui| {
                            ui.add(egui::Hyperlink::from_label_and_url(
                                "See User's Profile",
                                format!("http://example.com/u/{}", user.id),
                            ));
                        },
                    );
                }
            });
        });
    }
}

impl App for UsersApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // ctx.set_debug_on_hover(true); // to debug the layout for the rendered elements

            self.render_top_panel(ctx);
            self.render_header(ctx);
            self.render_users_list(ctx);

            // ui.push_id("dark-mode-toggler", |ui| {
            // egui::widgets::global_dark_light_mode_buttons(ui);
            // });
        });
    }
}
