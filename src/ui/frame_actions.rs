use crate::utils::precision_mode::PrecisionMode;
use crate::fractals::fractal_type::FractalType;
use crate::utils::color_scheme::ColorScheme;
use crate::utils::point::Point;
use crate::ui::fractal_app::FractalApp;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::Color32;
use egui::{ColorImage, TextureOptions};

impl eframe::App for FractalApp {
    /// Called to update the UI and handle events.
    #[inline]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Fractal", |ui| {
                    for fractal_type in [
                        FractalType::Mandelbrot,
                        FractalType::Julia,
                        FractalType::BurningShip,
                        FractalType::Tricorn,
                    ] {
                        if ui
                            .selectable_label(
                                self.fractal_type == fractal_type,
                                fractal_type.name(),
                            )
                            .clicked()
                        {
                            if self.fractal_type != fractal_type {
                                self.fractal_type = fractal_type;
                                self.center = fractal_type.default_center();
                                self.zoom = 1.0;
                                self.needs_update = true;
                            }
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Color", |ui| {
                    if ui.button("Reset View").clicked() {
                        self.center = self.fractal_type.default_center();
                        self.zoom = 1.0;
                        self.needs_update = true;
                        ui.close();
                    }

                    ui.separator();

                    for color_scheme in ColorScheme::all() {
                        if ui
                            .selectable_label(
                                self.color_scheme == color_scheme,
                                format!("{} Colors", color_scheme.name()),
                            )
                            .clicked()
                        {
                            self.color_scheme = color_scheme;
                            self.needs_update = true;
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("Show Control Panel").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close();
                    }
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Zoom: {:.2e}", self.zoom));
                    ui.separator();
                    ui.label(format!("({:.4}, {:.4})", self.center.x, self.center.y));
                    ui.separator();
                    ui.label(self.fractal_type.name());
                });
            });
        });

        if self.show_settings {
            egui::SidePanel::left("settings_panel")
                .resizable(true)
                .default_width(300.0)
                .min_width(280.0)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.heading(egui::RichText::new("🎨 Fractal Studio").size(20.0).strong());
                        ui.label(egui::RichText::new("Professional Fractal Explorer").size(11.0).weak());
                    });
                    ui.add_space(8.0);
                    ui.separator();

                    egui::Frame::new()
                        .fill(ui.visuals().extreme_bg_color)
                        .inner_margin(10.0)
                        .corner_radius(6.0)
                        .show(ui, |ui| {
                        ui.label(egui::RichText::new("⚙️ Render Settings").size(14.0).strong());
                        ui.add_space(6.0);

                        ui.label(egui::RichText::new("Quality").size(12.0));
                        if ui
                            .add(
                                egui::Slider::new(&mut self.max_iterations, 10..=3000)
                                    .text("Iterations")
                                    .logarithmic(true),
                            )
                            .changed()
                        {
                            self.needs_update = true;
                        }

                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("Resolution").size(12.0));
                        ui.horizontal(|ui| {
                            ui.label("W:");
                            if ui
                                .add(
                                    egui::DragValue::new(&mut self.image_size.0)
                                        .range(100..=8192)
                                        .suffix(" px")
                                        .speed(10.0),
                                )
                                .changed()
                            {
                                self.needs_update = true;
                            }
                            ui.label("H:");
                            if ui
                                .add(
                                    egui::DragValue::new(&mut self.image_size.1)
                                        .range(100..=8192)
                                        .suffix(" px")
                                        .speed(10.0),
                                )
                                .changed()
                            {
                                self.needs_update = true;
                            }
                        });

                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("Precision Mode").size(12.0));
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_value(
                                    &mut self.precision_mode,
                                    PrecisionMode::Fast,
                                    "🚀 Fast",
                                )
                                .on_hover_text("32-bit float - fastest rendering")
                                .clicked()
                            {
                                self.needs_update = true;
                            }
                            if ui
                                .selectable_value(
                                    &mut self.precision_mode,
                                    PrecisionMode::High,
                                    "🎯 High",
                                )
                                .on_hover_text("64-bit float - deeper zoom capability")
                                .clicked()
                            {
                                self.needs_update = true;
                            }
                            #[cfg(feature = "f128")]
                            if ui
                                .selectable_value(
                                    &mut self.precision_mode,
                                    PrecisionMode::UltraHigh,
                                    "🔬 Ultra",
                                )
                                .on_hover_text("128-bit decimal - extreme zoom for deep exploration")
                                .clicked()
                            {
                                self.needs_update = true;
                            }
                        });

                    });

                    if self.fractal_type == FractalType::Julia {
                        ui.add_space(8.0);
                        egui::Frame::NONE
                            .fill(ui.visuals().extreme_bg_color)
                            .inner_margin(10.0)
                            .corner_radius(6.0)
                            .show(ui, |ui| {
                            ui.label(egui::RichText::new("🌀 Julia Parameters").size(14.0).strong());
                            ui.add_space(6.0);

                            ui.label(egui::RichText::new("Constant (c)").size(12.0));
                            ui.horizontal(|ui| {
                                ui.label("Re:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.julia_c.x)
                                            .speed(0.001)
                                            .range(-2.0..=2.0),
                                    )
                                    .changed()
                                {
                                    self.needs_update = true;
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Im:");
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.julia_c.y)
                                            .speed(0.001)
                                            .range(-2.0..=2.0),
                                    )
                                    .changed()
                                {
                                    self.needs_update = true;
                                }
                            });

                            ui.add_space(6.0);
                            ui.label(egui::RichText::new("Presets").size(12.0));
                            ui.horizontal_wrapped(|ui| {
                                let presets = [
                                    ("🐉 Dragon", (-0.7269, 0.1889)),
                                    ("🌀 Spiral", (-0.8, 0.156)),
                                    ("⚡ Lightning", (-0.74529, 0.11307)),
                                    ("🌿 Dendrite", (-0.235, 0.827)),
                                    ("❄️ Snowflake", (-0.4, 0.6)),
                                    ("🔥 Fire", (0.285, 0.01)),
                                ];

                                for (name, c) in presets {
                                    if ui.button(name).clicked() {
                                        self.julia_c = Point::new(c.0, c.1);
                                        self.needs_update = true;
                                    }
                                }
                            });
                        });
                    }

                    ui.add_space(8.0);

                    egui::Frame::new()
                        .fill(ui.visuals().extreme_bg_color)
                        .inner_margin(10.0)
                        .corner_radius(6.0)
                        .show(ui, |ui| {
                        ui.label(egui::RichText::new("🗺️ Navigation").size(14.0).strong());
                        ui.add_space(6.0);

                        if ui.button(egui::RichText::new("🏠 Reset View").size(13.0)).clicked() {
                            self.center = self.fractal_type.default_center();
                            self.zoom = 1.0;
                            self.needs_update = true;
                        }

                        ui.add_space(6.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Current State").size(12.0).weak());
                        egui::Grid::new("info_grid")
                            .num_columns(2)
                            .spacing([10.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Position:");
                                ui.monospace(format!("({:.6}, {:.6})", self.center.x, self.center.y));
                                ui.end_row();

                                ui.label("Zoom:");
                                ui.monospace(format!("{:.2e}×", self.zoom));
                                ui.end_row();

                                ui.label("Fractal:");
                                ui.label(self.fractal_type.name());
                                ui.end_row();

                                ui.label("Size:");
                                ui.monospace(format!("{}×{}", self.image_size.0, self.image_size.1));
                                ui.end_row();
                            });
                    });

                    ui.add_space(8.0);

                    egui::Frame::new()
                        .fill(ui.visuals().faint_bg_color)
                        .inner_margin(8.0)
                        .corner_radius(4.0)
                        .show(ui, |ui| {
                        ui.label(egui::RichText::new("💡 Quick Tips").size(12.0).strong());
                        ui.add_space(4.0);
                        ui.label("🖱️ Drag to pan view");
                        ui.label("🔍 Scroll to zoom in/out");
                        ui.label("🖱️ Double-click to zoom to point");
                        ui.label("🖱️ Right-click for context menu");
                    });
                });
        }

        // Main fractal display area
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            self.image_size = (
                (available_size.x as usize).try_into().unwrap(),
                available_size.y as u32,
            );

            if self.needs_update && self.image_size.0 > 0 && self.image_size.1 > 0 {
                let image: ColorImage = self.generate_fractal_image();
                self.texture = Some(ui.ctx().load_texture(
                    "fractal",
                    image,
                    TextureOptions::default(),
                ));
                self.needs_update = false;
            }

            if let Some(texture) = &self.texture {
                let image_rect = Rect::from_min_size(
                    ui.next_widget_position(),
                    Vec2::new(self.image_size.0 as f32, self.image_size.1 as f32),
                );

                let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());

                // Right-click context menu
                response.context_menu(|ui| {
                    if ui.button("Reset View").clicked() {
                        self.center = self.fractal_type.default_center();
                        self.zoom = 1.0;
                        self.needs_update = true;
                        ui.close();
                    }

                    if ui.button("Toggle Settings Panel").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close();
                    }

                    ui.separator();
                    ui.label(format!(
                        "Position: ({:.6}, {:.6})",
                        self.center.x, self.center.y
                    ));
                    ui.label(format!("Zoom Level: {:.2e}", self.zoom));
                });

                ui.painter().image(
                    texture.id(),
                    image_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );

                self.handle_mouse_input(&response, image_rect);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                    ui.label("Generating fractal...");
                });
            }
        });

        if self.is_dragging {
            ctx.request_repaint();
        }
    }
}
