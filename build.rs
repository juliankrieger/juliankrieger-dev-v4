use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;

    let mut statics = ructe.statics()?;

    statics.add_files("static")?;

    ructe.compile_templates("templates")
}
