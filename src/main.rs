use refer_cli::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}.", e),
    }

    Ok(())
}
