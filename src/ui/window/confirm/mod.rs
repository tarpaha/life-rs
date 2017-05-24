// Simple confirmation window

use super::{WindowBase, InfoWindowTrait, PostAction, States};

use piston_window::{Input, Button, Key, Context, Event};
use opengl_graphics::GlGraphics;

use super::super::super::engine::Engine;
use super::super::super::Resources;

use std::rc::Rc;
use std::cell::{RefCell, Cell};

#[derive(PartialEq)]
pub enum UserChoice {
    Ok,
    Cancel,
}

pub struct ConfirmationWindow<'a, F>
    where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice) {

    msg: &'a str,

    scr_width: f64,
    scr_height: f64,

    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,

    callback: F,
}

impl<'a, F> ConfirmationWindow<'a, F>
    where F: FnMut(Rc<RefCell<Engine<'a>>>, UserChoice)  {

    pub fn new(resources: Rc<RefCell<Resources>>, engine: Rc<RefCell<Engine<'a>>>,
               callback: F, msg: &'a str, width: f64, height: f64) -> Self {

        ConfirmationWindow {
            msg: msg,

            scr_width: width,
            scr_height: height,

            engine: engine,
            resources: resources,

            callback: callback
        }
    }

}

impl<'a, F> InfoWindowTrait for ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>,
    UserChoice) {

}

impl<'a, F> WindowBase for ConfirmationWindow<'a, F> where F: FnMut(Rc<RefCell<Engine<'a>>>,
    UserChoice) {

    fn paint(&mut self, c: Context, g: &mut GlGraphics) {

        let (scr_width, scr_height) = (self.scr_width, self.scr_height);
        let resources = self.resources.clone();

        self.paint_info_window(c, g, scr_width, scr_height,
                               resources, self.msg, "(Y/N)");

    }

    fn event_dispatcher(&mut self, event: &Event, cur_state: &Cell<States>) -> PostAction {

        match event {

             &Event::Input(Input::Press(Button::Keyboard(Key::Y))) => {
                 (self.callback)(self.engine.clone(), UserChoice::Ok);
                 PostAction::Pop
             }

             &Event::Input(Input::Press(Button::Keyboard(Key::N))) => {
                 (self.callback)(self.engine.clone(), UserChoice::Cancel);
                 PostAction::Pop
             }

            _ => PostAction::Stop

        }

    }

    fn is_modal(&self) -> bool {
        true
    }

}
