use teloxide::utils::command::BotCommands;

#[derive(BotCommands)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
    #[command(description = "Point story poll template. Usage: /pointstory task")]
    PointStory,
}
