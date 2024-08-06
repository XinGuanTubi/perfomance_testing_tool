use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AdQueryData<'a> {
    #[serde(rename = "ap.debug")]
    ap_debug: &'a str,
    caller: AdQueryCallerData<'a>,
    stream_name: &'a str,
    exposure_id: &'a str,
    duration: u32,
    #[serde(rename = "ap.pt")]
    ap_pt: &'a str,
    #[serde(rename = "ap.sid")]
    ap_sid: &'a str,
    params: AdQueryParamsData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AdQueryCallerData<'a> {
    ip: &'a str,
    inst_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AdQueryParamsData<'a> {
    break_number: u32,
    content_id: &'a str,
    platform: &'a str,
    pub_id: &'a str,
    device_id: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let ad_query_data: AdQueryData = AdQueryData {
        ap_debug: "2",
        caller: AdQueryCallerData {
            ip: "127.0.0.1",
            inst_name: "load_test_stitcher_inst_name"
        },
        stream_name: "marge",
        exposure_id: "load_test_exposure_id",
        duration: 120_000,
        ap_pt: "1",
        ap_sid: "load_test_session_id",
        params: AdQueryParamsData {
            break_number: 1,
            content_id: "14",
            platform: "IPHONE",
            pub_id: "bf566fa34e54239e78c26db6ef5f977a",
            device_id: "70C76B9E-2326-40EE-BA17-4C1E017447E3",
        },
    };

    let concurrency = 256;

    let client = reqwest::Client::new();
    let mut handles = Vec::with_capacity(concurrency);

    for _ in 0..concurrency {
        let client = client.clone();
        let ad_query_data = ad_query_data.clone();
        let handle = tokio::spawn(async move {
            loop {
                let response_future = client
                    .clone()
                    // .post("http://127.0.0.1:18002/ad_query")
                    .post("http://apollo-promoter:18001/ad_query")
                    .json(&ad_query_data)
                    .send();

                let handle = response_future.await;

                let response_result = handle;
                match response_result {
                    Ok(response) => {
                        response.bytes().await.expect("can not read body");
                    }
                    Err(_err) => {
                        // info!("{:?}", err);
                    }
                }

                tokio::time::sleep(Duration::from_micros(1)).await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = tokio::join!(handle);
    }

    Ok(())
}


