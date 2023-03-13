mod openai;

use clap::{Parser, Subcommand};
use std::{env, fs};
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "sokach-dev")]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    OpenAI {
        /// openai key
        #[clap(short, long)]
        key: Option<String>,
        /// openai url
        #[clap(short, long)]
        url: Option<String>,
        /// database url
        #[clap(short, long)]
        db: Option<String>,
        /// word list file
        #[clap(short, long)]
        word_list: String,
        /// prompt
        #[clap(short, long)]
        prompt: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt()
        .with_timer(local_time)
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .with_line_number(true)
        .with_file(true)
        .init();

    let cli = Cli::parse();

    match &cli.subcmd {
        Commands::OpenAI {
            key,
            url,
            db,
            word_list,
            prompt,
        } => {
            let key = key
                .clone()
                .unwrap_or_else(|| env::var("OPENAI_KEY").unwrap());

            let url = url.clone().unwrap_or_else(|| {
                env::var("OPENAI_URL")
                    .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string())
            });

            let db = db
                .clone()
                .unwrap_or_else(|| env::var("VOCABULARY_DB").unwrap());

            if fs::metadata(word_list).is_err() {
                println!("File not found: {}", word_list);
                return;
            }

            let prompt = prompt
                .clone()
                .unwrap_or_else(|| "你将作为我的英语老师翻译单词的这些信息：单词、英式音标、词根词缀、中文释义、常用搭配、近义词、例句".to_string());

            let db_config = abi::DbConfig { database_url: db };

            openai::add_word_to_db(db_config, &key, &url, word_list, &prompt)
                .await
                .unwrap();
        }
    }
}
