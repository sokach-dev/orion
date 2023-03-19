mod openai;

use anyhow::Result;
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
    OpenAIWord {
        /// openai key, or through env `OPENAI_KEY`
        #[clap(short, long)]
        key: Option<String>,
        /// openai url, or through env `OPENAI_URL`
        #[clap(short, long)]
        url: Option<String>,
        /// word list file
        #[clap(short, long)]
        word_list: String,
        /// prompt
        #[clap(short, long)]
        prompt: Option<String>,
        /// vocabulary rpc server, eg: http://127.0.0.1:5015; or through env `VOCABULARY_RPC_SERVER`
        #[clap(short, long)]
        rpc_server: Option<String>,
    },
    OpenAIStory {
        /// openai key, or through env `OPENAI_KEY`
        #[clap(short, long)]
        key: Option<String>,
        /// openai url, or through env `OPENAI_URL`
        #[clap(short, long)]
        url: Option<String>,
        /// generate a story with how many word, commend 5.
        #[clap(short, long)]
        word_amount: i64,
        /// total need how many story
        #[clap(short, long)]
        total_generate_amount: u32,
        /// prompt
        #[clap(short, long)]
        prompt: Option<String>,
        /// vocabulary rpc server, eg: http://127.0.0.1:5015; or through env `VOCABULARY_RPC_SERVER`
        #[clap(short, long)]
        rpc_server: Option<String>,
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
        Commands::OpenAIWord {
            key,
            url,
            word_list,
            prompt,
            rpc_server,
        } => {
            let (key, url, rpc_server) =
                check_params(key.clone(), url.clone(), rpc_server.clone()).unwrap();

            if fs::metadata(word_list.clone()).is_err() {
                println!("File not found: {}", word_list);
                return;
            }

            let prompt = prompt
                .clone()
                .unwrap_or_else(|| "你将作为我的英语老师翻译单词的这些信息：单词、英式音标、词根词缀、中文释义、常用搭配、近义词、例句".to_string());

            let op = openai::OpenAI::new(rpc_server, key, url, prompt)
                .await
                .unwrap();

            op.add_word_from_file(word_list.clone()).await.unwrap();
        }
        Commands::OpenAIStory {
            key,
            url,
            word_amount,
            total_generate_amount,
            prompt,
            rpc_server,
        } => {
            let (key, url, rpc_server) =
                check_params(key.clone(), url.clone(), rpc_server.clone()).unwrap();

            let prompt = prompt
                .clone()
                .unwrap_or_else(|| "请使用这些单词用英语讲一个小故事，且给出翻译并用括号包裹".to_string());

            if *total_generate_amount < 1 || *word_amount < 1 {
                panic!("give right total_generate_amount or word_amount");
            }

            let op = openai::OpenAI::new(rpc_server, key, url, prompt)
                .await
                .unwrap();

            op.generate_story_with_words(*word_amount, *total_generate_amount)
                .await
                .unwrap();

            tracing::info!("story task over!");
            std::future::pending::<()>().await;
        }
    }
}
fn check_params(
    key: Option<String>,
    url: Option<String>,
    rpc_server: Option<String>,
) -> Result<(String, String, String)> {
    let key = key
        .clone()
        .unwrap_or_else(|| env::var("OPENAI_KEY").expect("need key for openai"));

    let url = url.clone().unwrap_or_else(|| {
        env::var("OPENAI_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string())
    });

    let rpc_server = rpc_server
        .clone()
        .unwrap_or_else(|| env::var("ORION_RPC_SERVER").expect("need rpc server"));

    Ok((key, url, rpc_server))
}
