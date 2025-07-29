pub use std::{cell::RefCell, rc::Rc};

const DEFAULT_ATTACK: u8 = 8;

#[derive(PartialEq, Debug)]
pub struct Player {
    name: String,
    hp: u8, // 100 is max, 2^8=256
    attack: u8,
}

impl Player {
    /// Creates a new player
    ///
    /// Examples
    /// ```
    /// use player_state::player::Player;
    ///
    /// let player = Player::new("4", "Name");
    ///
    /// assert_eq!(player.get_attack(), 4);
    /// assert_eq!(player.get_health(), 100);
    ///
    /// // Setting too large number
    /// let player = Player::new("1000000000", "Another Name");
    ///
    /// assert_eq!(player.get_attack(), Player::get_default_attack());
    /// assert_eq!(player.get_health(), 100);
    /// ```
    #[must_use]
    pub fn new(attack: &str, name: &str) -> Self {
        let parsed_attack: u8 = attack.trim().parse().unwrap_or_else(|e| {
            println!("{e}\nsetting to default: {DEFAULT_ATTACK}");
            DEFAULT_ATTACK
        });
        Self {
            name: name.to_string(),
            hp: 100,
            attack: parsed_attack,
        }
    }

    #[must_use]
    pub fn get_health(&self) -> u8 {
        self.hp
    }

    #[must_use]
    pub fn get_attack(&self) -> u8 {
        self.attack
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn get_default_attack() -> u8 {
        DEFAULT_ATTACK
    }

    pub fn set_health(&mut self, amount: u8) {
        self.hp = amount;
    }

    pub fn damage(&self, enemy: &Rc<RefCell<Player>>) {
        let enemy_new_health = enemy.borrow().hp.saturating_sub(self.attack);
        enemy.borrow_mut().set_health(enemy_new_health);
    }

    pub fn heal(&mut self, amount: u8) {
        let mut new_health = self.hp.saturating_add(amount);
        if new_health > 100 {
            new_health = 100;
        }
        self.hp = new_health;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_player_new_ok() {
        let player = Player::new("4", "A");

        assert_eq!(player.get_attack(), 4);
        assert_eq!(player.get_health(), 100);
        assert_eq!(player.get_name(), "A");
    }

    #[test]
    fn test_player_new_too_big_attack() {
        let player = Player::new("1000000000", "A");

        assert_eq!(player.get_attack(), Player::get_default_attack());
        assert_eq!(player.get_health(), 100);
    }

    #[test]
    fn test_player_new_negative_attack() {
        let player = Player::new("-12", "A");

        assert_eq!(player.get_attack(), Player::get_default_attack());
        assert_eq!(player.get_health(), 100);
    }

    #[test]
    fn test_player_attacked_ok() {
        let player = Player::new("10", "A");
        let enemy = Rc::new(RefCell::new(Player::new("10", "B")));

        player.damage(&enemy);

        assert_eq!(enemy.borrow().get_health(), 90);
    }

    #[test]
    fn test_player_attacked_too_much() {
        let player = Player::new("101", "A");
        let enemy = Rc::new(RefCell::new(Player::new("10", "B")));

        player.damage(&enemy);

        assert_eq!(enemy.borrow().get_health(), 0);
    }

    #[test]
    fn test_player_attacked_negative() {
        let player = Player::new("-12", "A");
        let enemy = Rc::new(RefCell::new(Player::new("10", "B")));

        player.damage(&enemy);

        let enemy_health = enemy.borrow().get_health();
        assert_eq!(enemy_health, 100 - player.get_attack());
    }

    #[test]
    fn test_player_heal_ok() {
        let player = Rc::new(RefCell::new(Player::new("10", "A")));
        let enemy = Rc::new(RefCell::new(Player::new("10", "B")));

        enemy.borrow().damage(&player);
        assert_eq!(player.borrow().get_health(), 90);

        let health_potion = 5u8;
        player.borrow_mut().heal(health_potion);

        assert_eq!(player.borrow().get_health(), 95);
    }

    #[test]
    fn test_player_heal_too_big() {
        let player = Rc::new(RefCell::new(Player::new("10", "A")));
        let enemy = Rc::new(RefCell::new(Player::new("10", "B")));

        enemy.borrow().damage(&player);
        assert_eq!(player.borrow().get_health(), 90);

        let health_potion = 100u8;
        player.borrow_mut().heal(health_potion);

        assert_eq!(player.borrow().get_health(), 100);
    }
}
