use super::Game;
#[test]
fn load_dynamic_ai_lib() {
    let mut game = Game::new(5);
    println!("{:?}", game.call_ai_script());
}
