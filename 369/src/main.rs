fn main() {
    println!("#{}", hex_color(0, 0, 0));
    println!("#{}", hex_color(255, 99, 71));
}

fn hex_color(r: u8, g: u8, b: u8) -> String {
    vec![r, g, b].iter().map(|v| dec_to_hex(v.clone(), 2)).collect::<Vec<String>>().join("")
}
fn hex_to_dec(hex: String) -> u32 {
    // Simple lookup of dec -> hex
    let hex_codes:[char; 16]  =
    ['0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F'];

    let currentValue = 0;

    for c in hex.chars() {
        //
    }

    // Push hex codes onto the stack (most significant digit first)
    // let mut character_stack = Vec::new();

    let mut current_value = dec;
    while current_value > 16 {    
        let remainder = current_value % 16;
        character_stack.push(hex_codes[remainder as usize]);
        current_value = current_value / 16;
    }
    // Handle final unit column
    character_stack.push(hex_codes[(current_value) as usize]);

    while character_stack.len() < pad_size as usize {
        character_stack.push('0')
    }

    let mut hex = String::new();
    while let Some(c) = character_stack.pop() {
        hex.push(c);
    }
    hex
}

fn dec_to_hex(dec: u8, pad_size: u8) -> String {
    // Simple lookup of dec -> hex
    let hex_codes:[char; 16]  =
    ['0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F'];

    // Push hex codes onto the stack (most significant digit first)
    // let mut character_stack = Vec::new();

    let mut current_value = dec;
    while current_value > 16 {    
        let remainder = current_value % 16;
        character_stack.push(hex_codes[remainder as usize]);
        current_value = current_value / 16;
    }
    // Handle final unit column
    character_stack.push(hex_codes[(current_value) as usize]);

    while character_stack.len() < pad_size as usize {
        character_stack.push('0')
    }

    let mut hex = String::new();
    while let Some(c) = character_stack.pop() {
        hex.push(c);
    }
    hex
}