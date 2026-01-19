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

impl IntransitiveVerbTrait for ToInteract {
    fn clone_box(&self) -> Box<dyn IntransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("interacts"),
            _ => String::from("interact"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToExamine;

impl TransitiveVerbTrait for ToExamine {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("examines"),
            _ => String::from("examine"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToCircumspect;

impl IntransitiveVerbTrait for ToCircumspect {
    fn clone_box(&self) -> Box<dyn IntransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("circumspects"),
            _ => String::from("circumspect"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToCheck;

impl TransitiveVerbTrait for ToCheck {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("checks"),
            _ => String::from("check"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToWield;

impl TransitiveVerbTrait for ToWield {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("wields"),
            _ => String::from("wield"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToUse;

impl TransitiveVerbTrait for ToUse {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("uses"),
            _ => String::from("use"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ToCast;

impl TransitiveVerbTrait for ToCast {
    fn clone_box(&self) -> Box<dyn TransitiveVerbTrait> {
        Box::new(*self)
    }

    fn conjugate(&self, person: &GrammaticalPerson) -> String {
        match person {
            GrammaticalPerson::ThirdPersonSingularGendered => String::from("casts"),
            _ => String::from("cast"),
        }
    }
}