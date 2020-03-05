use translator::*;

fn sum_digits(factory: &mut IdentifierFactory) -> Function {
    let identifier = factory.new();
    let n = factory.new();
    let n_variable = Variable { identifier: n, r#type: Type::Int32 };

    Function {
        identifier,
        returns: Type::Int32,
        parameters: vec![n_variable],
        instructions: vec![
            Instruction::If {
                condition: Box::new(Comparison::Equals(n, 0)),
                instructions: vec![
                    Instruction::Return(Box::new(0))
                ]
            },
            Instruction::Return(Box::new(
                Plus(
                    Remainder(n, 10),
                    Call(identifier, vec![Box::new(Divide(n, 10))])
                )
            ))
        ]
    }
}

fn next_item(factory: &mut IdentifierFactory, sum_digits: Identifier) -> Function {
    let identifier = factory.new();
    let n = factory.new();
    let n_variable = Variable { identifier: n, r#type: Type::Int32 };

    Function {
        identifier,
        returns: Type::Int32,
        parameters: vec![n_variable],
        instructions: vec![
            Instruction::Return(Box::new(
                Plus(n, Call(sum_digits, vec![Box::new(n)]))
            ))
        ]
    }
}

fn meeting_point(factory: &mut IdentifierFactory, next_item: Identifier) -> Function {
    let identifier = factory.new();
    let a = factory.new();
    let a_variable = Variable { identifier: a, r#type: Type::Int32 };
    let b = factory.new();
    let b_variable = Variable { identifier: b, r#type: Type::Int32 };

    Function {
        identifier,
        returns: Type::Int32,
        parameters: vec![a_variable, b_variable],
        instructions: vec![
            Instruction::While {
                condition: Box::new(Comparison::NotEquals(a, b)),
                instructions: vec![
                    Instruction::If {
                        condition: Box::new(Comparison::GreaterThan(a, b)),
                        instructions: vec![
                            Instruction::Assign(b, Box::new(Call(next_item, vec![Box::new(b)])))
                        ]
                    },
                    Instruction::If {
                        condition: Box::new(Comparison::LessThan(a, b)),
                        instructions: vec![
                            Instruction::Assign(a, Box::new(Call(next_item, vec![Box::new(a)])))
                        ]
                    }
                ]
            },
            Instruction::Return(Box::new(a))
        ]
    }
}

fn main() {
    let mut identifier_factory = IdentifierFactory::default();

    let sum_digits = sum_digits(&mut identifier_factory);
    let next_item = next_item(&mut identifier_factory, sum_digits.identifier);
    let meeting_point = meeting_point(&mut identifier_factory, next_item.identifier);

    let graph = Graph {
        functions: vec![
            sum_digits,
            next_item,
            meeting_point
        ],
        main: None
    };

    println!("{}", graph.convert());
}
