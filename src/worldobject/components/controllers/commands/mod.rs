pub mod move_command;
pub mod interact_action;
pub mod collect_command;
pub mod attack_command;
pub mod examine_command;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    Move(move_command::MoveCommand),
    Interact(interact_action::InteractAction),
    Collect(collect_command::CollectCommand),
    Attack(attack_command::AttackCommand),
    Examine(examine_command::ExamineCommand),
    Circumspect,
    Inventory,
}

#[derive(Debug)]
pub enum HumanActionParseError {
    NoActionNameProvided,
    InvalidActionName(String),
    MoveActionParseError(move_command::MoveActionParseError),
    InteractActionParseError(interact_action::InteractActionParseError),
    CollectActionParseError(collect_command::CollectCommandParseError),
    AttackActionParseError(attack_command::AttackActionParseError),
    ExamineActionParseError(examine_command::ExamineCommandParseError),
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

impl Command {
    fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, HumanActionParseError> {
        let action_name = words.next();

        match action_name {
            Some(borrowed_str) => match borrowed_str {
                "move" => move_command::MoveCommand::parse(words)
                    .map(Command::Move)
                    .map_err(HumanActionParseError::MoveActionParseError),
                "interact" => interact_action::InteractAction::parse(words)
                    .map(Command::Interact)
                    .map_err(HumanActionParseError::InteractActionParseError),
                "collect" => collect_command::CollectCommand::parse(words)
                    .map(Command::Collect)
                    .map_err(HumanActionParseError::CollectActionParseError),
                "attack" => attack_command::AttackCommand::parse(words)
                    .map(Command::Attack)
                    .map_err(HumanActionParseError::AttackActionParseError),
                "examine" => examine_command::ExamineCommand::parse(words)
                    .map(Command::Examine)
                    .map_err(HumanActionParseError::ExamineActionParseError),
                "circumspect" => Ok(Command::Circumspect),
                "inventory" => Ok(Command::Inventory),
                other => Err(HumanActionParseError::InvalidActionName(other.to_string())),
            },
            None => Err(HumanActionParseError::NoActionNameProvided),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = HumanActionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let words = value.trim().split(" ");
        Self::parse(&mut words.peekable())
    }
}