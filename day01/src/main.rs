use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    let f = File::open("input.txt")?;
    let mut out = File::create("out.rb")?;
    out.write(b"require 'set'\n")?;
    out.write(b"x = 0\n")?;
    out.write(b"values = Set.new\n")?;
    out.write(b"loop do\n")?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        if let Ok(l) = line {
            out.write(b"if values.include?(x) then puts(x); break else values.add(x) end\n")?;
            let line = format!("x = x {}\n", l);
            out.write(line.as_bytes())?;
        }
    }
    out.write(b"end")?;
    Ok(())
}
