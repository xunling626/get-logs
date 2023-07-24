use log::{info, error};

use crate::{types::{LogsjsonrpData, Jsonrpc, ReturnLogs}, logfile::log::init_log};

pub mod types;
pub mod logfile;

async fn getlogs(url : String,from_bolck : String , to_block : String , address : Vec<String> , methods : Vec<String> , from : Vec<String> , to : Vec<String> ){
    init_log("info".to_string());
    let mut topics = Vec::new();
    topics.push(methods);
    topics.push(from);
    topics.push(to);
    info!("进入getlogs方法");
    let logsjsonrpc = LogsjsonrpData{
        from_block: from_bolck,
        to_block : to_block,
        address: address,
        topics : topics,
    };
    let logs = Jsonrpc {
        jsonrpc: String::from("2.0"),
        method: String::from("eth_getLogs"),
        params: vec![logsjsonrpc],
        id: 1,
    };
    println!("{:#?}",logs);
    if let Ok(res) = surf::post(&url).body_json(&logs) {
        match res.recv_json::<ReturnLogs>().await {
            Ok(t) => {
                // println!("{:#?}",t);
                println!("len: {}", t.result.len());
                for x in t.result {
                    info!("logs address : {:#?}" , x);
                }
            }
            Err(e) => {
                error!(target:"app","{:#?}",e);
            }
        }
    }else {
        println!("错误");
    }
}

#[async_std::test]
async fn test_logs(){
    getlogs("https://eth.llamarpc.com/".to_string(), "0x10eb199".to_string(),
    "0x10eb199".to_string(), 
    vec!["0x8a1312e2fd07a4816e85704de5f3eeb58beb3d31".to_string(),"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string() , "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string()], 
    vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),"0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925".to_string()], 
    vec![], 
    vec!["0x000000000000000000000000493b733204e3ab799544cc4e73fc75707f8889e3".to_string()]).await;
}