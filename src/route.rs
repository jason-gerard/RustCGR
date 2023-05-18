use crate::contact::Contact;

#[derive(Debug)]
pub struct Route {
    pub hops: Vec<Contact>,
}

impl Route {
    pub fn new(hops: Vec<Contact>) -> Self {
        Self { hops }
    }
}
