use gtk4 as gtk;
use gtk::prelude::*;
use glib; // Use direct glib import
use gtk::cairo;
use gtk::MessageDialog; // Import MessageDialog
use std::rc::Rc; // Corrected Rc import
use std::cell::RefCell; // Separate RefCell import
use crate::game::Game;
use crate::ball::{Ball, BallColor};

// Helper function to convert BallColor to RGB
fn ball_color_to_rgb(ball_color: BallColor) -> (f64, f64, f64) {
    match ball_color {
        BallColor::Red => (1.0, 0.0, 0.0),
        BallColor::Green => (0.0, 1.0, 0.0),
        BallColor::Blue => (0.0, 0.0, 1.0),
        BallColor::Yellow => (1.0, 1.0, 0.0),
        BallColor::Brown => (0.6, 0.3, 0.0), // Example for Brown
        // Add other colors if present in your BallColor enum
    }
}

mod imp {
    use super::*;
    // use glib::subclass::InitializingObject; // Appears unused
    use glib::subclass::prelude::*; // Use direct glib import
    use std::cell::OnceCell; // For properties
    use gtk::Window; // For type casting parent window
    use glib; // For glib::Properties derive and other glib items

    #[derive(glib::Properties, Default)] // Use direct glib::Properties
    #[properties(wrapper_type = super::GameBoardWidget)]
    pub struct GameBoardWidget {
        #[property(get, set, construct_only, type = Option<Rc<RefCell<Game>>>)]
        game: OnceCell<Rc<RefCell<Game>>>,
    }

    #[glib::object_subclass] // Use direct glib::object_subclass
    impl ObjectSubclass for GameBoardWidget {
        const NAME: &'static str = "FiveBallsGameBoardWidget";
        type Type = super::GameBoardWidget;
        type ParentType = gtk::DrawingArea;

        fn new() -> Self {
            Self {
                game: OnceCell::new(),
            }
        }
    }

