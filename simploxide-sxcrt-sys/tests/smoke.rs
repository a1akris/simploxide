use simploxide_sxcrt_sys::{InitError, MigrationConfirmation, SimpleXChat};

#[test]
fn smoke_test() {
    let _ = std::fs::remove_dir_all("./test_db");
    std::fs::create_dir("./test_db").unwrap();
    let mut chat = SimpleXChat::init(
        "./test_db/smoke".to_owned(),
        "smoke".to_owned(),
        MigrationConfirmation::YesUp,
    )
    .unwrap();

    assert!(matches!(
        SimpleXChat::init(
            "./test_db/smoke".to_owned(),
            "wrong_password".to_owned(),
            MigrationConfirmation::YesUp
        ),
        Err(InitError::DbError(_))
    ));

    let output = chat.send_cmd("/v".to_owned()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        json["result"]["versionInfo"]["version"].as_str(),
        Some("6.5.0.9")
    );

    let output = chat.send_cmd("/users".to_owned()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(json["result"]["users"].as_array().unwrap().is_empty());

    let output = chat.send_cmd("/create user Test".to_owned()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(json["result"]["type"].as_str(), Some("activeUser"));

    let output = chat.send_cmd("/_start".to_owned()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(json["result"]["type"].as_str(), Some("chatStarted"));

    let output = chat
        .recv_msg_wait(std::time::Duration::from_millis(500))
        .unwrap();

    assert!(!output.is_empty());

    let output = chat.send_cmd("/_stop".to_owned()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(json["result"]["type"].as_str(), Some("chatStopped"));
}
