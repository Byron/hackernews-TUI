use super::comment_view::CommentView;
use super::story_view::StoryView;
use crate::prelude::*;

/// Construct a new Event view from a view with ListEventView trait
/// by adding simple key-pressed event handlers
pub fn construct_list_event_view<T: ListEventView>(view: T) -> OnEventView<T> {
    // add "j" and "k" for moving down and up the story list
    OnEventView::new(view)
        .on_pre_event_inner('k', |s, _| s.focus_up())
        .on_pre_event_inner('j', |s, _| s.focus_down())
        .on_pre_event_inner('t', |s, _| s.focus_top())
        .on_pre_event_inner('b', |s, _| s.focus_bottom())
        // event handlers for parsing numbers
        .on_pre_event_inner(EventTrigger::from_fn(|_| true), |s, e| match *e {
            Event::Char(c) => s.handle_digit(c),
            _ => None,
        })
        // ignore up,down,pageUp,pageDown keys. Rely on main scrollView to handle those keys
        .on_pre_event_inner(EventTrigger::from_fn(|_| true), |_, e| match *e {
            Event::Key(Key::Up)
            | Event::Key(Key::Down)
            | Event::Key(Key::PageUp)
            | Event::Key(Key::PageDown) => Some(EventResult::Ignored),
            _ => None,
        })
}

/// ListEventView is a trait that implements basic method interfaces
/// to interact with a List View (normally CommentView or StoryView)
pub trait ListEventView {
    fn focus_top(&mut self) -> Option<EventResult> {
        None
    }
    fn focus_bottom(&mut self) -> Option<EventResult> {
        None
    }
    fn focus_up(&mut self) -> Option<EventResult> {
        None
    }
    fn focus_down(&mut self) -> Option<EventResult> {
        None
    }
    fn handle_digit(&mut self, _: char) -> Option<EventResult> {
        None
    }
    fn add_help_dialog(&self) -> Option<EventResult> {
        None
    }
}

impl ListEventView for LinearLayout {
    fn focus_top(&mut self) -> Option<EventResult> {
        if self.len() > 0 {
            match self.set_focus_index(0) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => None,
            }
        } else {
            Some(EventResult::Consumed(None))
        }
    }
    fn focus_bottom(&mut self) -> Option<EventResult> {
        if self.len() > 0 {
            match self.set_focus_index(self.len() - 1) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => None,
            }
        } else {
            Some(EventResult::Consumed(None))
        }
    }
    fn focus_up(&mut self) -> Option<EventResult> {
        let id = self.get_focus_index();
        if id > 0 {
            match self.set_focus_index(id - 1) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => Some(EventResult::Ignored),
            }
        } else {
            Some(EventResult::Ignored)
        }
    }

    fn focus_down(&mut self) -> Option<EventResult> {
        let id = self.get_focus_index();
        if id + 1 < self.len() {
            match self.set_focus_index(id + 1) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => Some(EventResult::Ignored),
            }
        } else {
            Some(EventResult::Ignored)
        }
    }
}

#[macro_export]
macro_rules! raw_command {
    () => {
        pub fn add_raw_command_char(&mut self, c: char) {
            self.raw_command.push(c);
        }

        pub fn clear_raw_command(&mut self) {
            self.raw_command.clear();
        }

        pub fn get_raw_command_as_number(&self) -> Result<usize> {
            Ok(self.raw_command.parse::<usize>()?)
        }
    };
}

#[macro_export]
macro_rules! list_event_view_wrapper {
    ($($x:expr),*) => {
        fn focus_up(&mut self) -> Option<EventResult> {
            self.get_inner_mut().focus_up()
        }
        fn focus_down(&mut self) -> Option<EventResult> {
            self.get_inner_mut().focus_down()
        }
        fn focus_top(&mut self) -> Option<EventResult> {
            self.get_inner_mut().focus_top()
        }
        fn focus_bottom(&mut self) -> Option<EventResult> {
            self.get_inner_mut().focus_bottom()
        }
        fn handle_digit(&mut self, c: char) -> Option<EventResult> {
            if '0' <= c && c <= '9' {
                self.add_raw_command_char(c);
                Some(EventResult::Consumed(None))
            } else {
                match c {
                    $(
                        $x => {},
                    )*
                    _ => {
                        self.clear_raw_command();
                    }
                };
                None
            }
        }
    };
}

impl ListEventView for StoryView {
    crate::list_event_view_wrapper!('g');
}

impl ListEventView for CommentView {
    crate::list_event_view_wrapper!('f');
}
