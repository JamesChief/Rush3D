use eframe::egui::{self, Pos2, pos2, Area, Sense, TopBottomPanel};
use eframe::{App, Frame};

// 定义页面枚举
#[derive(PartialEq, Eq, Clone, Copy)]
enum TabPage {
    Home,
    Files,
    Help,
}

struct MyApp {
    // 菜单状态
    show_about: bool,
    
    // 工具栏状态
    toolbar_pos: Pos2,
    is_toolbar_dragging: bool,
    last_mouse_pos: Pos2,
    
    // Tabs 状态
    current_tab: TabPage,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_about: false,
            toolbar_pos: pos2(100.0, 50.0),
            is_toolbar_dragging: false,
            last_mouse_pos: Pos2::ZERO,
            current_tab: TabPage::Home,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // 1. 绘制顶部菜单
        TopBottomPanel::top("main_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // 文件菜单
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("New file created");
                    }
                    if ui.button("Exit").clicked() {
                        _frame.close();
                    }
                });

                // 帮助菜单
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // 2. 绘制可移动工具栏
        let toolbar_area = Area::new("toolbar_area")
            .fixed_pos(self.toolbar_pos)
            .movable(true)
            .interactable(true);

        toolbar_area.show(ctx, |ui| {
            // 处理工具栏拖动
            let response = ui.interact(
                ui.available_rect_before_wrap(),
                egui::Id::new("toolbar_drag_zone"),
                Sense::drag()
            );

            if response.drag_started() {
                self.is_toolbar_dragging = true;
                self.last_mouse_pos = ctx.input(|i| i.pointer.hover_pos().unwrap_or_default());
            }
            if response.dragged() && self.is_toolbar_dragging {
                if let Some(current_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    let delta = current_pos - self.last_mouse_pos;
                    self.toolbar_pos += delta;
                    self.last_mouse_pos = current_pos;
                }
            }
            if response.drag_released() {
                self.is_toolbar_dragging = false;
            }

            // 工具栏内容
            ui.horizontal(|ui| {
                ui.button("📁 Open");
                ui.button("💾 Save");
                ui.separator();
                ui.button("🔍 Search");
            });
        });

        // 3. 绘制左侧 Tabs
        egui::SidePanel::left("tabs_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Navigation");
                ui.separator();
                
                // Home Tab
                let home_btn = ui.selectable_label(
                    self.current_tab == TabPage::Home,
                    "🏠 Home"
                );
                if home_btn.clicked() {
                    self.current_tab = TabPage::Home;
                }

                // Files Tab
                let files_btn = ui.selectable_label(
                    self.current_tab == TabPage::Files,
                    "📂 Files"
                );
                if files_btn.clicked() {
                    self.current_tab = TabPage::Files;
                }

                // Help Tab
                let help_btn = ui.selectable_label(
                    self.current_tab == TabPage::Help,
                    "❓ Help"
                );
                if help_btn.clicked() {
                    self.current_tab = TabPage::Help;
                }
            });
        });

        // 4. 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                TabPage::Home => {
                    ui.heading("Home Dashboard");
                    ui.label("Welcome to the main interface!");
                }
                TabPage::Files => {
                    ui.heading("File Manager");
                    ui.label("Recent files list...");
                }
                TabPage::Help => {
                    ui.heading("Help Center");
                    ui.button("Open Documentation").clicked();
                }
            }


            let mut showtemp = self.show_about;
            // 关于对话框
            if self.show_about {
                egui::Window::new("About")
                    .open(&mut self.show_about)
                    .show(ctx, |ui| {
                        ui.label("Version 1.0.0");
                        if ui.button("Close").clicked() {
                            showtemp = false;
                        }
                    });
            }
            self.show_about = showtemp;
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rush 3D",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    );
}
