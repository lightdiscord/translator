#[derive(Default)]
pub struct IdentifierFactory(usize);

impl IdentifierFactory {
    pub fn new(&mut self) -> Identifier {
        let identifier = Identifier::new(self.0);

        self.0 += 1;
        identifier
    }
}

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

#[derive(Clone, Copy)]
pub enum Type {
    Int32
}

impl Convert for Type {
    fn convert(&self) -> String {
        match self {
            Type::Int32 => "int".to_string(),
        }
    }
}

pub struct Remainder<A, B>(pub A, pub B);

impl<A: Convert, B: Convert> Convert for Remainder<A, B> {
    fn convert(&self) -> String {
        let Remainder(a, b) = self;

        format!("{} % {}", a.convert(), b.convert())
    }
}

pub struct Divide<A, B>(pub A, pub B);

impl<A: Convert, B: Convert> Convert for Divide<A, B> {
    fn convert(&self) -> String {
        let Divide(a, b) = self;

        format!("{} / {}", a.convert(), b.convert())
    }
}

pub struct Plus<A, B>(pub A, pub B);

impl<A: Convert, B: Convert> Convert for Plus<A, B> {
    fn convert(&self) -> String {
        let Plus(a, b) = self;

        format!("{} + {}", a.convert(), b.convert())
    }
}

impl Convert for (usize, &Vec<Instruction>) {
    fn convert(&self) -> String {
        let (padding, instructions) = self;
        let padding = "\t".repeat(*padding);

        instructions.iter()
            .map(|instruction| format!("{}{}\n", padding, instruction.convert()))
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Convert for Vec<Instruction> {
    fn convert(&self) -> String {
        (1, self).convert()
    }
}

pub enum Comparison<A, B> {
    Equals(A, B),
    NotEquals(A, B),
    GreaterThan(A, B),
    LessThan(A, B)
}

impl<A: Convert, B: Convert> Convert for Comparison<A, B> {
    fn convert(&self) -> String {
        match self {
            Comparison::Equals(a, b) => format!("{} == {}", a.convert(), b.convert()),
            Comparison::NotEquals(a, b) => format!("{} != {}", a.convert(), b.convert()),
            Comparison::GreaterThan(a, b) => format!("{} > {}", a.convert(), b.convert()),
            Comparison::LessThan(a, b) => format!("{} < {}", a.convert(), b.convert())
        }
    }
}

pub struct Call<A>(pub A, pub Vec<Box<dyn Convert>>);

impl<A: Convert> Convert for Call<A> {
    fn convert(&self) -> String {
        let Call(function, parameters) = self;
        
        let parameters = parameters.iter().map(|param| param.convert()).collect::<Vec<String>>().join(", ");

        format!("{}({})", function.convert(), parameters)
    }
}

pub enum Instruction {
    ReadLn(Variable),
    WriteLn(Variable),
    Declare(Variable),

    Return(Box<dyn Convert>),
    Assign(Identifier, Box<dyn Convert>),
    If {
        condition: Box<dyn Convert>,
        instructions: Vec<Instruction>
    },
    While {
        condition: Box<dyn Convert>,
        instructions: Vec<Instruction>
    },

    /// Should not be used because it is language specific.
    Custom(String)
}

impl Convert for Instruction {
    fn convert(&self) -> String {
        match self {
            Instruction::ReadLn(variable) => {
                let instructions = vec![
                    Instruction::Declare(*variable),
                    Instruction::Custom(format!(r#"scanf("%d", &{});"#, variable.identifier.convert()))
                ];

                instructions.convert()
            }
            Instruction::WriteLn(variable) => {
                Instruction::Custom(format!(r#"printf("%d", {});"#, variable.identifier.convert())).convert()
            },
            Instruction::Declare(variable) => {
                format!("{} {};", variable.r#type.convert(), variable.identifier.convert())
            },
            Instruction::Return(data) => format!("return {};", data.convert()),
            Instruction::Assign(identifier, data) => format!("{} = {};", identifier.convert(), data.convert()),
            Instruction::If { condition, instructions } => {
                format!("if ({}) {{\n{}}}", condition.convert(), instructions.convert())
            }
            Instruction::While { condition, instructions } => {
                format!("while ({}) {{\n{}}}", condition.convert(), instructions.convert())
            },
            Instruction::Custom(content) => content.to_string()
        }
    }
}

#[derive(Clone, Copy)]
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

        format!(
            "{} {}({}) {{\n{}}}",
            self.returns.convert(),
            self.identifier.convert(),
            parameters,
            self.instructions.convert()
        )
    }
}

