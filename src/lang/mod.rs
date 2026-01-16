pub mod verbs;

use std::fmt::{Display, Formatter, Error};

pub enum GrammaticalPerson {
    FirstPersonSingular,

    SecondPersonSingular,

    ThirdPersonSingularGendered,
    ThirdPersonSingularNeuter,

    Plural,
}

#[derive(Clone)]
pub enum VerbPhrase {
    Transitive(TransitiveVerbPhrase),
    Intransitive(IntransitiveVerb),
    Prepositional(PrepositionalVerbPhrase),
}

impl VerbPhrase {
    pub fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match self {
            Self::Transitive(verb) => verb.conjugate(person),
            Self::Intransitive(verb) => verb.conjugate(person),
            Self::Prepositional(verb) => format!("{} {}", verb.main_verb_phrase.conjugate(person), verb.prepositional_phrase.to_string()),
        }
    }
}

pub trait IntransitiveVerbTrait {
    fn clone_box(&self) -> Box<dyn IntransitiveVerbTrait>;
    fn conjugate(&self, person: &GrammaticalPerson) -> String;
}

impl Clone for Box<dyn IntransitiveVerbTrait> {
    fn clone(&self) -> Box<dyn IntransitiveVerbTrait> {
        (**self).clone_box()
    }
}

#[derive(Clone)]
pub struct IntransitiveVerb(pub Box<dyn IntransitiveVerbTrait>);

impl IntransitiveVerb {
    pub fn new(verb: impl IntransitiveVerbTrait + 'static) -> Self {
        Self(Box::new(verb))
    }
}

impl IntransitiveVerbTrait for IntransitiveVerb {
    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        self.0.conjugate(person)
    }

    fn clone_box(&self) -> Box<dyn IntransitiveVerbTrait> {
        Box::new(self.clone())
    }
}

pub trait TransitiveVerbTrait {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait>;
    fn conjugate(&self, person: &GrammaticalPerson) -> String;
}

impl Clone for Box<dyn TransitiveVerbTrait> {
    fn clone(&self) -> Box<dyn TransitiveVerbTrait> {
        (**self).clone_box()
    }
}

#[derive(Clone)]
pub struct TransitiveVerb(pub Box<dyn TransitiveVerbTrait>);

impl TransitiveVerb {
    pub fn new(verb: impl TransitiveVerbTrait + 'static) -> Self {
        Self(Box::new(verb))
    }

    pub fn with_direct_object(&self, object: String) -> TransitiveVerbPhrase {
        TransitiveVerbPhrase {
            verb: self.clone(),
            direct_object: object,
        }
    }
}

#[derive(Clone)]
pub struct TransitiveVerbPhrase {
    pub verb: TransitiveVerb,
    pub direct_object: String,
}

impl TransitiveVerbPhrase {
    pub fn conjugate(&self, person: &GrammaticalPerson) -> String {
        format!("{} {}", self.verb.0.conjugate(person), self.direct_object)
    }
}

#[derive(Clone)]
pub struct PrepositionalVerbPhrase {
    pub main_verb_phrase: Box<VerbPhrase>,
    pub prepositional_phrase: PrepositionalPhrase,
}

#[derive(Clone)]
pub struct PrepositionalPhrase {
    pub preposition: String,
    pub object: String,
}

impl Display for PrepositionalPhrase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.preposition, self.object)
    }
}