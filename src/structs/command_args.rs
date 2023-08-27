use clap::Parser;

/// Encapsulates command arguments into a struct
#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CommandArgs {
    /// Instance where the bot has been created
    pub instance: String,

    /// Community to post to 
    pub community: String,

    /// Filter posts based on minimum upvotes
    #[arg(long)]
    pub min_ups: Option<u64>,

    /// Filter posts based on maximum upvotes
    #[arg(long)]
    pub max_ups: Option<u64>
}