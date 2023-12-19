mod game;
mod player;

fn main() {
    let game = game::Game::new();
    let mut player = player::Player::new(game::Game::ask_user_for_name());
    game.start(&mut player);
}
