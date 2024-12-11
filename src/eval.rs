use crate::ast::AST;

use std::collections::HashMap;

pub fn eval(expr: AST, context: &mut HashMap<String, AST>) {
    match expr {
        AST::Call { name, args } => {
            match name.as_str() {
                "print" => {
                    match args[0].clone() {
                        AST::String(s) => {
                            println!("{}", s.replace("\"", ""));
                        }

                        AST::Number(n) => {
                            println!("{}", n);
                        }

                        AST::Identifer(name) => {
                            match context.get(&name) {
                                Some(value) => {
                                    match value {
                                        AST::String(s) => {
                                            println!("{}", s.replace("\"", ""));
                                        }

                                        AST::Number(n) => {
                                            println!("{}", n);
                                        }

                                        _ => {
                                            println!("Unknown value: {:?}", value);
                                        }
                                    }
                                }

                                None => {
                                    println!("Unknown variable: {}", name);
                                }
                            }
                        }

                        _ => {
                            println!("Unknown argument: {:?}", args[0]);
                        }
                    }
                }

                "exit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                }

                _ => {
                    println!("Unknown function: {}", name);
                }
            }
        }

        AST::LetDeclaration { name, value } => {
            if let Some(name) = name {
                context.insert(name, *value);
            }
        }

        _ => {
            println!("Unknown expression: {:?}", expr);
        }
    }
}