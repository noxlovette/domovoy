use keyring::Entry;
use owo_colors::OwoColorize;

const CLIENT_ID: &'static str = env!("YANDEX_CLIENT_ID");
// const CLIENT_SECRET: &'static str = env!("YANDEX_CLIENT_SECRET");
pub const SERVICE: &str = "domovoy";
pub const NAME: &str = "oauth_token";

fn oauth_url() -> String {
    format!("https://oauth.yandex.ru/authorize?response_type=token&client_id={CLIENT_ID}")
}

pub fn init() -> anyhow::Result<()> {
    println!("{}", "Welcome!".cyan().bold());
    println!("We will now authenticate you on domovoy.\n");

    print!("{}", "Now please go to the following url: ".yellow());
    println!("{}\n", oauth_url().dimmed());

    let token = rpassword::prompt_password(
        "-> Paste the token you received (you will not see it on paste): ".green(),
    )?;
    let entry = Entry::new(SERVICE, NAME)?;

    entry.set_password(&token)?;

    println!(
        "{}",
        "Great, you are authenticated. Now run domovoy tui to start using the service".green()
    );
    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    println!("{}", "Deleting credentials...".red());
    let entry = Entry::new(SERVICE, NAME)?;
    entry.delete_credential()?;
    println!("{}", "You no longer exist.".red());

    Ok(())
}
