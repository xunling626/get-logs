use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Jsonrpc {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<LogsjsonrpData>,
    pub id: u8,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogsjsonrpData{
    #[serde(rename = "fromBlock")]
    pub from_block : String,
    #[serde(rename = "toBlock")]
    pub to_block : String,
    pub address : Vec<String>,
    pub topics : Vec<Vec<String>>
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct ReturnLogs {
    pub jsonrpc: String,
    pub id: u8,
    pub result: Vec<LogsResult>,
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct LogsResult{
    pub address : String,
    pub topics : Vec<String>,
    pub data : String,
    #[serde(rename = "blockNumber")]
    pub block_number : String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash : String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index : String,
    #[serde(rename = "blockHash")]
    pub block_hash : String,
    #[serde(rename = "logIndex")]
    pub log_index : String,
    pub removed : bool
}