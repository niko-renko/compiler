use std::{
    env, fs,
    io::{Read, Write},
};

mod ast;
mod ast_extract;
mod cfg;
mod cfg_extract;
mod cfg_update;
mod traits;

use cfg_update::Update;
use traits::Extract;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut in_stream = in_stream()?;
    let mut out_stream = out_stream()?;

    let mut program_string = String::new();
    in_stream.read_to_string(&mut program_string)?;

    let ast = ast::AST::try_from(program_string)?;
    let classes = ast_extract::Classes::extract(&ast)?;
    let functions = ast_extract::Functions::extract(&ast)?;

    out_stream.write(b"data:\n")?;
    classes.write(&mut out_stream)?;

    out_stream.write(b"code:\n")?;
    for function in functions {
        let mut cfg = cfg::CFG::new();
        cfg_update::Build::from(&classes, &function).update(&mut cfg)?;
        cfg_update::SSA::new().update(&mut cfg)?;
        cfg_update::Peephole::new(function.get_this_id()).update(&mut cfg)?;
        cfg_update::VN::new().update(&mut cfg)?;
        cfg::Write::write(&cfg, &mut out_stream, &classes, &function)?;
    }

    Ok(())
}

fn in_stream() -> Result<impl std::io::Read, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file = args.into_iter().skip(1).find(|x| !x.starts_with("-"));
    if file.is_none() {
        return Err(Box::from(String::from("No input file")));
    }
    let file = fs::File::open(file.unwrap())?;
    Ok(file)
}

fn out_stream() -> Result<impl std::io::Write, Box<dyn std::error::Error>> {
    Ok(std::io::stdout())
}
