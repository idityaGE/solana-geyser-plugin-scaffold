use {
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        ReplicaAccountInfoVersions, ReplicaBlockInfoVersions, ReplicaTransactionInfoVersions,
        SlotStatus,
    },
    bs58,
    chrono::{DateTime, Utc},
    log::error,
    serde::{Deserialize, Serialize},
    std::{
        fs::OpenOptions,
        io::{BufWriter, Write},
        sync::mpsc::{self, Sender},
        thread,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub slot: Option<u64>,
    pub data: serde_json::Value,
}

#[derive(Clone)]
pub struct ThreadSafeLogger {
    sender: Sender<LogEntry>,
}

impl ThreadSafeLogger {
    pub fn new(output_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, receiver) = mpsc::channel::<LogEntry>();
        let output_file = output_file.to_string();

        thread::spawn(move || {
            let file = match OpenOptions::new()
                .create(true)
                .append(true)
                .open(&output_file)
            {
                Ok(file) => file,
                Err(e) => {
                    error!("Failed to open log file {}: {}", output_file, e);
                    return;
                }
            };

            let mut writer = BufWriter::new(file);

            while let Ok(entry) = receiver.recv() {
                if let Ok(json_line) = serde_json::to_string(&entry) {
                    if let Err(e) = writeln!(writer, "{}", json_line) {
                        error!("Failed to write to log file: {}", e);
                    } else if let Err(e) = writer.flush() {
                        error!("Failed to flush log file: {}", e);
                    }
                }
            }
        });

        Ok(ThreadSafeLogger { sender })
    }

    pub fn log(&self, event_type: &str, slot: Option<u64>, data: serde_json::Value) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            slot,
            data,
        };

        if let Err(e) = self.sender.send(entry) {
            error!("Failed to send log entry to background thread: {}", e);
        }
    }

    pub fn log_account_update(
        &self,
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

        self.log("account_update", Some(slot), data);
    }

    pub fn log_transaction(&self, transaction: &ReplicaTransactionInfoVersions, slot: u64) {
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

        self.log("transaction", Some(slot), data);
    }

    pub fn log_block_metadata(&self, blockinfo: &ReplicaBlockInfoVersions) {
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

        self.log("block_metadata", Some(slot), data);
    }

    pub fn log_slot_status(&self, slot: u64, parent: Option<u64>, status: &SlotStatus) {
        let data = serde_json::json!({
            "parent": parent,
            "status": format!("{:?}", status)
        });

        self.log("slot_status", Some(slot), data);
    }
}
