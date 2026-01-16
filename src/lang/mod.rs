pub mod verbs;

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