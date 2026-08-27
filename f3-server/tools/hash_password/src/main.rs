use adapter_out_persistence::password_hasher_impl::Argon2PasswordHasher;
use application::domain::model::password_model::RawPassword;
use application::port::outbound::password_hasher::PasswordHasherTrait;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rpassword::prompt_password("Password: ")?;
    let confirm = rpassword::prompt_password("Confirm : ")?;
    if input != confirm {
        eprintln!("error: passwords do not match");
        std::process::exit(1);
    }

    let raw = RawPassword::new(input)?;
    let hasher = Argon2PasswordHasher::new();
    let hashed = hasher.hash(&raw)?;

    println!("{}", hashed.expose_for_persistence());
    Ok(())
}
