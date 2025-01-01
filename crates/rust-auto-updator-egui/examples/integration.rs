//! Compiling integration skeleton. The host supplies its window and event loop.
use rust_auto_updator::{Config, Controller, Prepared};

struct Updates {
    controller: Controller,
    check_on_startup: bool,
    pending: Option<Prepared>,
    settings_dirty: bool,
}

impl Updates {
    fn new(ctx: egui::Context, saved_preference: Option<bool>) -> Result<Self, String> {
        let mut config = Config::new(
            "your-account",
            "your-repository",
            env!("CARGO_PKG_VERSION"),
            "YourProduct",
        )?;
        config.install.restart_args = vec!["--updated".into()];
        Ok(Self {
            controller: rust_auto_updator_egui::start(config, ctx)?,
            check_on_startup: saved_preference.unwrap_or(true),
            pending: None,
            settings_dirty: false,
        })
    }

    fn logic(&mut self) {
        let event = self.controller.poll(self.check_on_startup);
        if event.ready_to_install {
            self.pending = self.controller.take_prepared();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        if rust_auto_updator_egui::show(ui, &mut self.controller, &mut self.check_on_startup)
            .preference_changed
        {
            self.settings_dirty = true;
        }
    }

    /// Call only after saving user data and finishing pending host operations.
    /// Return true to let the host exit; never exit on a failed handoff.
    fn handoff_after_saving(&mut self) -> bool {
        let Some(mut prepared) = self.pending.take() else {
            return false;
        };
        match rust_auto_updator::handoff(&mut prepared) {
            Ok(()) => true,
            Err(error) => {
                self.controller.installation_failed(error);
                false
            }
        }
    }
}

fn main() {
    // Check the integration signatures without creating a UI or making requests.
    let _ = Updates::new;
    let _ = Updates::logic;
    let _ = Updates::ui;
    let _ = Updates::handoff_after_saving;
    println!(
        "Integrate Updates into the host event loop; see the workspace README for CLI routing."
    );
}
