use std::collections::BTreeSet;

use communication::{
    client::{Communication, ConnectionStatus, Cycler, CyclerOutput},
    messages::{Fields, Path},
};

use serde_json::Value;
use tokio::runtime::{Builder, Runtime};

use crate::{image_buffer::ImageBuffer, value_buffer::ValueBuffer};

pub struct Nao {
    communication: Communication,
    runtime: Runtime,
}

impl Nao {
    pub fn new(address: Option<String>, connect: bool) -> Self {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let _guard = runtime.enter();
        let communication = Communication::new(address, connect);
        Self {
            communication,
            runtime,
        }
    }

    pub fn set_connect(&self, connect: bool) {
        self.runtime
            .block_on(self.communication.set_connect(connect))
    }

    pub fn set_address(&self, address: String) {
        self.runtime
            .block_on(self.communication.set_address(address));
    }

    pub fn subscribe_output(&self, output: CyclerOutput) -> ValueBuffer {
        let _guard = self.runtime.enter();
        ValueBuffer::output(self.communication.clone(), output)
    }

    pub fn subscribe_image(&self, cycler: Cycler) -> ImageBuffer {
        let _guard = self.runtime.enter();
        ImageBuffer::new(self.communication.clone(), cycler)
    }

    pub fn subscribe_parameter(&self, path: &str) -> ValueBuffer {
        let _guard = self.runtime.enter();
        ValueBuffer::parameter(self.communication.clone(), path.to_string())
    }

    pub fn subscribe_status_updates(&self) -> tokio::sync::mpsc::Receiver<ConnectionStatus> {
        let _guard = self.runtime.enter();
        self.runtime
            .block_on(self.communication.subscribe_connection_updates())
    }

    pub fn get_output_fields(&self) -> Option<Fields> {
        self.runtime
            .block_on(self.communication.get_output_fields())
    }

    pub fn get_parameter_fields(&self) -> Option<BTreeSet<Path>> {
        self.runtime
            .block_on(self.communication.get_parameter_fields())
    }

    pub fn update_parameter_value(&self, path: &str, value: Value) {
        self.runtime
            .block_on(self.communication.update_parameter_value(path, value));
    }
}
