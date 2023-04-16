use voicevox_api::apis::{configuration::Configuration, default_api as voicevox};

#[tokio::main]
async fn main() {
    let conf = Configuration::new();

    speakers(&conf).await;
    engine_manifest(&conf).await;
}

async fn speakers(conf: &Configuration) {
    let speakers = voicevox::speakers_speakers_get(&conf, None).await.unwrap();
    speakers.iter().for_each(|speaker| {
        speaker.styles.iter().for_each(|style| {
            println!("{}: {}, id: {}", speaker.name, style.name, style.id);
        });
    });
}

async fn engine_manifest(conf: &Configuration) {
    let engine_manifest = voicevox::engine_manifest_engine_manifest_get(&conf)
        .await
        .unwrap();
    println!("{:#?}", engine_manifest);
}
