use crate::utils::conversion;
use crate::intcode::IntcodeValue;

#[derive(Clone, PartialEq)]
pub enum IntcodeInstruction {
    Add{x: IntcodeValue, y: IntcodeValue, position: usize},
    Multiply{x: IntcodeValue, y: IntcodeValue, position: usize},
    Input{position: usize},
    Output{value: IntcodeValue},
    JumpIfTrue{test_position: IntcodeValue, jump_position: IntcodeValue},
    JumpIfFalse{test_position: IntcodeValue, jump_position: IntcodeValue},
    IsLessThan{x: IntcodeValue, y: IntcodeValue, position: usize},
    IsEquals{x: IntcodeValue, y: IntcodeValue, position: usize},
    SetRelativeBase{offset: IntcodeValue},
    Halt,
}

impl IntcodeInstruction {
    pub fn new(opcode_and_param_modes: i64, params: &[i64]) -> Self {
        use IntcodeInstruction::*;

        let digits: Vec<usize> = conversion::i64_into_digits(&opcode_and_param_modes)
            .into_iter()
            .rev()
            .collect();
            
        let opcode = digits[0] + 10 * digits.get(1).unwrap_or(&0);
        let get_value = |param_position| {
            let mode = *digits.get(param_position + 2).unwrap_or(&0);
            match mode {
                0 => IntcodeValue::Position(params[param_position] as usize),
                1 => IntcodeValue::Immediate(params[param_position]),
                2 => IntcodeValue::Relative(params[param_position]),
                _ => panic!("Invalid parameter mode: {}", mode),
            }
        };

        match opcode {
            1 => {
                Add {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            2 =>  {
                Multiply{ 
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }                
            },
            3 =>  {
                Input{ position: params[0] as usize }
            },
            4 =>  {
                Output{ 
                    value: get_value(0)
                }
            },
            5 => {
                JumpIfTrue { 
                    test_position: get_value(0),
                    jump_position: get_value(1),
                }
            },
            6 => {
                JumpIfFalse { 
                    test_position: get_value(0),
                    jump_position: get_value(1),
                }
            },
            7 => {
                IsLessThan {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            8 => {
                IsEquals {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            9 => SetRelativeBase { offset: get_value(0) },
            99 => Halt,
            _ => panic!("Invalid instruction: {:?}", opcode),
        }
    }
}

impl std::fmt::Debug for IntcodeInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IntcodeInstruction::*;
        
        let text = match self {
            Add{x, y, position} => format!("Add {:?} {:?} {:?}", x, y, position),
            Multiply{x, y, position} => format!("Mul {:?} {:?} {:?}", x, y, position),
            Input{position} => format!("Inp {:?}", position),
            Output{value} => format!("Out {:?}", value),
            JumpIfTrue{test_position, jump_position} => format!("JmT {:?} {:?}", test_position, jump_position),
            JumpIfFalse{test_position, jump_position} => format!("JmF {:?} {:?}", test_position, jump_position),
            IsLessThan{x, y, position} => format!("Lst {:?} {:?} {:?}", x, y, position),
            IsEquals{x, y, position} => format!("Eqt {:?} {:?} {:?}", x, y, position),
            SetRelativeBase{offset} => format!("Srb {:?}", offset),
            Halt => format!("Halt"),
        };
        
        write!(f, "{}", text)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use IntcodeInstruction::*;
    use IntcodeValue::*;

    #[test]
    fn test_param_modes() {
        assert_eq!(
            IntcodeInstruction::new(1, &[1, 2, 3]), 
            Add{x: Position(1), y: Position(2), position: 3});

        assert_eq!(
            IntcodeInstruction::new(101, &[4, 5, 6]), 
            Add{x: Immediate(4), y: Position(5), position: 6});

        assert_eq!(
            IntcodeInstruction::new(1001, &[4, 5, 6]), 
            Add{x: Position(4), y: Immediate(5), position: 6});

        assert_eq!(
            IntcodeInstruction::new(1101, &[4, 5, 6]), 
            Add{x: Immediate(4), y: Immediate(5), position: 6});
    }

    #[test]
    fn test_add() {
        assert_eq!(
            IntcodeInstruction::new(1, &[0, 1, 2]), 
            Add{x: Position(0), y: Position(1), position: 2}
        );
            
        assert_eq!(
            IntcodeInstruction::new(101, &[0, 1, 2]), 
            Add{x: Immediate(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(1001, &[0, 1, 2]), 
            Add{x: Position(0), y: Immediate(1), position: 2}
        );
    }

    #[test]
    fn test_multiply() {
        assert_eq!(
            IntcodeInstruction::new(2, &[0, 1, 2]), 
            Multiply{x: Position(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(102, &[0, 1, 2]), 
            Multiply{x: Immediate(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(1002, &[0, 1, 2]), 
            Multiply{x: Position(0), y: Immediate(1), position: 2}
        );
    }

    #[test]
    fn test_input() {
        assert_eq!(
            IntcodeInstruction::new(3, &[0]), 
            Input{position: 0}
        );
    }
    
    #[test]
    fn test_output() {
        assert_eq!(
            IntcodeInstruction::new(4, &[1]), 
            Output{value: Position(1)}
        );

        assert_eq!(
            IntcodeInstruction::new(104, &[1]), 
            Output{value: Immediate(1)}
        );
    }
    
    #[test]
    fn test_jump_if_true() {
        assert_eq!(
            IntcodeInstruction::new(5, &[0, 1, 2]), 
            JumpIfTrue{test_position: Position(0), jump_position: Position(1)}
        );
    
        assert_eq!(
            IntcodeInstruction::new(105, &[0, 1, 2]), 
            JumpIfTrue{test_position: Immediate(0), jump_position: Position(1)}
        );

        assert_eq!(
            IntcodeInstruction::new(1005, &[0, 1, 2]), 
            JumpIfTrue{test_position: Position(0), jump_position: Immediate(1)}
        );
    }
    
    #[test]
    fn test_jump_if_false() {
        assert_eq!(
            IntcodeInstruction::new(6, &[0, 1, 2]), 
            JumpIfFalse{test_position: Position(0), jump_position: Position(1)}
        );

        assert_eq!(
            IntcodeInstruction::new(106, &[0, 1, 2]), 
            JumpIfFalse{test_position: Immediate(0), jump_position: Position(1)}
        );

        assert_eq!(
            IntcodeInstruction::new(1006, &[0, 1, 2]), 
            JumpIfFalse{test_position: Position(0), jump_position: Immediate(1)}
        );
    }
    
    #[test]
    fn test_less_than() {
        assert_eq!(
            IntcodeInstruction::new(7, &[0, 1, 2]), 
            IsLessThan{x: Position(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(107, &[0, 1, 2]), 
            IsLessThan{x: Immediate(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(1007, &[0, 1, 2]), 
            IsLessThan{x: Position(0), y: Immediate(1), position: 2}
        );
    }
    
    #[test]
    fn test_equals() {
        assert_eq!(
            IntcodeInstruction::new(8, &[0, 1, 2]), 
            IsEquals{x: Position(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(108, &[0, 1, 2]), 
            IsEquals{x: Immediate(0), y: Position(1), position: 2}
        );

        assert_eq!(
            IntcodeInstruction::new(1008, &[0, 1, 2]), 
            IsEquals{x: Position(0), y: Immediate(1), position: 2}
        );
    }

    #[test]
    fn test_set_relative() {
        assert_eq!(
            IntcodeInstruction::new(9, &[0]), 
            SetRelativeBase{offset: Position(0)}
        );

        assert_eq!(
            IntcodeInstruction::new(109, &[0]), 
            SetRelativeBase{offset: Immediate(0)}
        );

        assert_eq!(
            IntcodeInstruction::new(209, &[0]), 
            SetRelativeBase{offset: Relative(0)}
        );
    }
    
    #[test]
    fn test_halt() {
        assert_eq!(IntcodeInstruction::new(99, &[]), Halt); 
    }
}