    impl ObjectImpl for GameBoardWidget {
        fn properties() -> &'static [glib::ParamSpec] { // Use direct glib::ParamSpec
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) { // Use direct glib::Value, glib::ParamSpec
            Self::derived_set_property(self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value { // Use direct glib::Value, glib::ParamSpec
            Self::derived_property(self, id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed(); // Important!
            let widget = self.obj().clone(); // To move into closure

            let gesture = gtk::GestureClick::new();
            gesture.connect_pressed(move |_gesture, _n_press, x, y| {
                let game_model_opt = widget.property::<Option<Rc<RefCell<Game>>>>("game");
                if let Some(game_model_rc) = game_model_opt {
                    // Check if game is already over at the beginning of the handler
                    if game_model_rc.borrow().game_over {
                        return; // Do nothing if game is already over
                    }

                    if let Ok(mut game) = game_model_rc.try_borrow_mut() {
                        let grid_size = game.grid.size;
                        if grid_size == 0 { return; }

                        let widget_width = widget.width() as f64;
                        let widget_height = widget.height() as f64;
                        
                        let cell_width = widget_width / grid_size as f64;
                        let cell_height = widget_height / grid_size as f64;

                        let col = (x / cell_width) as usize;
                        let row = (y / cell_height) as usize;

                        if row < grid_size && col < grid_size {
                            game.select_cell(row, col);
                        }
                        
                        // Check for game over after the move and update UI accordingly
                        let game_is_over = game.borrow().game_over;
                        let final_score = game.borrow().score; // Borrow again to get score

                        // queue_draw should happen before dialog to show final state
                        widget.queue_draw(); 

                        if game_is_over {
                            if let Some(window) = widget.root().and_then(|r| r.downcast::<Window>().ok()) {
                                Self::show_game_over_dialog_with_score(&window, final_score);
                            }
                        }
                    } else {
                        eprintln!("GameBoardWidget: Game already borrowed mutably during click logic.");
                    }
                } else {
                     widget.queue_draw(); // Still draw if game model wasn't available for some reason
                }
            });
            self.obj().add_controller(gesture);
        }
    }
    
    impl GameBoardWidget {
        fn show_game_over_dialog_with_score(parent_window: &impl IsA<Window>, score: u32) {
            let dialog = MessageDialog::new(
                Some(parent_window),
                gtk::DialogFlags::MODAL | gtk::DialogFlags::DESTROY_WITH_PARENT,
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                "Game Over!",
            );
            dialog.set_secondary_text(Some(&format!("The board is full.\nFinal Score: {}", score)));
            dialog.connect_response(|d, _| {
                d.close(); // Use close for Dialog as destroy is handled by DESTROY_WITH_PARENT
            });
            dialog.present();
        }
    }

    impl WidgetImpl for GameBoardWidget {}

    impl DrawingAreaImpl for GameBoardWidget {
        // Corrected signature for draw method
        fn draw(&self, cr: &gtk::cairo::Context, width: i32, height: i32) {
            // Background
            cr.set_source_rgb(0.95, 0.95, 0.95); // Light gray
            cr.paint().expect("Failed to paint background");

            let game_rc_opt = self.obj().property::<Option<Rc<RefCell<Game>>>>("game");

            if let Some(game_rc) = game_rc_opt {
                if let Ok(game) = game_rc.try_borrow() {
                    let grid_size = game.grid.size;
                    if grid_size == 0 { return; } // Avoid division by zero

                    let cell_width = width as f64 / grid_size as f64;
                    let cell_height = height as f64 / grid_size as f64;
                    let ball_radius = (cell_width.min(cell_height) / 2.0) * 0.8; // 80% of half cell size

                    // Draw grid lines
                    cr.set_source_rgb(0.7, 0.7, 0.7); // Medium gray for grid lines
                    cr.set_line_width(1.0);
                    for i in 0..=grid_size {
                        // Vertical lines
                        cr.move_to(i as f64 * cell_width, 0.0);
                        cr.line_to(i as f64 * cell_width, height as f64);
                        // Horizontal lines
                        cr.move_to(0.0, i as f64 * cell_height);
                        cr.line_to(width as f64, i as f64 * cell_height);
                    }
                    cr.stroke().expect("Failed to draw grid lines");

                    // Draw balls
                    for r in 0..grid_size {
                        for c in 0..grid_size {
                            if let Some(ball) = &game.grid.cells[r][c] {
                                let (rgb_r, rgb_g, rgb_b) = ball_color_to_rgb(ball.color);
                                cr.set_source_rgb(rgb_r, rgb_g, rgb_b);
                                
                                let center_x = (c as f64 * cell_width) + (cell_width / 2.0);
                                let center_y = (r as f64 * cell_height) + (cell_height / 2.0);
                                
                                cr.arc(center_x, center_y, ball_radius, 0.0, 2.0 * std::f64::consts::PI);
                                cr.fill().expect("Failed to fill ball");
                            }
                        }
                    }

                    // Draw highlight for selected ball
                    if let Some((selected_r, selected_c)) = game.selected_ball_pos {
                        if selected_r < grid_size && selected_c < grid_size { // Ensure it's within bounds
                            // Highlight the cell
                            cr.set_source_rgba(0.0, 0.0, 1.0, 0.3); // Semi-transparent blue
                            cr.rectangle(selected_c as f64 * cell_width, 
                                         selected_r as f64 * cell_height, 
                                         cell_width, 
                                         cell_height);
                            cr.fill().expect("Failed to fill highlight for cell");

                            // Optionally, highlight the ball itself with a border
                            if game.grid.cells[selected_r][selected_c].is_some() {
                                let center_x = (selected_c as f64 * cell_width) + (cell_width / 2.0);
                                let center_y = (selected_r as f64 * cell_height) + (cell_height / 2.0);
                                cr.set_source_rgba(0.1, 0.1, 0.1, 0.7); // Darker, semi-transparent border
                                cr.set_line_width(2.0);
                                cr.arc(center_x, center_y, ball_radius + 1.5, 0.0, 2.0 * std::f64::consts::PI);
                                cr.stroke().expect("Failed to stroke highlight for ball");
                            }
                        }
                    }
                } else {
                     // Game is already borrowed, skip drawing this frame or log an error
                    eprintln!("GameBoardWidget: Game model already borrowed during draw call.");
                    cr.set_source_rgb(1.0, 0.0, 0.0); // Red background to indicate error
                    cr.paint().expect("Failed to paint error background");
                    return;
                }
            } else {
                // Game property not set or is None
                cr.set_source_rgb(0.5, 0.5, 0.5); // Dark gray if no game
                cr.paint().expect("Failed to paint no-game background");
                // Optionally, draw text indicating no game data
                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.select_font_face("sans-serif", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                cr.set_font_size(20.0);
                let text = "Game not loaded";
                let extents = cr.text_extents(text).unwrap();
                cr.move_to( (width as f64 - extents.width()) / 2.0, (height as f64 - extents.height()) / 2.0 + extents.height());
                cr.show_text(text).expect("Failed to show text");
            }
        }
    }
}

glib::wrapper! { // Use direct glib::wrapper
    pub struct GameBoardWidget(ObjectSubclass<imp::GameBoardWidget>)
        @extends gtk::Widget, gtk::DrawingArea;
}

impl GameBoardWidget {
    // Constructor now takes the game model
    pub fn new(game_model: Rc<RefCell<Game>>) -> Self {
        glib::Object::new(&[("game", &Some(game_model))]) // Pass as Option<Rc<RefCell<Game>>>
            .expect("Failed to create GameBoardWidget with game model")
    }
}

// Default for GameBoardWidget is not practical if it requires a game model at construction.
// However, if `imp::GameBoardWidget::game` was `RefCell<Option<...>>` and not construct_only,
// a default new could exist and game set later. But with `construct_only` and `OnceCell`,
// it must be provided at construction.
// Removing Default impl as it's no longer suitable.
// impl Default for GameBoardWidget {
//     fn default() -> Self {
//         Self::new() // This would fail as new() now requires an argument
//     }
// }
