pub enum SymType<'b> {
    Var(f32),
    Func(&'b dyn FnOnce(f32) -> f32),
}

pub struct Symtable<'a>(Vec<(String, SymType<'a>)>);

impl<'a> Symtable<'a> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn set_value(&mut self, name: String, value: SymType) {
        match self.0.iter().find(|(id, _)| name == *id) {
            Some(var) => var.1 = value,
            None => self.0.push((name, value)),
        }
    }

    fn get_value(&self, name: String) -> Option<f32> {
        if let Some((_, value)) = self.0.iter().find(|(id, _)| name == *id) {
            match value {
                SymType::Var(value) => return Some(*value),
                SymType::Func(_) => {
                    println!("Can't get the value of that variable, it's a function!");
                    return None;
                }
            }
        }

        println!("That variable doesn't exist yet!");

        None
    }

    fn apply_func(&self, name:String, param: f32) -> Option<f32> {
        
        if let Some((_, value)) = self.0.iter().find(|(id, _)| name == *id) {
            match value {
                SymType::Var(value) => {
                    println!("That id isn't a function!");
                    return None;
                }
                SymType::Func(func) => {
                    return Some(func(param));
                }
            }
        }

        println!("That function doesn't exist yet!");

        None
    }
}
