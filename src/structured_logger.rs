use {
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        ReplicaAccountInfoVersions, ReplicaBlockInfoVersions, ReplicaTransactionInfoVersions,
        SlotStatus,
    },
    bs58,
    chrono::{DateTime, Utc},
    log::info,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub slot: Option<u64>,
    pub data: serde_json::Value,
}

pub struct StructuredLogger;

impl StructuredLogger {
    pub fn log(event_type: &str, slot: Option<u64>, data: serde_json::Value) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            slot,
            data,
        };

        if let Ok(json_line) = serde_json::to_string(&entry) {
            info!("{}", json_line);
        }
    }

    pub fn log_account_update(
        account: &ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) {
        let data = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                serde_json::json!({
                    "version": "V0_0_1",
                    "error": "Version not supported"
                })
            }
            ReplicaAccountInfoVersions::V0_0_2(account) => {
                serde_json::json!({
                    "version": "V0_0_2",
                    "pubkey": bs58::encode(account.pubkey).into_string(),
                    "owner": bs58::encode(account.owner).into_string(),
                    "lamports": account.lamports,
                    "data_len": account.data.len(),
                    "executable": account.executable,
                    "rent_epoch": account.rent_epoch,
                    "is_startup": is_startup
                })
            }
            ReplicaAccountInfoVersions::V0_0_3(account) => {
                serde_json::json!({
                    "version": "V0_0_3",
                    "pubkey": bs58::encode(account.pubkey).into_string(),
                    "owner": bs58::encode(account.owner).into_string(),
                    "lamports": account.lamports,
                    "data_len": account.data.len(),
                    "executable": account.executable,
                    "rent_epoch": account.rent_epoch,
                    "write_version": account.write_version,
                    "is_startup": is_startup
                })
            }
        };

        Self::log("account_update", Some(slot), data);
    }

    pub fn log_transaction(transaction: &ReplicaTransactionInfoVersions, slot: u64) {
        let data = match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(tx) => {
                serde_json::json!({
                    "version": "V0_0_1",
                    "signature": bs58::encode(&tx.signature).into_string(),
                    "is_vote": tx.is_vote,
                    "transaction_status_meta": format!("{:?}", tx.transaction_status_meta)
                })
            }
            ReplicaTransactionInfoVersions::V0_0_2(tx) => {
                serde_json::json!({
                    "version": "V0_0_2",
                    "signature": bs58::encode(&tx.signature).into_string(),
                    "is_vote": tx.is_vote,
                    "transaction_status_meta": format!("{:?}", tx.transaction_status_meta),
                    "index": tx.index
                })
            }
            ReplicaTransactionInfoVersions::V0_0_3(tx) => {
                serde_json::json!({
                    "version": "V0_0_3",
                    "signature": bs58::encode(&tx.signature).into_string(),
                    "is_vote": tx.is_vote,
                    "transaction_status_meta": format!("{:?}", tx.transaction_status_meta),
                    "index": tx.index
                })
            }
        };

        Self::log("transaction", Some(slot), data);
    }

    pub fn log_block_metadata(blockinfo: &ReplicaBlockInfoVersions) {
        let (data, slot) = match blockinfo {
            ReplicaBlockInfoVersions::V0_0_1(block) => (
                serde_json::json!({
                    "version": "V0_0_1",
                    "blockhash": bs58::encode(&block.blockhash).into_string(),
                    "rewards": format!("{:?}", block.rewards),
                    "block_time": block.block_time,
                }),
                block.slot,
            ),
            ReplicaBlockInfoVersions::V0_0_2(block) => (
                serde_json::json!({
                    "version": "V0_0_2",
                    "blockhash": bs58::encode(&block.blockhash).into_string(),
                    "rewards": format!("{:?}", block.rewards),
                    "block_time": block.block_time,
                    "block_height": block.block_height,
                }),
                block.slot,
            ),
            ReplicaBlockInfoVersions::V0_0_3(block) => (
                serde_json::json!({
                    "version": "V0_0_3",
                    "blockhash": bs58::encode(&block.blockhash).into_string(),
                    "rewards": format!("{:?}", block.rewards),
                    "block_time": block.block_time,
                    "block_height": block.block_height,
                    "executed_transaction_count": block.executed_transaction_count,
                }),
                block.slot,
            ),
            ReplicaBlockInfoVersions::V0_0_4(block) => (
                serde_json::json!({
                    "version": "V0_0_4",
                    "blockhash": bs58::encode(&block.blockhash).into_string(),
                    "rewards": format!("{:?}", block.rewards),
                    "block_time": block.block_time,
                    "block_height": block.block_height,
                    "executed_transaction_count": block.executed_transaction_count,
                }),
                block.slot,
            ),
        };

        Self::log("block_metadata", Some(slot), data);
    }

    pub fn log_slot_status(slot: u64, parent: Option<u64>, status: &SlotStatus) {
        let data = serde_json::json!({
            "parent": parent,
            "status": format!("{:?}", status)
        });

        Self::log("slot_status", Some(slot), data);
    }
}
