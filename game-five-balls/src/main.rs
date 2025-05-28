mod game;
use crate::game::Game;

fn main() {
    let mut game = Game::new(10);
    game.initialize_game();
    // Game loop
    while !game.game_over {
        // TODO: Implement game loop logic
        println!("Game loop");
        // This could include:
        // - Getting user input for ball movement
        println!("Getting user input for ball movement");
        // - Calling game.handle_move()
        println!("Calling game.handle_move()");
        // - Displaying the game state
        println!("Displaying the game state");
    }
}
