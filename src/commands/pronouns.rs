use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::json::JsonMap;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::Role;
use serenity::model::prelude::User;

pub async fn run(options: &[CommandDataOption], user: &User, ctx: &Context) -> String {
    let option = options
        .get(0)
        .expect("Expected pronouns.")
        .resolved
        .as_ref()
        .expect("Expected pronouns.");

    if let CommandDataOptionValue::String(p) = option {
        let pronouns = p.to_ascii_lowercase();
        let roles = ctx
            .http
            .get_guild_roles(crate::GUILD_ID)
            .await
            .expect("Could not fetch server roles.");
        let mut existing_role: Vec<Role> = roles
            .iter()
            .filter(|role| role.name == pronouns)
            .map(|r| r.to_owned())
            .collect();

        let new_role: Role = if existing_role.is_empty() {
            let mut role_map: JsonMap = JsonMap::new();
            role_map.insert("name".to_string(), pronouns.to_ascii_lowercase().into());
            ctx.http
                .create_role(crate::GUILD_ID, &role_map, None)
                .await
                .expect("Failed to add role")
        } else {
            existing_role.pop().expect("Empty")
        };
        ctx.http
            .add_member_role(crate::GUILD_ID, user.id.0, new_role.id.0, None)
            .await
            .expect("Could not update user pronouns.");
        String::from("Successfully updated pronouns.")
    } else {
        String::from("Invalid string")
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("pronouns")
        .description("Give custom pronouns not provided in bot")
        .create_option(|option| {
            option
                .name("pronouns")
                .description("Please input your preferred pronouns")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
