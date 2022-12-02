use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

mod commands;

// The Guild Id of the Co '26 server
pub const GUILD_ID: u64 = 1047659404696244286;

// Initialize an empty struct
struct Handler;

// Add implementation for handling discord events
#[async_trait]
impl EventHandler for Handler {
    // Initialize the bot on the server
    async fn ready(&self, ctx: Context, _r: Ready) {
        // Add the commands to the guild
        let commands =
            GuildId::set_application_commands(&GuildId(GUILD_ID), &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::auth::register(command))
                    .create_application_command(|command| commands::deauth::register(command))
                    .create_application_command(|command| commands::pronouns::register(command))
            })
            .await;

        // Handle errors if commands were not able to be added
        match commands {
            Ok(_) => println!("Successfully added slash commands to guild."),
            Err(e) => println!("Error adding slash commands to guild: {}", e),
        };
    }

    // Run on slash command usage
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Match the slash command to corresponding function
            let content = match command.data.name.as_str() {
                "auth" => {
                    commands::auth::run(
                        &command.data.options,
                        &command.channel_id,
                        &command.user,
                        &ctx,
                    )
                    .await
                }
                "deauth" => commands::deauth::run(&command.user, &ctx).await,
                "pronouns" => {
                    commands::pronouns::run(&command.data.options, &command.user, &ctx).await
                }
                _ => "not implemented :(".to_string(),
            };

            // Handle errors on slash commands
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load .env file if in development environment
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
