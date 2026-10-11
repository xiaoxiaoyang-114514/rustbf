use std::io::Read;

pub fn parentheses_match(code: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    let mut temp: Vec<usize> = Vec::new();
    let mut out_left: Vec<usize> = Vec::new();
    let mut out_right: Vec<usize> = Vec::new();
    out_left.resize(code.len(), 0);
    out_right.resize(code.len(), 0);
    for i in 0..code.len() {
        match code[i] {
            b'[' => {
                temp.push(i);
            },
            b']' => {
                left.push(temp.pop().unwrap_or_else(|| {
                    eprintln!("Parentheses could not match in {}.",i);
                    std::process::exit(1);
                }));
                right.push(i)
            },
            _ => {},
        }
    }
    for i in 0..left.len() {
        out_left[left[i]] = right[i];
        out_right[right[i]] = left[i];
    }
    (out_left, out_right)
}

pub fn run_bf(src: &str, input: Option<Vec<char>>, stream_opt: bool) -> Result<Option<String>, String> {
    let mut input_index = 0;
    let mut stdin_input = std::io::stdin().bytes();
    let src_bytes = src.as_bytes();
    let mut mem :Vec<u8> = vec![0];
    let mut index = 0;
    let mut line: usize = 0;
    let mut c: u8;
    let mut output: String = String::new();
    let (left, right) = parentheses_match(&src_bytes);
    
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
            if stream_opt{
                print!("{}", mem[index] as char);
            } else{
                output.push(mem[index] as char);
            }
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
                line = left[line];
            }
        } else if c == b']' {
            if mem[index] != 0 {
                line = right[line];
            }
        }
        line += 1;
    }
    output.push('\n');
    if stream_opt{
        Ok(None)
    } else {
        Ok(Some(output))
    }
}



#[cfg(test)]
//some simple tests
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let out = run_bf(src, None, false).unwrap().unwrap().into_iter().collect::<String>();
        assert_eq!(out, "Hello World!\n\n");
    }
    
    #[test]
    fn double() {
        let src = ">,>[-]>[-]<<[->+>+<<]>.>.";
        let input: Vec<char> = ['a'].to_vec();
        let out = run_bf(src, Some(input), false).unwrap().unwrap().into_iter().collect::<String>();
        assert_eq!(out, "aa\n");
    }
}
