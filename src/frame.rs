use crate::color_scheme::ColorScheme;
use crate::fractal_app::FractalApp;
use crate::fractals::FractalType;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::Color32;
use crate::point::Point;

impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Reset View").clicked() {
                        self.center = self.fractal_type.default_center();
                        self.zoom = 1.0;
                        self.needs_update = true;
                        ui.close_menu();
                    }

                    ui.separator();

                    for color_scheme in [
                        ColorScheme::Classic,
                        ColorScheme::Hot,
                        ColorScheme::Cool,
                        ColorScheme::Grayscale,
                    ] {
                        if ui
                            .selectable_label(
                                self.color_scheme == color_scheme,
                                format!("{} Colors", color_scheme.name()),
                            )
                            .clicked()
                        {
                            self.color_scheme = color_scheme;
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("Show Control Panel").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close_menu();
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

        // Left side panel for settings
        if self.show_settings {
            egui::SidePanel::left("settings_panel")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.heading("🎛️ Controls");
                    ui.separator();

                    ui.group(|ui| {
                        ui.label("Fractal Parameters");

                        ui.horizontal(|ui| {
                            ui.label("Iterations:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut self.max_iterations, 10..=1200)
                                        .logarithmic(true),
                                )
                                .changed()
                            {
                                self.needs_update = true;
                            }
                        });

                        // Julia set specific controls
                        if self.fractal_type == FractalType::Julia {
                            ui.separator();
                            ui.label("Julia Constant (c):");

                            ui.horizontal(|ui| {
                                ui.label("Real:");
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
                                ui.label("Imag:");
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

                            // Preset Julia constants
                            ui.separator();
                            ui.label("Presets:");
                            ui.horizontal_wrapped(|ui| {
                                let presets = [
                                    ("Dragon", (-0.7269, 0.1889)),
                                    ("Spiral", (-0.8, 0.156)),
                                    ("Lightning", (-0.74529, 0.11307)),
                                    ("Dendrite", (-0.235, 0.827)),
                                ];

                                for (name, c) in presets {
                                    if ui.small_button(name).clicked() {
                                        self.julia_c = Point::new(c.0, c.1);
                                        self.needs_update = true;
                                    }
                                }
                            });
                        }
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("Navigation");

                        if ui.button("🏠 Reset View").clicked() {
                            self.center = self.fractal_type.default_center();
                            self.zoom = 1.0;
                            self.needs_update = true;
                        }

                        ui.separator();
                        ui.label("Current Position:");
                        ui.monospace(format!("X: {:.6}", self.center.x));
                        ui.monospace(format!("Y: {:.6}", self.center.y));
                        ui.monospace(format!("Zoom: {:.2e}", self.zoom));
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("Instructions");
                        ui.separator();
                        ui.small("• Click and drag to pan");
                        ui.small("• Scroll to zoom in/out");
                        ui.small("• Right-click for context menu");
                        ui.small("• Use menu bar to switch fractals");
                    });
                });
        }

        // Main fractal display area
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            self.image_size = (available_size.x as usize, available_size.y as usize);

            if self.needs_update && self.image_size.0 > 0 && self.image_size.1 > 0 {
                let image = self.generate_fractal_image();
                self.texture = Some(ui.ctx().load_texture("fractal", image, Default::default()));
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
                        ui.close_menu();
                    }

                    if ui.button("Toggle Settings Panel").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close_menu();
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

        // Show a subtle help overlay for first-time users
        if !self.show_settings {
            egui::Window::new("💡 Quick Tip")
                .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-10.0, -10.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.small("Press 'Settings' in the menu to show controls");
                    ui.small("or right-click on the fractal for options");
                });
        }

        if self.is_dragging {
            ctx.request_repaint();
        }
    }
}
