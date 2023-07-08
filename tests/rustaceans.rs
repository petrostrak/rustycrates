use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!(
            {"email":"pit.trak@gmail.com",
            "name": "petros trak"}
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json::<Value>().unwrap()
}

fn clean_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean_1 = create_test_rustacean(&client);
    let rustacean_2 = create_test_rustacean(&client);

    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let response_json = response.json::<Value>().unwrap();
    assert!(response_json.as_array().unwrap().contains(&rustacean_1));
    assert!(response_json.as_array().unwrap().contains(&rustacean_2));

    clean_test_rustacean(&client, rustacean_1);
    clean_test_rustacean(&client, rustacean_2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();

    let rustacean = create_test_rustacean(&client);
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "email":"pit.trak@gmail.com",
            "name": "petros trak",
            "created_at": rustacean["created_at"]
        })
    );

    clean_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .json(&json!({"email":"trak.pit@gmail.com",
            "name": "trak petros"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean = response.json::<Value>().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "email":"trak.pit@gmail.com",
            "name": "trak petros",
            "created_at": rustacean["created_at"]
        })
    );
    clean_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let json = create_test_rustacean(&client);

    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", json["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}
