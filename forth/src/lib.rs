use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type ActionFn = fn(&mut Forth) -> Result;


#[derive(Default)]
pub struct Forth {
    nums_stack: Vec<Value>,
    definitions: HashMap<String, ActionFn>,
    str_to_def: HashMap<String, Vec<String>>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            nums_stack: Default::default(),
            definitions: HashMap::from([
                ("DROP".to_string(), Forth::drop_action as ActionFn),
                ("DUP".to_string(), Forth::dup_action as ActionFn),
                ("SWAP".to_string(), Forth::swap_action as ActionFn),
                ("OVER".to_string(), Forth::over_action as ActionFn),

                ("+".to_string(), Forth::add_action as ActionFn),
                ("-".to_string(), Forth::sub_action as ActionFn),
                ("*".to_string(), Forth::mul_action as ActionFn),
                ("/".to_string(), Forth::div_action as ActionFn),
            ]),
            str_to_def: Default::default()
        }
    }

    fn add_action(&mut self) -> Result {
        if let (Some(x), Some(y)) = (self.nums_stack.pop(), self.nums_stack.pop()) {
            self.nums_stack.push(y+x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn sub_action(&mut self) -> Result {
        if let (Some(x), Some(y)) = (self.nums_stack.pop(), self.nums_stack.pop()) {
            self.nums_stack.push(y-x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn mul_action(&mut self) -> Result {
        if let (Some(x), Some(y)) = (self.nums_stack.pop(), self.nums_stack.pop()) {
            self.nums_stack.push(y*x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn div_action(&mut self) -> Result {
        if let (Some(x), Some(y)) = (self.nums_stack.pop(), self.nums_stack.pop()) {
            let res = y.checked_div(x).ok_or(Error::DivisionByZero)?;
            self.nums_stack.push(res);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop_action(&mut self) -> Result {
        if self.nums_stack.pop().is_some() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn dup_action(&mut self) -> Result {
        if let Some(x) = self.nums_stack.get(self.nums_stack.len().saturating_sub(1)) {
            self.nums_stack.push(*x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap_action(&mut self) -> Result {
        let l = self.nums_stack.len();
        if l < 2 {
            Err(Error::StackUnderflow)
        } else {
            let temp = self.nums_stack[l-1];
            self.nums_stack[l-1] = self.nums_stack[l-2];
            self.nums_stack[l-2] = temp;
            Ok(())
        }
    }

    fn over_action(&mut self) -> Result {
        let l = self.nums_stack.len();
        if l < 2 {
            Err(Error::StackUnderflow)
        } else {
            self.nums_stack.push(self.nums_stack[l-2]);
            Ok(())
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.nums_stack
    }

    fn is_number(s: &[u8]) -> bool {
    
        // Check if the string is empty or starts with a '-' and the rest is digits
        if s.is_empty() {
            return false;
        }
    
        // Check if the string starts with a '-' or a digit and the rest are digits
        if s.starts_with(&[b'-']) {
            s.len() > 1 && s[1..].iter().all(|x| x.is_ascii_digit())
        } else {
            s.iter().all(|x| x.is_ascii_digit())
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut is_definition = false;
        let mut new_def: Option<String> = None;

        println!("{:?}", self.definitions);
        println!("{:?}", self.str_to_def);

        input.as_bytes().split(|x| *x==b' ').map(|x| {
            match x {
                x if Forth::is_number(x) => {
                    self.nums_stack.push(std::str::from_utf8(x).unwrap().parse::<Value>().unwrap());
                    Ok(())
                },
                x if x[0] == b':' => {
                    is_definition = true;
                    Ok(())
                },
                x if x[0] == b';' => {
                    is_definition = false;
                    new_def = None;
                    Ok(())
                },
                //* other special chars (+, -, *, / ...)
                _ => {
                    let def = std::str::from_utf8(&x.to_ascii_uppercase()).unwrap().to_string();

                    if !is_definition {

                        if let Some(k) = self.str_to_def.get(&def).cloned() {
                            k.iter().map(|x| {
                                if let Some(c) = self.definitions.get(x) {
                                    c(self)
                                } else {
                                    Err(Error::UnknownWord)
                                }
                            }).collect()
                        } else if let Some(k) = self.definitions.get(&def) {
                            k(self)
                        } else {
                            Err(Error::UnknownWord)
                        }
                    } else {
                        if def.chars().any(|c| c.is_numeric()) {
                            Err(Error::InvalidWord)
                        } else if new_def.is_some() {
                            self.str_to_def.entry(new_def.clone().unwrap()).or_default().push(def);
                            Ok(())
                        } else {
                            new_def = Some(def);
                            println!("new def: {:?}", new_def);
                            Ok(())
                        }
                    }
                }
            }
        }).collect()
    }
}