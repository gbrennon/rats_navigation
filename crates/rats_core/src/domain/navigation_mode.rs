#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavigationMode {
    #[default]
    Standard,
    Vim,
    Emacs,
}

impl NavigationMode {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Vim, Self::Emacs];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Vim => "vim",
            Self::Emacs => "emacs",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Vim => "Vim",
            Self::Emacs => "Emacs",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_expose_metadata() {
        let metadata = [
            (NavigationMode::Standard, "standard", "Standard"),
            (NavigationMode::Vim, "vim", "Vim"),
            (NavigationMode::Emacs, "emacs", "Emacs"),
        ];

        metadata.into_iter().for_each(|(mode, value, label)| {
            assert_eq!(mode.as_str(), value);
            assert_eq!(mode.label(), label);
        });
        assert_eq!(NavigationMode::default(), NavigationMode::Standard);
        assert_eq!(NavigationMode::ALL.len(), 3);
    }
}
