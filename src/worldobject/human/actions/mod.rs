pub mod move_action;
pub mod interact_action;
pub mod collect_action;
pub mod attack_action;
pub mod examine_action;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum HumanAction {
    Move(move_action::MoveAction),
    Interact(interact_action::InteractAction),
    Collect(collect_action::CollectAction),
    Attack(attack_action::AttackAction),
    Examine(examine_action::ExamineAction),
    Circumspect,
    Inventory,
}

#[derive(Debug)]
pub enum HumanActionParseError {
    NoActionNameProvided,
    InvalidActionName(String),
    MoveActionParseError(move_action::MoveActionParseError),
    InteractActionParseError(interact_action::InteractActionParseError),
    CollectActionParseError(collect_action::CollectActionParseError),
    AttackActionParseError(attack_action::AttackActionParseError),
    ExamineActionParseError(examine_action::ExamineActionParseError),
}

impl std::fmt::Display for HumanActionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoActionNameProvided => write!(f, "no action name provided"),
            Self::InvalidActionName(name) => write!(f, "invalid action name \"{}\"", name),
            Self::MoveActionParseError(move_err) => write!(f, "failed to parse move action: {}", move_err),
            Self::CollectActionParseError(collect_err) => write!(f, "failed to parse collect action: {}", collect_err),
            Self::AttackActionParseError(attack_err) => write!(f, "failed to parse attack action: {}", attack_err),
            Self::ExamineActionParseError(examine_err) => write!(f, "failed to parse examine action: {}", examine_err),
            Self::InteractActionParseError(interact_err) => write!(f, "failed to parse interact action: {}", interact_err)
        }
    }
}

impl std::error::Error for HumanActionParseError {}

impl HumanAction {
    fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, HumanActionParseError> {
        let action_name = words.next();

        match action_name {
            Some(borrowed_str) => match borrowed_str {
                "move" => move_action::MoveAction::parse(words)
                    .map(HumanAction::Move)
                    .map_err(HumanActionParseError::MoveActionParseError),
                "interact" => interact_action::InteractAction::parse(words)
                    .map(HumanAction::Interact)
                    .map_err(HumanActionParseError::InteractActionParseError),
                "collect" => collect_action::CollectAction::parse(words)
                    .map(HumanAction::Collect)
                    .map_err(HumanActionParseError::CollectActionParseError),
                "attack" => attack_action::AttackAction::parse(words)
                    .map(HumanAction::Attack)
                    .map_err(HumanActionParseError::AttackActionParseError),
                "examine" => examine_action::ExamineAction::parse(words)
                    .map(HumanAction::Examine)
                    .map_err(HumanActionParseError::ExamineActionParseError),
                "circumspect" => Ok(HumanAction::Circumspect),
                "inventory" => Ok(HumanAction::Inventory),
                other => Err(HumanActionParseError::InvalidActionName(other.to_string())),
            },
            None => Err(HumanActionParseError::NoActionNameProvided),
        }
    }
}

impl TryFrom<&str> for HumanAction {
    type Error = HumanActionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let words = value.trim().split(" ");
        Self::parse(&mut words.peekable())
    }
}