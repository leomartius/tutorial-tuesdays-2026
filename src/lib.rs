mod console;
mod logic;
mod ui;

use anyhow::Result;

use console::{Console, Event};
use logic::Game;
use ui::{SceneStack, StartScene, Theme, Transition};

pub fn run() -> Result<()> {
    let theme = Theme {};
    let mut console = Console::new(80, 50, "Yet Another Roguelike Tutorial")?;
    let mut game = Game::new();
    let mut scenes = SceneStack::new(Box::new(StartScene));
    loop {
        console.clear();
        for scene in &scenes {
            scene.render(&game, &mut console, theme);
        }
        console.display()?;
        let event = console.read_event()?;
        match event {
            Event::ClearScreen => console.reset()?,
            Event::Abort => {
                while !scenes.is_empty() {
                    scenes.pop();
                }
            }
            event => match scenes.top_mut().handle_event(&mut game, event) {
                Transition::Okay => {}
                Transition::Beep => console.alert()?,
                Transition::Push(next) => {
                    scenes.push(next);
                }
                Transition::Pop => {
                    scenes.pop();
                }
            },
        }
        if scenes.is_empty() {
            break;
        }
    }
    Ok(())
}
