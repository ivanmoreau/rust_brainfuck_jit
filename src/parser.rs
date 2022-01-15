use nom::{IResult, multi::{many1_count, many0}, bytes::complete::tag, branch::alt};

#[derive(Debug)]
pub enum Instruction {
    Inc(u64),
    Dec(u64),
    Add(u64),
    Min(u64),
    Out,
    Int,
    While(Vec<Instruction>)
}

pub fn parse(program: String) -> Vec<Instruction> {
    let program: String = program.chars().filter(
        |x| *x == '>' || *x == '<' || *x == '+' || *x == '-' || 
        *x == '.' || *x == ',' || *x == '[' || *x == ']' 
    ).collect();
    match parse_instructions(&program) {
        Ok(x) => x.1,
        Err(_) => vec![],
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_inc, parse_dec, parse_add, parse_min, parse_out, parse_int,
         parse_while))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

fn parse_while(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("[")(input)?;
    let (input, n) = parse_instructions(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Instruction::While(n)))
}


fn parse_inc(input: &str) -> IResult<&str, Instruction> {
    let (input, n)= many1_count(tag(">"))(input)?;
    Ok((input, Instruction::Inc(n as u64)))
}

fn parse_dec(input: &str) -> IResult<&str, Instruction> {
    let (input, n)= many1_count(tag("<"))(input)?;
    Ok((input, Instruction::Dec(n as u64)))
}

fn parse_add(input: &str) -> IResult<&str, Instruction> {
    let (input, n)= many1_count(tag("+"))(input)?;
    Ok((input, Instruction::Add(n as u64)))
}

fn parse_min(input: &str) -> IResult<&str, Instruction> {
    let (input, n)= many1_count(tag("-"))(input)?;
    Ok((input, Instruction::Min(n as u64)))
}

fn parse_out(input: &str) -> IResult<&str, Instruction> {
    let (input, _)= tag(".")(input)?;
    Ok((input, Instruction::Out))
}

fn parse_int(input: &str) -> IResult<&str, Instruction> {
    let (input, _)= tag(",")(input)?;
    Ok((input, Instruction::Int))
}