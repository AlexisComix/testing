use std::{collections::HashMap, env, fs};

// Static array used for checking if a token is a keyword
static KEYWORDS: &'static [&'static str] = &[
    "iadd", "out", "imul", "isub", "idiv", "set", "equ", "//", "free", "le", "gr", "leq", "geq",
    "jnz", "jmp",
];
static INT_OPERATIONS: &'static [&'static str] = &[
    "iadd", "imul", "isub", "idiv", "equ", "le", "gr", "leq", "geq",
];

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect args passed on run
    if args.len() != 2 {
        panic!("Invalid arguments")
    } // Panic on invalid no. args
    let filepath: &String = &args[1]; // Get filepath

    let mut memory: HashMap<String, i32> = HashMap::new();
    let lines: Vec<String> = read_lines(&filepath);
    let tokenised_lines: Vec<Vec<String>> = tokenise_lines(&lines);

    // dbg!(
    //     &args,
    //     &filepath,
    //     &lines,
    //     &tokenised_lines,
    //     &memory,
    // );

    println!("\n--- Output ---");
    run_lines(&tokenised_lines, &mut memory);
    println!("--------------\n");

    // dbg!(&memory);
}

fn run_line(line: &Vec<String>, mem: &mut HashMap<String, i32>) -> () {
    /*
        Runs a line
    */
    let first: &str = &line[0][..];

    if !KEYWORDS.contains(&first) {
        panic!("Invalid keyword {}", first)
    }

    if INT_OPERATIONS.contains(&first) {
        let a = line[1]
            .parse::<i32>()
            .unwrap_or_else(|_| get_val(&line[1], mem));
        let b = line[2]
            .parse::<i32>()
            .unwrap_or_else(|_| get_val(&line[2], mem));

        let val = match first {
            "iadd" => iadd(&a, &b),
            "imul" => imul(&a, &b),
            "isub" => isub(&a, &b),
            "idiv" => idiv(&a, &b),
            "equ" => equ(&a, &b),
            "le" => le(&a, &b),
            "gr" => gr(&a, &b),
            "leq" => leq(&a, &b),
            "geq" => geq(&a, &b),
            &_ => panic!("Invalid operation {}", first),
        };
        istore(&line[3], &val, Box::new(mem).as_mut());
    } else {
        match first {
            "out" => out(&line[1].to_owned(), mem),
            "set" => istore(
                &line[2],
                &line[1]
                    .parse::<i32>()
                    .unwrap_or_else(|_| get_val(&line[1], mem)),
                Box::new(mem).as_mut(),
            ),
            "free" => free(&line[1].to_owned(), mem),
            "//" => (),  // Comments
            "jnz" => (), // jnz handled by run_lines
            "jmp" => (), // jmp handled by run_lines
            &_ => panic!("Invalid operation {}", first),
        }
    }
}

fn get_val(var: &String, mem: &mut HashMap<String, i32>) -> i32 {
    match mem.get(var) {
        Some(i) => return *i,
        None => panic!("No such variable name {}", var),
    };
}

fn free(var: &String, mem: &mut HashMap<String, i32>) -> () {
    /*
        Frees a space in memory with name var
    */
    mem.remove(var);
}

// fn dump(mem: &mut HashMap<String, i32>) -> () {
//     /*
//         DBG output whats in memory
//     */
//     dbg!(mem);
// }

fn le(a: &i32, b: &i32) -> i32 {
    /*
        a less than b
    */
    (a < b) as i32
}

fn gr(a: &i32, b: &i32) -> i32 {
    /*
        a greater than b
    */
    (a > b) as i32
}

fn leq(a: &i32, b: &i32) -> i32 {
    /*
        a less than or equal to b
    */
    (a <= b) as i32
}

fn geq(a: &i32, b: &i32) -> i32 {
    /*
        a greater than or equal to b
    */
    (a >= b) as i32
}

fn run_lines(lines: &Vec<Vec<String>>, mem: &mut HashMap<String, i32>) -> () {
    /*
        Runs the lines
    */
    let num_lines: usize = lines.len();
    let mut i: usize = 0;
    while i < num_lines {
        run_line(&lines[i], mem);
        let first: &str = &lines[i][0][..];

        match first {
            "//" => i += 1,
            "jnz" => {
                let a = lines[i][1]
                    .parse::<i32>()
                    .unwrap_or_else(|_| get_val(&lines[i][1], mem));
                match a {
                    0 => (),
                    1 => {
                        let b = lines[i][2]
                            .parse::<i32>()
                            .unwrap_or_else(|_| get_val(&lines[i][2], mem));
                        i += b as usize;
                    }
                    _ => panic!("Non boolean value {}", a),
                }
            }
            "jmp" => {
                let a = lines[i][1]
                    .parse::<i32>()
                    .unwrap_or_else(|_| get_val(&lines[i][1], mem));
                i += a as usize;
            }
            _ => {
                i += 1;
            }
        };
    }
}

fn out(var: &String, mem: &mut HashMap<String, i32>) -> () {
    /*
        Prints a character referenced to by a variable name var in memory mem
    */
    match mem.get(var) {
        Some(x) => println!("{:?}", x),
        None => panic!("No variable named {}", var),
    };
}

fn istore(var: &String, val: &i32, mem: &mut HashMap<String, i32>) -> () {
    /*
        Inserts something into mem with key var and value val
    */
    mem.insert(var.to_string(), *val);
}

fn read_lines(filepath: &String) -> Vec<String> {
    /*
        Reads all of the lines in the file specified by filepath.
        Returns a Vec<String> of the lines split by ";".
        Removes \r\n and leading/trailing whitespace.
    */
    fs::read_to_string(&filepath)
        .expect("Failed to read file.")
        .split(";")
        .map(|x| x.to_owned().replace("\r\n", "").trim().to_owned())
        .filter(|x| x != "")
        .collect::<Vec<String>>()
}

fn tokenise_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    /*
        Takes a Vec<String> of lines and splits each line into
        a Vec<String> by single spaces. Returns a Vec<Vec<String>>.
    */
    lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn iadd(a: &i32, b: &i32) -> i32 {
    /*
    Adding function for two i32.
    */
    a + b
}

fn imul(a: &i32, b: &i32) -> i32 {
    /*
        Multiply function for two i32
    */
    a * b
}

fn isub(a: &i32, b: &i32) -> i32 {
    /*
        Subtract function for i32
    */
    a - b
}

fn idiv(a: &i32, b: &i32) -> i32 {
    /*
        Divide two i32s
    */
    if b == &0 {
        panic!("Zero division error")
    }
    a / b
}

fn equ(a: &i32, b: &i32) -> i32 {
    /*
        1 if a == b else 0
    */
    (a == b) as i32
}
