pub use std::{cell::RefCell, rc::Rc};

pub use crate::Player;

#[derive(Debug)]
pub enum ArenaErr {
    SameFighterErr,
}

pub struct Arena {
    events: Rc<RefCell<Vec<Rc<Event>>>>,
    fighters: Rc<RefCell<Vec<Rc<RefCell<Player>>>>>,
}

#[derive(Debug)]
pub enum Event {
    Duel(Rc<RefCell<Player>>, Rc<RefCell<Player>>),
}

pub trait Perform {
    fn perform(&self) -> Result<(), ArenaErr>;
}

impl Perform for Event {
    fn perform(&self) -> Result<(), ArenaErr> {
        match self {
            Event::Duel(p1, p2) => {
                println!(
                    "Ladies and gentlemen!\nWelcome to the duel of{} and {}",
                    p1.borrow().get_name(),
                    p2.borrow().get_name()
                );
                p1.borrow().damage(&Rc::clone(p2));
            }
        }
        Ok(())
    }
}

impl Arena {
    pub fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
            fighters: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get_fighters(&self) -> &Rc<RefCell<Vec<Rc<RefCell<Player>>>>> {
        &self.fighters
    }

    pub fn get_events(&self) -> &Rc<RefCell<Vec<Rc<Event>>>> {
        &self.events
    }

    /// Adds new event, if the participants not in the fighter vec, they will be added!
    ///
    /// Examples
    ///
    /// ```
    /// use player_state::{Arena, Player, Event};
    /// pub use std::{rc::Rc, cell::RefCell};
    ///
    ///  let arena = Arena::new();
    /// let duel = Rc::new(Event::Duel(
    /// Rc::new(RefCell::new(Player::new("10", "A"))),
    /// Rc::new(RefCell::new(Player::new("10", "B"))),
    /// ));
    /// if let Err(e) = arena.add_event(&duel) {
    ///  panic!("Error while adding event {duel:?} : {e:?}");
    /// }
    /// let events = arena.get_events();
    ///  assert_eq!(events.borrow().len(), 1);
    ///  let fighters = arena.get_fighters().borrow();
    ///  assert_eq!(fighters.len(), 2);
    ///  assert_eq!(fighters.get(0).unwrap().borrow().get_name(), "A");
    ///  assert_eq!(fighters.get(1).unwrap().borrow().get_name(), "B");
    /// ```
    ///
    /// Errors
    /// - If the participants of Duel are the same person
    pub fn add_event(&self, event: &Rc<Event>) -> Result<(), ArenaErr> {
        let e = event.as_ref();
        match e {
            Event::Duel(p1, p2) => {
                if p1 == p2 {
                    return Err(ArenaErr::SameFighterErr);
                }
                println!(
                    "Player {} and player {} are having a DUEL!!!",
                    p1.borrow().get_name(),
                    p2.borrow().get_name()
                );
                if !self.fighters.borrow().contains(p1) {
                    self.add_fighter(p1);
                }
                if !self.fighters.borrow().contains(p2) {
                    self.add_fighter(p2);
                }
            }
        }
        self.events.borrow_mut().push(Rc::clone(&event));
        Ok(())
    }

    pub fn handle_events(&self) {
        self.get_events().borrow().iter().for_each(|event| {
            if let Err(e) = event.perform() {
                println!("Could not perform {event:?}! | Cause: {e:?}");
            };
        });
    }

    pub fn add_fighter(&self, fighter: &Rc<RefCell<Player>>) {
        self.fighters.borrow_mut().push(Rc::clone(fighter));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arena_new_ok() {
        let arena = Arena::new();

        let events = arena.events;
        let fighters = arena.fighters;

        assert_eq!(Rc::strong_count(&events), 1);
        assert_eq!(Rc::weak_count(&events), 0);

        assert_eq!(Rc::strong_count(&fighters), 1);
        assert_eq!(Rc::weak_count(&fighters), 0);
    }

    #[test]
    fn test_arena_add_event_ok() {
        let arena = Arena::new();
        let duel = Rc::new(Event::Duel(
            Rc::new(RefCell::new(Player::new("10", "A"))),
            Rc::new(RefCell::new(Player::new("10", "B"))),
        ));
        if let Err(e) = arena.add_event(&duel) {
            panic!("Error while adding event {duel:?} : {e:?}");
        }
        let events = arena.events;

        assert_eq!(events.borrow().len(), 1);

        let fighters = arena.fighters.borrow();
        assert_eq!(fighters.len(), 2);
        assert_eq!(fighters.get(0).unwrap().borrow().get_name(), "A");
        assert_eq!(fighters.get(1).unwrap().borrow().get_name(), "B");
    }

    #[test]
    #[should_panic]
    fn test_arena_add_event_same_fighter_fail() {
        let arena = Arena::new();
        let duel = Rc::new(Event::Duel(
            Rc::new(RefCell::new(Player::new("10", "A"))),
            Rc::new(RefCell::new(Player::new("10", "A"))),
        ));
        if let Err(e) = arena.add_event(&duel) {
            panic!("Error while adding event {duel:?} : {e:?}");
        }
    }
}
