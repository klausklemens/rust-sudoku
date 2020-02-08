#[derive(Copy, Clone)]
pub enum FieldContent {
    Digit(u8),
    Hints([bool; 9]),
    None
}

impl FieldContent {
    pub fn is_some(&self) -> bool {
        return match *self {
            FieldContent::Digit(_) => true,
            _ => false
        }
    }
    
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
    
    pub fn unwrap(&self) -> u8 {
        let wrapper: Option<u8> = match *self {
            FieldContent::Digit(d) => Some(d),
            _ => None
        };
        return wrapper.unwrap();
    }
}
