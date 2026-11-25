use crate::domain::{NavigationAction, NavigationKey, NavigationMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMapper {
    mode: NavigationMode,
    pending_vim_g: bool,
}

impl NavigationMapper {
    pub const fn new(mode: NavigationMode) -> Self {
        Self {
            mode,
            pending_vim_g: false,
        }
    }

    pub const fn mode(&self) -> NavigationMode {
        self.mode
    }

    pub const fn set_mode(self, mode: NavigationMode) -> Self {
        Self {
            mode,
            pending_vim_g: false,
        }
    }

    pub fn map(self, key: NavigationKey) -> (Self, Option<NavigationAction>) {
        if key == NavigationKey::Character('?') {
            return (
                Self {
                    pending_vim_g: false,
                    ..self
                },
                Some(NavigationAction::Help),
            );
        }

        match self.mode {
            NavigationMode::Standard => (
                Self {
                    pending_vim_g: false,
                    ..self
                },
                Self::map_standard(key),
            ),
            NavigationMode::Vim => self.map_vim(key),
            NavigationMode::Emacs => (
                Self {
                    pending_vim_g: false,
                    ..self
                },
                Self::map_emacs(key),
            ),
        }
    }

    fn map_standard(key: NavigationKey) -> Option<NavigationAction> {
        match key {
            NavigationKey::Up | NavigationKey::Character('k') => Some(NavigationAction::Previous),
            NavigationKey::Down | NavigationKey::Character('j') => Some(NavigationAction::Next),
            NavigationKey::Left | NavigationKey::Character('h') => Some(NavigationAction::Left),
            NavigationKey::Right | NavigationKey::Character('l') => Some(NavigationAction::Right),
            NavigationKey::PageUp => Some(NavigationAction::PageUp),
            NavigationKey::PageDown => Some(NavigationAction::PageDown),
            _ => None,
        }
    }

    fn map_vim(self, key: NavigationKey) -> (Self, Option<NavigationAction>) {
        if self.pending_vim_g {
            return (
                Self {
                    pending_vim_g: false,
                    ..self
                },
                (key == NavigationKey::Character('g')).then_some(NavigationAction::Top),
            );
        }

        if key == NavigationKey::Character('g') {
            return (
                Self {
                    pending_vim_g: true,
                    ..self
                },
                None,
            );
        }

        (
            Self {
                pending_vim_g: false,
                ..self
            },
            match key {
                NavigationKey::Character('h') => Some(NavigationAction::Left),
                NavigationKey::Character('j') => Some(NavigationAction::Next),
                NavigationKey::Character('k') => Some(NavigationAction::Previous),
                NavigationKey::Character('l') => Some(NavigationAction::Right),
                NavigationKey::Character('G') => Some(NavigationAction::Bottom),
                NavigationKey::Control('d') => Some(NavigationAction::PageDown),
                NavigationKey::Control('u') => Some(NavigationAction::PageUp),
                _ => None,
            },
        )
    }

    fn map_emacs(key: NavigationKey) -> Option<NavigationAction> {
        match key {
            NavigationKey::Control('f') => Some(NavigationAction::Right),
            NavigationKey::Control('b') => Some(NavigationAction::Left),
            NavigationKey::Control('n') => Some(NavigationAction::Next),
            NavigationKey::Control('p') => Some(NavigationAction::Previous),
            NavigationKey::Control('a') => Some(NavigationAction::Top),
            NavigationKey::Control('e') => Some(NavigationAction::Bottom),
            NavigationKey::Control('v') => Some(NavigationAction::PageDown),
            NavigationKey::Alt('v') => Some(NavigationAction::PageUp),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_action(
        mapper: NavigationMapper,
        key: NavigationKey,
        expected: Option<NavigationAction>,
    ) {
        let (_, actual) = mapper.map(key);
        assert_eq!(actual, expected);
    }

    #[test]
    fn standard_maps_directional_and_paging_inputs() {
        let mapper = NavigationMapper::new(NavigationMode::Standard);
        [
            (NavigationKey::Up, NavigationAction::Previous),
            (NavigationKey::Character('k'), NavigationAction::Previous),
            (NavigationKey::Down, NavigationAction::Next),
            (NavigationKey::Character('j'), NavigationAction::Next),
            (NavigationKey::Left, NavigationAction::Left),
            (NavigationKey::Character('h'), NavigationAction::Left),
            (NavigationKey::Right, NavigationAction::Right),
            (NavigationKey::Character('l'), NavigationAction::Right),
            (NavigationKey::PageUp, NavigationAction::PageUp),
            (NavigationKey::PageDown, NavigationAction::PageDown),
        ]
        .into_iter()
        .for_each(|(key, action)| assert_action(mapper, key, Some(action)));
    }

    #[test]
    fn vim_maps_motions_sequences_and_paging_inputs() {
        let mapper = NavigationMapper::new(NavigationMode::Vim);
        [
            (NavigationKey::Character('h'), NavigationAction::Left),
            (NavigationKey::Character('j'), NavigationAction::Next),
            (NavigationKey::Character('k'), NavigationAction::Previous),
            (NavigationKey::Character('l'), NavigationAction::Right),
            (NavigationKey::Character('G'), NavigationAction::Bottom),
            (NavigationKey::Control('d'), NavigationAction::PageDown),
            (NavigationKey::Control('u'), NavigationAction::PageUp),
        ]
        .into_iter()
        .for_each(|(key, action)| assert_action(mapper, key, Some(action)));

        let (pending, action) = mapper.map(NavigationKey::Character('g'));
        assert_eq!(action, None);
        let (complete, action) = pending.map(NavigationKey::Character('g'));
        assert_eq!(complete.mode(), NavigationMode::Vim);
        assert_eq!(action, Some(NavigationAction::Top));

        let (pending, action) = mapper.map(NavigationKey::Character('g'));
        assert_eq!(action, None);
        let (_, action) = pending.map(NavigationKey::Character('x'));
        assert_eq!(action, None);
    }

    #[test]
    fn emacs_maps_control_and_alt_inputs() {
        let mapper = NavigationMapper::new(NavigationMode::Emacs);
        [
            (NavigationKey::Control('f'), NavigationAction::Right),
            (NavigationKey::Control('b'), NavigationAction::Left),
            (NavigationKey::Control('n'), NavigationAction::Next),
            (NavigationKey::Control('p'), NavigationAction::Previous),
            (NavigationKey::Control('a'), NavigationAction::Top),
            (NavigationKey::Control('e'), NavigationAction::Bottom),
            (NavigationKey::Control('v'), NavigationAction::PageDown),
            (NavigationKey::Alt('v'), NavigationAction::PageUp),
        ]
        .into_iter()
        .for_each(|(key, action)| assert_action(mapper, key, Some(action)));
    }

    #[test]
    fn help_is_available_and_unknown_inputs_are_ignored() {
        NavigationMode::ALL.into_iter().for_each(|mode| {
            let mapper = NavigationMapper::new(mode);
            assert_action(
                mapper,
                NavigationKey::Character('?'),
                Some(NavigationAction::Help),
            );
            assert_action(mapper, NavigationKey::Character('x'), None);
        });
    }

    #[test]
    fn changing_mode_resets_pending_vim_sequence() {
        let (pending, _) =
            NavigationMapper::new(NavigationMode::Vim).map(NavigationKey::Character('g'));
        let standard = pending.set_mode(NavigationMode::Standard);
        assert_eq!(standard.mode(), NavigationMode::Standard);
        assert_action(standard, NavigationKey::Character('g'), None);
    }
}
