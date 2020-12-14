pub struct BootCodeComputer {
    accumulator: i64,
    code: Vec<(String, i64)>,
    instruction_index: usize,
}

impl BootCodeComputer {
    pub fn new(code: &Vec<String>) -> BootCodeComputer {
        let mut parsed_code = Vec::new();
        for i in code {
            parsed_code.push(BootCodeComputer::parse_instruction(i));
        }
        return BootCodeComputer {
            accumulator: 0,
            code: parsed_code,
            instruction_index: 0,
        };
    }

    pub fn get_accumulator_value(&self) -> i64 {
        return self.accumulator;
    }

    pub fn run_until_complete_with_instruction_swaps(&mut self) {
        let swappable_indices: Vec<usize> = self.code.iter().enumerate()
            .filter(|e| e.1.0 == "nop" || e.1.0 == "jmp")
            .map(|e| e.0)
            .collect();

        for swap_index in &swappable_indices {
            self.accumulator = 0;
            self.instruction_index = 0;
            self.swap_instruction(*swap_index);

            self.run_until_first_loop();
            if self.instruction_index == self.code.len() {
                return;
            }

            self.swap_instruction(*swap_index);
        }
    }

    fn swap_instruction(&mut self, index: usize) {
        match &self.code[index].0 as &str {
            "nop" => self.code[index].0 = "jmp".to_string(),
            "jmp" => self.code[index].0 = "nop".to_string(),
            _ => {},
        }
    }

    pub fn run_until_first_loop(&mut self) {
        let mut executed_indices = std::collections::HashSet::<usize>::new();
        while self.instruction_index < self.code.len() && !executed_indices.contains(&self.instruction_index) {
            executed_indices.insert(self.instruction_index);
            self.execute_current_instruction();
        }
    }

    fn execute_current_instruction(&mut self) {
        match &self.code[self.instruction_index].0 as &str {
            "nop" => self.instruction_index += 1,
            "acc" => {
                self.accumulator += (self.code[self.instruction_index]).1;
                self.instruction_index += 1;
            },
            "jmp" => {
                let new_index = self.instruction_index as i64 + (self.code[self.instruction_index]).1;
                self.instruction_index = new_index as usize;
            },
            _ => panic!("Bad boot code instruction"),
        }
    }

    fn parse_instruction(instruction: &String) -> (String, i64) {
        let parts: Vec<&str> = instruction.splitn(2, " ").collect();
        let value = parts[1].parse::<i64>().unwrap();

        return ((*parts[0]).to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn accumulator_value_at_first_loop() {
        let input = vec!(
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        );

        let mut computer = super::BootCodeComputer::new(&input);
        computer.run_until_first_loop();
        let actual = computer.get_accumulator_value();

        assert_eq!(5, actual);
    }

    #[test]
    fn accumulator_value_after_swap_completion() {
        let input = vec!(
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        );

        let mut computer = super::BootCodeComputer::new(&input);
        computer.run_until_complete_with_instruction_swaps();
        let actual = computer.get_accumulator_value();

        assert_eq!(8, actual);
    }
}
