use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), io::Error> {
    let f = File::open("input.txt")?;
    let file = BufReader::new(&f);

    let lines: Vec<String> = file
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    for (i, line) in lines.iter().enumerate() {
        for (j, line2) in lines.iter().enumerate() {
            if i != j {
                compare_str(&line, &line2);
            }
        }
    }
    Ok(())
}

fn compare_str(s1: &str, s2: &str) {
    let mut index: Option<usize> = None;
    let mut is_one_off = false;
    for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if c1 != c2 {
            if is_one_off {
                return;
            }
            is_one_off = true;
            index = Some(i);
        }
    }
    println!("s1: {}  s2: {}", s1, s2);
    println!("INDEX: {:?}", index);
}

// fn main() -> Result<(), io::Error> {
//     let f = File::open("input.txt")?;
//     let file = BufReader::new(&f);

//     let mut triple_count = 0;
//     let mut double_count = 0;

//     for line in file.lines() {
//         if let Ok(l) = line {
//             let (is_double, is_triple) = count_letters(&l);
//             triple_count = triple_count + (is_triple as i32);
//             double_count = double_count + (is_double as i32);
//         }
//     }
//     println!("RES: {}", triple_count * double_count);
//     Ok(())
// }

// fn count_letters(s: &str) -> (bool, bool) {
//     let mut letters = HashMap::new();
//     let mut triple_count = 0;
//     let mut double_count = 0;
//     let bytes = s.as_bytes();
//     for b in bytes {
//         match letters.get(&c) {
//             None => {
//                 letters.insert(c, 1);
//                 ()
//             }
//             Some(count) => {
//                 if *count == 1 {
//                     println!("DOUBLE: {}", c);
//                     double_count += 1;
//                 }
//                 if *count == 2 {
//                     println!("TRIPLE: {}", c);
//                     triple_count += 1;
//                 }
//                 letters.insert(c, count + 1);
//             }
//         }
//     }
//     return (double_count != triple_count, triple_count != 0);
// }
