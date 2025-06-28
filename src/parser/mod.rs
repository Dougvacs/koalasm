pub fn parse_add(line: &str, add: &dyn Fn(usize, usize, usize)) {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() != 4 || parts[0] != "add" {
        panic!("Invalid add command.");
    }
    let rd = parts[1].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    let rs1 = parts[2].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    let rs2 = parts[3].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    add(rd, rs1, rs2);
}

pub fn parse_li(line: &str, li: &dyn Fn(usize, i64)) {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() != 3 || parts[0] != "li" {
        panic!("Invalid li command.");
    }
    let rd = parts[1].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    let imm = parts[2].parse::<i64>().unwrap();
    li(rd, imm);
}

pub fn parse_addi(line: &str, addi: &dyn Fn(usize, usize, i64)) {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() != 4 || parts[0] != "addi" {
        panic!("Invalid addi command.");
    }
    let rd = parts[1].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    let rs1 = parts[2].strip_prefix("x").unwrap().parse::<usize>().unwrap();
    let imm = parts[3].parse::<i64>().unwrap();
    addi(rd, rs1, imm);
}