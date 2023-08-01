use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CommandArgs {
    /// Instance where the bot has been created
    pub instance: String,

    /// Community to post to 
    pub community: String
}