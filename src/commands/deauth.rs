use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::user::User;

const ROLE_ID: u64 = 1047688153571541035;

pub async fn run(user: &User, ctx: &Context) -> String {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    return match ctx
        .http
        .remove_member_role(
            crate::GUILD_ID,
            user.id.0,
            ROLE_ID,
            Some("User manually logged out"),
        )
        .await
    {
        Ok(_) => String::from("Success"),
        Err(e) => format!("Unable to remove role. Error: {}", e),
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("deauth")
        .description("Deauthenticate (hide super secret channels)")
}
