use eframe::{
    egui::{self, CentralPanel, Context, Layout, RichText, ScrollArea, TopBottomPanel, Ui},
    emath::Align,
    App,
};

use crate::api_handlers::{self, UsersError, UsersResponse};

#[derive(Default)]
pub struct UsersApp {
    users_response: UsersResponse,
    // other configs of this app goes here as another struct
}

impl UsersApp {
    pub async fn new() -> Self {
        let users_response = api_handlers::fetch_users()
            .await
            .expect("Failed fetching users");

        Self { users_response }
    }

    fn render_header(self: &Self, ctx: &Context) {
        TopBottomPanel::top("users-app-header").show(ctx, |ui| {
            ui.label(RichText::new("Random Users List").size(30.0));
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

            self.render_header(ctx);
            self.render_users_list(ctx);
            // ui.push_id("dark-mode-toggler", |ui| {
            // egui::widgets::global_dark_light_mode_buttons(ui);
            // });

            // custom_window_frame(ctx, frame, "egui with custom frame", |ui| {
            //     ui.label("This is just the contents of the window");
            //     ui.horizontal(|ui| {
            //         ui.label("egui theme:");
            //         egui::widgets::global_dark_light_mode_buttons(ui);
            //     });
            // });
        });
    }
}

#[allow(dead_code)]
fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let title_height = 28.0;

    let text_color = ctx.style().visuals.text_color();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();

            let painter = ui.painter();

            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            painter.text(
                rect.center_top() + vec2(0.0, title_height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(title_height - 2.0),
                text_color,
            );

            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, title_height),
                    rect.right_top() + vec2(-2.0, title_height),
                ],
                Stroke::new(1.0, Color32::LIGHT_YELLOW),
            );

            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(title_height)),
                Button::new(RichText::new("❌").size(title_height - 4.0)).frame(false),
            );

            if close_response.clicked() {
                frame.quit();
            }

            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + title_height;
                rect
            };

            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::drag());

            if title_bar_response.drag_started() {
                frame.drag_window();
            }

            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);

            let mut content_ui = ui.child_ui(content_rect, *ui.layout());

            add_contents(&mut content_ui);
        });
}
