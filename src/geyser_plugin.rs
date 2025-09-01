use {
    crate::structured_logger::StructuredLogger,
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaTransactionInfoVersions, Result as GeyserPluginResult, SlotStatus,
    },
    log::*,
    serde::{Deserialize, Serialize},
    std::{fs::File, io::Read},
};

pub struct GeyserPluginHook {
    config: Option<GeyserPluginConfig>,
}

impl Default for GeyserPluginHook {
    fn default() -> Self {
        Self { config: None }
    }
}

impl std::fmt::Debug for GeyserPluginHook {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct GeyserPluginConfig {
    pub output_file: Option<String>,
    pub account_owners_filter: Option<Vec<String>>,
}

type Slot = u64;

impl GeyserPlugin for GeyserPluginHook {
    fn name(&self) -> &'static str {
        "GeyserPluginHook"
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> GeyserPluginResult<()> {
        solana_logger::setup_with_default("info");
        info!("[on_load] - config_file: {:#?}", config_file);

        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let result: serde_json::Value = serde_json::from_str(&contents).unwrap();

        let config = Self::extract_config(&result);
        self.config = Some(config.clone());

        if let Some(config) = &self.config {
            info!("[on_load] - output_file: {:#?}", config.output_file);

            StructuredLogger::log(
                "plugin_loaded",
                None,
                serde_json::json!({
                    "config_file": config_file,
                    "config": config
                }),
            );

            info!("[on_load] - Logger initialized successfully");
        }

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("[on_unload] - Starting plugin unload");

        StructuredLogger::log("plugin_unloading", None, serde_json::json!({}));

        info!("[on_unload] - Plugin unload complete");
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> GeyserPluginResult<()> {
        StructuredLogger::log_account_update(&account, slot, is_startup);
        Ok(())
    }

    fn notify_end_of_startup(&self) -> GeyserPluginResult<()> {
        StructuredLogger::log("startup_completed", None, serde_json::json!({}));
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: Slot,
        parent: Option<u64>,
        status: &SlotStatus,
    ) -> GeyserPluginResult<()> {
        StructuredLogger::log_slot_status(slot, parent, status);
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> GeyserPluginResult<()> {
        StructuredLogger::log_transaction(&transaction, slot);
        Ok(())
    }

    fn notify_block_metadata(&self, blockinfo: ReplicaBlockInfoVersions) -> GeyserPluginResult<()> {
        StructuredLogger::log_block_metadata(&blockinfo);
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        info!(
            "[account_data_notifications_enabled] - plugin interface is asking if data notifs should be enabled?"
        );
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        info!(
            "[transaction_notifications_enabled] - plugin interface is asking if transactions notifs should be enabled?"
        );
        true
    }
}

impl GeyserPluginHook {
    fn extract_config(config_json: &serde_json::Value) -> GeyserPluginConfig {
        let config_section = &config_json["config"];

        if config_section.is_object() {
            match serde_json::from_value::<GeyserPluginConfig>(config_section.clone()) {
                Ok(config) => config,
                Err(err) => {
                    error!("Failed to parse config: {}", err);
                    GeyserPluginConfig::default()
                }
            }
        } else {
            warn!("Config section is not an object, using default config");
            GeyserPluginConfig::default()
        }
    }
}
