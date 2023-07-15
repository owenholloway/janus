use serial_test::serial;
use tokio::net::TcpListener;

use crate::transport::bind_tcp;

static TEST_ADDRESS: &str = "127.0.0.1";
static TEST_PORT: &str = "51222";
static TEST_PORT_SECOND: &str = "51223";

pub async fn given_tcp_socket_is_open(socket_port: String) -> TcpListener {
    bind_tcp(TEST_ADDRESS.to_string(), socket_port.to_string())
        .await
        .unwrap()
}

#[tokio::test]
#[serial]
pub async fn can_bind_local() {
    let bind = bind_tcp(TEST_ADDRESS.to_string(), TEST_PORT.to_string()).await;

    assert!(bind.is_ok())
}

#[tokio::test]
#[serial]
pub async fn can_bind_second_local_different_port() {
    let _existing_port = given_tcp_socket_is_open(TEST_PORT_SECOND.to_string()).await;

    let bind = bind_tcp(TEST_ADDRESS.to_string(), TEST_PORT.to_owned()).await;

    assert!(bind.is_ok())
}

#[tokio::test]
#[serial]
pub async fn cannot_bind_second_local_same_port() {
    let _existing_port = given_tcp_socket_is_open(TEST_PORT.to_string()).await;

    let bind = bind_tcp(TEST_ADDRESS.to_string(), TEST_PORT.to_owned()).await;

    assert!(bind.is_err())
}

#[tokio::test]
#[serial]
pub async fn can_open_bind_close_repeat_100() {
    for _ in 0..100 {
        open_binding().await;
    }
}

async fn open_binding() {
    let bind = bind_tcp(TEST_ADDRESS.to_string(), TEST_PORT.to_string()).await;
    assert!(bind.is_ok());
}
