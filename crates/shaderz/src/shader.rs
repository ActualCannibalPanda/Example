use std::fmt;

pub enum ShaderType {
    INT,
    FLOAT,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::INT => "int",
                Self::FLOAT => "float",
            }
        )
    }
}

pub struct ArgumentList {
    pub arguments: Vec<(ShaderType, String)>,
}

impl ArgumentList {
    pub fn new(arguments: Vec<(ShaderType, String)>) -> Self {
        Self { arguments }
    }
}

impl fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.arguments.len() {
            0 => Ok(()),
            1 => {
                let arg = &self.arguments[0];
                write!(f, "{} {}", arg.0, arg.1)
            }
            _ => {
                write!(
                    f,
                    "{}",
                    self.arguments
                        .iter()
                        .map(|x| format!("{} {}", x.0, x.1))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
pub struct ShaderFunction {
    pub t: ShaderType,
    pub name: String,
    pub arguments: ArgumentList,
}

impl fmt::Display for ShaderFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}({}) {{\n\n}}", self.t, self.name, self.arguments)
    }
}
