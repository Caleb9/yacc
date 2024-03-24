use std::io;
use yapp::PasswordReader;

fn main() -> io::Result<()> {
    let mut yapp = yapp::new();
    yapp.set_echo_symbol('*');
    let password = yapp.read_password()?;
    println!("You typed: {password}");
    Ok(())
}
