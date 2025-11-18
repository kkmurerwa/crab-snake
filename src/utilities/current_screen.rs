pub enum CurrentScreen {
    LaunchScreen,
    GameScreen,
    ScoreScreen,
}

impl PartialEq for CurrentScreen {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CurrentScreen::LaunchScreen, CurrentScreen::LaunchScreen) => true,
            (CurrentScreen::GameScreen, CurrentScreen::GameScreen) => true,
            (CurrentScreen::ScoreScreen, CurrentScreen::ScoreScreen) => true,
            _ => false,
        }
    }
}
