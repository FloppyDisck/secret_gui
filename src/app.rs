use egui::{ScrollArea, TextStyle};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SecretApp {
    // Account
    account: usize,
    accounts: Vec<String>,

    // Snip20
    adding_snip20: bool,

    // Data TXs
    //tx_history: Vec<String>,
}

impl Default for SecretApp {
    fn default() -> Self {
        Self {
            account: 0,
            accounts: vec!["Account1".to_string(), "Account2".to_string(), "Account3".to_string()],
            adding_snip20: false,
        }
    }
}

impl SecretApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SecretApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //let Self { account, accounts, adding_snip20 } = self;

        egui::SidePanel::right("tokens_panel").show(ctx, |ui| {
            ui.heading("Menu");
            ui.add_space(4.0);

            ui.label("")
            egui::ComboBox::from_label("")
                .selected_text(&self.accounts[self.account])
                .show_ui(ui, |ui| {
                    for (i, acc) in self.accounts.iter().enumerate() {
                        ui.selectable_value(&mut self.account, i, acc);
                    }
                });

            // TODO: if last used account =! current account then requery assets
            // TODO: Keep a list of added assets
            let assets = vec![("scrt", 20), ("SHD", 30)];
            for asset in assets.iter() {
                ui.label(format!("{}: {:?}", &asset.0, asset.1));
            }

            if ui.button("Add Snip20").clicked() {
                self.adding_snip20 = true;
            }
            if ui.button("Refresh").clicked() {
                self.adding_snip20 = true;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().stick_to_bottom().show_rows(
                ui,
                row_height,
                5,
                |ui, row_range| {
                    for row in (0..5) {
                        let text = format!("This is row {}", row + 1);
                        ui.label(text);
                    }
                },
            );
            egui::warn_if_debug_build(ui);
        });

        if self.adding_snip20 {
            egui::Window::new("AddSnip20").show(ctx, |ui| {
                ui.label("Add Snip20");
                ui.label("Address");
                ui.label("Authenticate with VK, Permit or custom permit");
                if ui.button("Close").clicked() {
                    self.adding_snip20 = false;
                }
            });
        }

    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}