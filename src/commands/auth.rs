use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::id::ChannelId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::user::User;

const CHANNEL_ID: u64 = 1047686769799344189;
const ROLE_ID: u64 = 1047688153571541035;

pub async fn run(
    options: &[CommandDataOption],
    channel_id: &ChannelId,
    user: &User,
    ctx: &Context,
) -> String {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let password = match std::env::var("MOLE_PASSWORD") {
        Ok(pass) => pass,
        Err(_) => panic!("Can not find the password in environment"),
    };

    let option = options
        .get(0)
        .expect("Expected password.")
        .resolved
        .as_ref()
        .expect("Expected password");

    if let CommandDataOptionValue::String(attempt) = option {
        if attempt == &password && channel_id.0 == CHANNEL_ID {
            let add_role_result = ctx
                .http
                .add_member_role(
                    crate::GUILD_ID,
                    user.id.0,
                    ROLE_ID,
                    Some("Authenticated user"),
                )
                .await;
            return match add_role_result {
                Ok(_) => String::from("Successfully authenticated."),
                Err(e) => format!("Could not add role due to error: {}", e),
            };
        } else {
            String::from("Invalid password. Unable to authenticate.")
        }
    } else {
        String::from("Invalid command.")
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("auth")
        .description("Authenticate for 30 minutes")
        .create_option(|option| {
            option
                .name("password")
                .description("The password to authenticate with")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
