use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add {
        service: String,
        username: String,
        password: String,
    },
    Remove {
        service: String,
    },
    Get {
        service: String,
    },
    Init,
    Edit {
        service: String,
        username: Option<String>,
        password: Option<String>,
    },
    Export {
        filepath: String,
    },
}
