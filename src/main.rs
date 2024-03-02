use std::{
    env, fs,
    io::{Read, Write},
};

mod ast;
mod ast_extract;
mod cfg;
mod cfg_builder;
mod cfg_extract;
mod cfg_update;
mod cfg_writer;
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

    let mut static_space = String::from("data:\n");
    let mut code_space = String::from("code:\n");
    let mut writer = cfg_writer::Writer::from(&mut static_space, &mut code_space);

    for function in functions {
        let mut cfg = cfg::CFG::new();
        cfg_builder::Builder::from(&classes, &function).update(&mut cfg)?;

        cfg_update::SSA::new().update(&mut cfg)?;

        let this = ast::Local::from(ast::Name::from(String::from("this")));
        cfg_update::Peephole::from(function.get_local_id(&this)).update(&mut cfg)?;

        cfg_update::VN::new().update(&mut cfg)?;
        cfg_writer::Write::write(&cfg, &mut writer, &classes, &function);
    }

    out_stream.write(static_space.as_bytes())?;
    out_stream.write(code_space.as_bytes())?;
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
