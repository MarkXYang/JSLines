mod game;
mod gui; // Declare the new gui module
use crate::game::Game; // Keep for later use

use gtk4 as gtk;
use gtk::prelude::*;
use glib;
use crate::gui::GameBoardWidget; // Import GameBoardWidget
use std::cell::{Rc, RefCell}; // Import Rc and RefCell
use crate::game::Game; // Ensure Game is imported (already there but good to confirm)

fn main() {
    let application = gtk::Application::new(
        Some("com.example.fiveballs"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Five Balls"));
    window.set_default_size(400, 300); // Default size, can be adjusted

    // Create and initialize the game model
    let game_model = Rc::new(RefCell::new(Game::new(9))); // 9x9 grid
    game_model.borrow_mut().initialize_game();

    // Create the GameBoardWidget and pass the game model
    let game_board_widget = GameBoardWidget::new(game_model.clone());
    game_board_widget.set_hexpand(true);
    game_board_widget.set_vexpand(true);

    window.set_child(Some(&game_board_widget));
    window.present();
}
