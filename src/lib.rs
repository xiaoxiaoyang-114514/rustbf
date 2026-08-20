use std::io::Read;

pub fn run_bf(src: &str, input: Option<Vec<char>>) -> Result<Vec<char>, String> {
    let mut input_index = 0;
    let mut stdin_input = std::io::stdin().bytes();
    let src_bytes = src.as_bytes();
    let mut mem :Vec<u8> = [0].to_vec();
    let mut index = 0;
    let mut line: usize = 0;
    let mut c: u8;
    let mut output: Vec<char> = Vec::new();
    while line < src_bytes.len(){
        c = src_bytes[line];
        if c == b'>' {
            index += 1;
            if index >= mem.len() {
                mem.push(0);
            }
        } else if c == b'<' {
            if index == 0{
                return Err("the index is negative.".to_string());
            }
            index -= 1;
            if (mem[index+1] == 0) && (mem.len() == index + 2) {
                mem.pop();
            }
        } else if c == b'+' {
            mem[index] = mem[index].wrapping_add(1);
        } else if c == b'-' {
            mem[index] = mem[index].wrapping_sub(1);
        } else if c == b'.' {
            output.push(mem[index] as char);
        } else if c == b',' {
            mem[index] = match input {
                Some(ref list) => {
                    if input_index < list.len() {
                        list[input_index] as u8
                    } else {
                        0
                    }
                },
                None => match stdin_input.next() {
                    Some(Ok(c)) => c,
                    _ => 0,
                },
            };
            input_index += 1;
        } else if c == b'[' {
            if mem[index] == 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    line += 1;
                    if line >= src_bytes.len() {
                        return Err(format!("Unexpected token \"[\" at the character {}. ", tmpline+1).to_string());
                    }
                    if src_bytes[line] == 91 {
                        nesting += 1;
                    } else if src_bytes[line] == 93 {
                        nesting -= 1;
                    }
                }
            }
        } else if c == b']' {
            if mem[index] != 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    if line == 0 && nesting != 0{
                        return Err(format!("Unexpected token \"]\" at the character {}.", tmpline+1).to_string());
                    }
                    line -= 1;
                    if src_bytes[line] == 93 {
                        nesting += 1;
                    } else if src_bytes[line] == 91 {
                        nesting -= 1;
                    }
                }
            }
        }
        line += 1;
    }
    output.push('\n');
    Ok(output)
}



#[cfg(test)]
//some simple tests
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let out = run_bf(src, None).unwrap().into_iter().collect::<String>();
        assert_eq!(out, "Hello World!\n\n");
    }
    
    #[test]
    fn double() {
        let src = ">,>[-]>[-]<<[->+>+<<]>.>.";
        let input: Vec<char> = ['a'].to_vec();
        let out = run_bf(src, Some(input)).unwrap().into_iter().collect::<String>();
        assert_eq!(out, "aa\n");
    }
}
