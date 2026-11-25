#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationKey {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Character(char),
    Control(char),
    Alt(char),
}

impl NavigationKey {
    pub const fn character(value: char) -> Self {
        Self::Character(value)
    }

    pub const fn control(value: char) -> Self {
        Self::Control(value)
    }

    pub const fn alt(value: char) -> Self {
        Self::Alt(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_create_key_values() {
        assert_eq!(NavigationKey::character('x'), NavigationKey::Character('x'));
        assert_eq!(NavigationKey::control('d'), NavigationKey::Control('d'));
        assert_eq!(NavigationKey::alt('v'), NavigationKey::Alt('v'));
    }
}
