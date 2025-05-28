// Declare modules at the crate root (for a binary crate, this is main.rs)
mod ball;
mod grid;
mod solver;
mod pathfinding; // Assuming this file exists as per prior context
mod game;
mod gui;

use gtk4 as gtk;
use gtk::prelude::*;
// use glib; // gtk::glib should be preferred if gtk4 is already a dependency
use crate::gui::GameBoardWidget;
use std::rc::Rc; // Corrected import for Rc
use std::cell::RefCell;
use crate::game::Game; // Single import for Game

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
    // initialize_game is called within Game::new or should be called if necessary
    // game_model.borrow_mut().initialize_game(); // This was called in Game::new in some versions

    // Create the GameBoardWidget and pass the game model
    let game_board_widget = GameBoardWidget::new(game_model); // Pass game_model directly, new() takes Rc<RefCell<Game>>
    game_board_widget.set_hexpand(true);
    game_board_widget.set_vexpand(true);

    window.set_child(Some(&game_board_widget));
    window.present();
}
