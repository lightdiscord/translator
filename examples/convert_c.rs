use translator::*;

fn main() {
    let main_identifier = Identifier::new(0);

    let graph = Graph {
        functions: vec![
            Function {
                identifier: main_identifier,
                parameters: vec![],
                returns: Type::Int32,
                instructions: vec![
                    Instruction::Return(Box::new(0))
                ]
            }
        ],
        main: Some(main_identifier)
    };

    println!("{}", graph.convert());
}
