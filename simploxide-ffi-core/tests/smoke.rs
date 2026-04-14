use simploxide_ffi_core::{DbOpts, DefaultUser, SimplexVersion};

#[tokio::test]
async fn smoke() {
    let _ = std::fs::remove_dir_all("./test_db");
    std::fs::create_dir("./test_db").unwrap();

    let (client, mut events) = simploxide_ffi_core::init(
        DefaultUser {
            display_name: "Test".to_owned(),
            is_bot: true,
        },
        DbOpts::unencrypted("./test_db/smoke"),
    )
    .await
    .unwrap();

    let handle = tokio::spawn(async move {
        let mut events_count = 0;

        while let Some(event) = events.next_event().await {
            println!("{event:?}\n\n");
            let _ = event.unwrap();
            events_count += 1;
        }

        assert!(events_count > 0);
    });

    let version = client.version().await.unwrap();
    assert_eq!(version, SimplexVersion::new(6, 5, 0, 11));

    let output = client.send("/users".to_owned()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        json["result"]["users"][0]["user"]["localDisplayName"].as_str(),
        Some("Test")
    );

    let output = client.send("/sa".to_owned()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(json["error"].is_object());

    let output = client.send("/ad".to_owned()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        json["result"]["type"].as_str(),
        Some("userContactLinkCreated")
    );

    let created_address = json["result"]["connLinkContact"]["connShortLink"].as_str();

    let output = client.send("/sa".to_owned()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let existing_address =
        json["result"]["contactLink"]["connLinkContact"]["connShortLink"].as_str();

    assert_eq!(created_address, existing_address);

    let output = client.send("/da".to_owned()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        json["result"]["type"].as_str(),
        Some("userContactLinkDeleted")
    );

    let cloned_client = client.clone();

    // handle.await must hang without disconnect or droppin both clients
    cloned_client.disconnect().await;
    handle.await.unwrap();
}
