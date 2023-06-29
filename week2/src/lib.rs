pub trait Game {
    fn start(&self);
    fn pause(&self);
    fn stop(&self);
    fn clean(&self);
}

pub fn start_games(games: Vec<&dyn Game>) {
    for game in games {
        game.start();
        game.clean();
        game.stop();
    }
}
