use crate::util::CloneBox;

enum GrammaticalPerson {
    FirstPersonSingular,
    ThirdPersonSingular,
    SecondPersonOrPlural,
}

pub enum VerbPhrase {
    Transitive(TransitiveVerbPhrase),
    Intransitive(IntransitiveVerb),
}

impl VerbPhrase {
    pub fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match self {
            Self::Transitive(verb) => verb.conjugate(person),
            Self::Intransitive(verb) => verb.conjugate(person),
        }
    }
}

pub trait IntransitiveVerbTrait {
    fn conjugate(&self, person: &GrammaticalPerson) -> String;
}

pub struct IntransitiveVerb(Box<dyn IntransitiveVerbTrait>);

impl IntransitiveVerbTrait for IntransitiveVerb {
    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        self.0.conjugate(person)
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
pub struct TransitiveVerb(Box<dyn TransitiveVerbTrait>);

impl TransitiveVerb {
    pub fn with_direct_object(&self, object: String) -> TransitiveVerbPhrase {
        TransitiveVerbPhrase {
            verb: self.clone(),
            direct_object: object,
        }
    }
}

pub struct TransitiveVerbPhrase {
    pub verb: TransitiveVerb,
    pub direct_object: String,
}

impl TransitiveVerbPhrase {
    pub fn conjugate(&self, person: &GrammaticalPerson) -> String {
        format!("{} {}", self.verb.0.conjugate(person), self.direct_object)
    }
}