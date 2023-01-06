use std::fs;

fn read_binary_file(filename: &str) -> std::io::Result<()>
{
    let mut code = Vec::new();
    code = fs::read(filename).unwrap();

    println!("{:?}", code);

    Ok(())
}

pub fn rop(filename: &str) -> std::io::Result<()> {
    read_binary_file(filename)?;

    Ok(())
}

