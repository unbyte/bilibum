use crate::client::Client;
use crate::prompt::Prompt;

mod client;
mod config;
mod prompt;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let p = Prompt::new();
            let client = Client::new(None);
            let id = p.ask_video_post_id().unwrap();
            let vp = client.get_video_post(&id).await.unwrap();
            let ids = p.select_video_ids(&vp.videos).unwrap();
            println!("{:?}", ids);
        })
}
