use crate::lang::{
    GrammaticalPerson,
    TransitiveVerbTrait,
    IntransitiveVerbTrait,
};

#[derive(Copy, Clone)]
pub struct ToDo;

impl TransitiveVerbTrait for ToDo {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("does"),
            _ => String::from("do"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToCollect;

impl TransitiveVerbTrait for ToCollect {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("collects"),
            _ => String::from("collect"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToMove;

impl IntransitiveVerbTrait for ToMove {
    fn clone_box(&self) -> Box<dyn IntransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("moves"),
            _ => String::from("move"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToAttack;

impl TransitiveVerbTrait for ToAttack {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("attacks"),
            _ => String::from("attack"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToInteract;

impl TransitiveVerbTrait for ToInteract {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("interacts"),
            _ => String::from("interact"),
        }
    }
}