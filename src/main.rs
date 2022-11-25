use rustop::opts;
use std::fs;

struct VM {
    memory: [i16; 30000],
    data_ptr: usize,
    instr_ptr: usize
}

impl Default for VM {
    fn default() -> VM {
        VM {
            memory: [0; 30000],
            data_ptr: 0,
            instr_ptr: 0
        }
    }
}

fn loop_forwards(vm: &mut VM, instr: &str) {
    if !matches!(vm.memory[vm.data_ptr], 0) {
        return;
    }

    let mut match_level = 1;
    vm.instr_ptr += 1;
    while match_level > 0 {
        if matches!(instr.chars().nth(vm.instr_ptr).unwrap(), '[') {
            match_level += 1;
        }
        else if matches!(instr.chars().nth(vm.instr_ptr).unwrap(), ']') {
            match_level -= 1;
        }
        
        if match_level > 0 {
            vm.instr_ptr += 1;
        }
    }
}

fn loop_backwards(vm: &mut VM, instr: &str) {
    if matches!(vm.memory[vm.data_ptr], 0) {
        return;
    }

    let mut match_level = 1;
    vm.instr_ptr -= 1;
    while match_level > 0 {
        if matches!(instr.chars().nth(vm.instr_ptr).unwrap(), '[') {
            match_level -= 1;
        }
        else if matches!(instr.chars().nth(vm.instr_ptr).unwrap(), ']') {
            match_level += 1;
        }
        
        if match_level > 0 {
            vm.instr_ptr -= 1;
        }
    }
}

fn print_func(c: char) {
    print!("{}", c);
}

fn run(instr: &str, mut out_func: impl FnMut(char)) {
    let mut vm: VM = Default::default();

    while vm.instr_ptr < instr.len() {
        match instr.chars().nth(vm.instr_ptr).unwrap() {
            '>' => vm.data_ptr += 1,
            '<' => vm.data_ptr -= 1,
            '+' => vm.memory[vm.data_ptr] += 1,
            '-' => vm.memory[vm.data_ptr] -= 1,
            '.' => out_func(vm.memory[vm.data_ptr] as u8 as char),
            '[' => loop_forwards(&mut vm, instr),
            ']' => loop_backwards(&mut vm, instr),
            _ => ()
        }

        vm.instr_ptr += 1;
    }

    println!("");
}

fn main() {
    let (args, _) = opts! {
        synopsis "MC Dreamer's BF Interpreter";
        version "v0.1";
        opt code:Option<String>, desc:"The code to run";
        opt file:Option<String>, desc:"The file containing the code to run";
    }.parse_or_exit();

    if let Some(code) = args.code {
        run(&code, print_func);
    } else if let Some(file) = args.file {
        let code = fs::read_to_string(file).expect("Error reading file...");
        run(&code, print_func);
    } else {
        println!("Nothing to do...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use string_builder::Builder;

    #[test]
    fn test_numbers() {
        let mut builder = Builder::default();
        let out_func = |c: char| builder.append(c);
        run("++++++++++++++++++++++++++++++++++++++++++++++++ >++++++++++ [< .+ >-]", out_func);
        assert_eq!(builder.string().unwrap(), "0123456789");
    }

    #[test]
    fn test_hello_world() {
        let mut builder = Builder::default();
        let out_func = |c: char| builder.append(c);
        run("++++++++++[>+++++++>++++++++++>+++>++++<<<<-]>++.>+.+++++++..+++.>>++++.<++.<++++++++.--------.+++.------.--------.>+.", out_func);
        assert_eq!(builder.string().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_hello_world_nested_loops() {
        let mut builder = Builder::default();
        let out_func = |c: char| builder.append(c);
        run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", out_func);
        assert_eq!(builder.string().unwrap(), "Hello World!\n");
    }
}