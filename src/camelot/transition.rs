use crate::camelot::{Key, Mode};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KeyTransition {
    Vertical,
    Diagonal,
    ChangeIndex(isize),
    MajorToMinor,
    FlatToMinor,
}

pub const fn harmonic_transitions() -> [KeyTransition; 10] {
    [
        KeyTransition::Vertical,
        KeyTransition::Diagonal,
        KeyTransition::MajorToMinor,
        KeyTransition::FlatToMinor,
        KeyTransition::ChangeIndex(1),
        KeyTransition::ChangeIndex(2),
        KeyTransition::ChangeIndex(7),
        KeyTransition::ChangeIndex(-1),
        KeyTransition::ChangeIndex(-2),
        KeyTransition::ChangeIndex(-7),
    ]
}

pub fn make_transition(scale: Key, transition: KeyTransition) -> Key {
    match transition {
        KeyTransition::Vertical => scale.swap_kind(),
        KeyTransition::ChangeIndex(amount) => scale.change_index(amount),
        KeyTransition::Diagonal if matches!(scale.mode, Mode::Major) => {
            scale.swap_kind().change_index(1)
        }
        KeyTransition::Diagonal if matches!(scale.mode, Mode::Minor) => {
            scale.swap_kind().change_index(-1)
        }
        KeyTransition::FlatToMinor if matches!(scale.mode, Mode::Minor) => {
            scale.swap_kind().change_index(-4)
        }
        KeyTransition::FlatToMinor if matches!(scale.mode, Mode::Major) => {
            scale.swap_kind().change_index(4)
        }
        KeyTransition::MajorToMinor if matches!(scale.mode, Mode::Minor) => {
            scale.swap_kind().change_index(3)
        }
        KeyTransition::MajorToMinor if matches!(scale.mode, Mode::Major) => {
            scale.swap_kind().change_index(-3)
        }
        _ => unreachable!(),
    }
}