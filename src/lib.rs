pub trait Convert {
    fn convert(&self) -> String;
}

impl Convert for usize {
    fn convert(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy)]
pub struct Identifier(usize);

impl Identifier {
    pub fn new(id: usize) -> Self {
        Identifier(id)
    }
}

impl Convert for Identifier {
    fn convert(&self) -> String {
        format!("identifier_{}", self.0)
    }
}

pub enum Type {
    Int32
}

impl Convert for Type {
    fn convert(&self) -> String {
        match self {
            Type::Int32 => "int".to_string()
        }
    }
}

pub enum Instruction {
    Return(Box<dyn Convert>)
}

impl Convert for Instruction {
    fn convert(&self) -> String {
        match self {
            Instruction::Return(data) => format!("return {};", data.convert())
        }
    }
}

pub struct Variable {
    pub identifier: Identifier,
    pub r#type: Type
}

pub struct Function {
    pub identifier: Identifier,
    pub parameters: Vec<Variable>,
    pub returns: Type,
    pub instructions: Vec<Instruction>
}

pub struct Graph {
    pub functions: Vec<Function>,
    pub main: Option<Identifier>
}

impl Convert for Graph {
    fn convert(&self) -> String {
        let mut functions = self.functions.iter()
            .map(Convert::convert)
            .collect::<Vec<String>>();

        if let Some(main) = self.main {
            functions.push(format!(
                "int main(void) {{ return {}(); }}",
                main.convert()
            ));
        }

        functions.join("\n\n")
    }
}

impl Convert for Function {
    fn convert(&self) -> String {
        let parameters = self.parameters.iter()
            .map(|variable| format!(
                "{} {}",
                variable.r#type.convert(),
                variable.identifier.convert()
            ))
            .collect::<Vec<String>>()
            .join(", ");

        let instructions = self.instructions.iter()
            .map(|instruction| instruction.convert())
            .collect::<Vec<String>>()
            .join("\n\t");

        format!(
            "{} {}({}) {{\n\t{}\n}}",
            self.returns.convert(),
            self.identifier.convert(),
            parameters,
            instructions
        )
    }
}

