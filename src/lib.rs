

use surf::Error;

use crate::types::{Jsonrpc, LogsjsonrpData, ReturnLogs};

pub mod types;

async fn getlogs(
    url: String,
    from_bolck: String,
    to_block: String,
    address: Vec<String>,
    methods: Vec<String>,
    from: Vec<String>,
    to: Vec<String>,
) -> Result<ReturnLogs,Error>{
    let mut topics = Vec::new();
    topics.push(methods);
    topics.push(from);
    topics.push(to);
    let logsjsonrpc = LogsjsonrpData {
        from_block: from_bolck,
        to_block: to_block,
        address: address,
        topics: topics,
    };
    let logs = Jsonrpc {
        jsonrpc: String::from("2.0"),
        method: String::from("eth_getLogs"),
        params: vec![logsjsonrpc],
        id: 1,
    };
    match  surf::post(&url).body_json(&logs) {
        Ok(t) => {
            match t.recv_json::<ReturnLogs>().await {
                Ok(logs) => Ok(logs),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

#[async_std::test]
async fn test_logs() {
    let log = getlogs(
        "https://eth.llamarpc.com/".to_string(),
        "0x10eb199".to_string(),
        "0x10eb199".to_string(),
        vec![
            "0x8a1312e2fd07a4816e85704de5f3eeb58beb3d31".to_string(),
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
            "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
        ],
        vec![
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925".to_string(),
        ],
        vec![],
        vec!["0x000000000000000000000000493b733204e3ab799544cc4e73fc75707f8889e3".to_string()],
    )
    .await;
    match log {
        Ok(x) => println!("{:#?}", x),
        Err(e) => println!("{:#?}",e)
    }
}
