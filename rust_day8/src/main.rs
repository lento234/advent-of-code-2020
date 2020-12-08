use std::fs::read_to_string;

type Instruction<'a> = (&'a str, i32);
type Instructions<'a> = Vec<Instruction<'a>>;

fn main() {
    let raw = read_to_string("./src/input.txt").expect("cannot read file");
    let raw = raw.trim();
    let instructions = instructions_to_vec(raw);
    
    let (part_1_acc, _) = infinite_loop(&instructions, Vec::new(), 0, 0);
    println!("{}", part_1_acc);

    let part_2_acc = find_finite_loop(&instructions);
    println!("{}", part_2_acc);
}

fn instructions_to_vec(instructions: &str) -> Instructions {
    let mut ret = Vec::new();
    for instruction in instructions.lines() {
        let temp: Vec<&str> = instruction.split(" ").collect();
        let (command, info) = (temp[0], temp[1]);
        let (sign, digit_str) = info.split_at(1);
        
        let digit = match sign {
            "+" => digit_str.parse::<i32>().unwrap(),
            _ => -digit_str.parse::<i32>().unwrap()
        };
        
        ret.push(
            (command, digit),
        )
    }
    ret
}

fn infinite_loop(instructions: &Instructions, mut visited: Vec<i32>, mut pos: i32, mut acc: i32) -> (i32, bool) {
    let mut finite = true;

    loop {
        if pos as usize > instructions.len() - 1 {
            break;
        }

        if visited.contains(&pos) {
            finite = false;
            break;
        }

        if let Some(instruction) = &instructions.get(pos as usize) {
            visited.push(pos);
            update_instruction(instruction, &mut pos, &mut acc);
        }
    }
    
    (acc, finite)
}

fn update_instruction(instruction: &Instruction, curr_pos: &mut i32, acc: &mut i32) {
    match instruction.0 {
        "acc" => {
            *acc += instruction.1;
            *curr_pos += 1;
        }
        "jmp" => {
            *curr_pos += instruction.1;
        }
        _ => {
            *curr_pos += 1;
        }
    }
}

fn find_finite_loop(instructions: &Instructions) -> i32 {
    let mut pos = 0i32;
    let mut acc = 0i32;
    let mut visited: Vec<i32> = Vec::new();

    loop {
        visited.push(pos);
        let instruction = instructions[pos as usize];
        
        if should_try_swap_instruction(&instruction) {
            let (mut temp_pos, mut temp_acc) = (pos, acc);
            swap_and_update_instruction(&instruction, &mut temp_pos, &mut temp_acc);

            let (inner_acc, finite) = infinite_loop(&instructions, visited.clone(), temp_pos, temp_acc);

            match finite {
                true => {
                    acc = inner_acc;
                    break;
                },
                false => {
                    update_instruction(&instruction, &mut pos, &mut acc);
                },
            }
        } else {
            update_instruction(&instruction, &mut pos, &mut acc);
        }
    }
    
    acc
}

fn should_try_swap_instruction(instruction: &Instruction) -> bool {
    match instruction.0 {
        "jmp" | "nop" => true,
        _ => false,
    }
}

fn swap_and_update_instruction(instruction: &Instruction, curr_pos: &mut i32, acc: &mut i32) {
    match instruction.0 {
        "jmp" => update_instruction(&("nop", instruction.1), curr_pos, acc),
        "nop" => update_instruction(&("jmp", instruction.1), curr_pos, acc),
        _ => panic!(),
    }
}
