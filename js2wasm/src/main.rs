use color_eyre::Result;
use js2wasm;

fn main() -> Result<()> {
    color_eyre::install()?;

    // let src = std::fs::read_to_string("in.js")?;
    let src = r#"
    let x, y, z;
    "#;
    println!("{}", js2wasm::compile(src)?);

    // std::fs::write("out.wat", res)?;

    Ok(())
}
