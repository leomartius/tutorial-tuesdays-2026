mod input;
mod render;
mod scenes;
mod theme;

use crate::console::{Console, Event};
use crate::logic::Game;

pub use scenes::GameScene as StartScene;
pub use theme::Theme;

pub(crate) struct SceneStack {
    stack: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new(scene: Box<dyn Scene>) -> Self {
        Self { stack: vec![scene] }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, scene: Box<dyn Scene>) {
        self.stack.push(scene);
    }

    pub fn pop(&mut self) {
        debug_assert!(!self.stack.is_empty());
        self.stack.pop();
    }

    pub fn top_mut(&mut self) -> &mut Box<dyn Scene> {
        debug_assert!(!self.stack.is_empty());
        self.stack.last_mut().unwrap()
    }
}

impl<'a> IntoIterator for &'a SceneStack {
    type Item = &'a Box<dyn Scene>;
    type IntoIter = std::slice::Iter<'a, Box<dyn Scene>>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack.iter()
    }
}

pub(crate) trait Scene {
    /// Draw one full frame of this scene.
    fn render(&self, game: &Game, console: &mut Console, theme: Theme);

    /// Handle one event and decide the next state.
    fn handle_event(&mut self, game: &mut Game, event: Event) -> Transition;
}

pub enum Transition {
    /// Continue without changing the current scene.
    Okay,
    /// Stay on the current scene and play an alert/beep.
    Beep,
    /// Pause the current scene and open a new scene on top of it.
    Push(Box<dyn Scene>),
    /// Close the current scene and resume the previous scene, or quit if none exists.
    Pop,
}
