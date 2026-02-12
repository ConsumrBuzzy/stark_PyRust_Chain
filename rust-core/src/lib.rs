use pyo3::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod vault;
mod starknet_client;
mod supply_chain;
mod rate_limiter;
mod influence_api;

use vault::Vault;
use starknet_client::StarknetClient;
use supply_chain::{SupplyChainGraph, Recipe};
use influence_api::{InfluenceClient, Asteroid};
use std::collections::HashMap;

// --- PyO3 Wrappers ---

#[pyclass]
struct PyVault {
    inner: Vault,
}

#[pymethods]
impl PyVault {
    #[new]
    fn new(password: &str) -> PyResult<Self> {
        let vault = Vault::new(password).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(PyVault { inner: vault })
    }

    fn encrypt(&self, plaintext: &str) -> PyResult<String> {
        self.inner.encrypt(plaintext).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn decrypt(&self, ciphertext: &str) -> PyResult<String> {
        self.inner.decrypt(ciphertext).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
struct PyStarknetClient {
    inner: Arc<StarknetClient>,
    rt: Runtime,
}

#[pymethods]
impl PyStarknetClient {
    #[new]
    fn new(rpc_url: Option<String>) -> PyResult<Self> {
        let url_slice = rpc_url.as_deref();
        let client = StarknetClient::new(url_slice).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let rt = Runtime::new().unwrap();
        Ok(PyStarknetClient { 
            inner: Arc::new(client),
            rt 
        })
    }

    fn get_block_number(&self) -> PyResult<u64> {
        self.rt.block_on(async {
            self.inner.get_block_number().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

#[pyclass]
struct PySupplyChain {
    inner: SupplyChainGraph,
}

#[pymethods]
impl PySupplyChain {
    #[new]
    fn new() -> Self {
        PySupplyChain { inner: SupplyChainGraph::new() }
    }

    fn add_recipe(&mut self, name: String, inputs: HashMap<String, u32>, outputs: HashMap<String, u32>, time: u32) {
        let recipe = Recipe {
            inputs,
            outputs,
            process_time_seconds: time,
        };
        self.inner.add_recipe(&name, recipe);
    }
    
    fn find_sources(&self, resource: String) -> Option<Vec<String>> {
        self.inner.find_production_path(&resource)
    }
}

#[pyclass]
struct PyInfluenceClient {
    inner: Arc<InfluenceClient>,
    rt: Runtime,
}

#[pymethods]
impl PyInfluenceClient {
    #[new]
    fn new() -> PyResult<Self> {
        let client = InfluenceClient::new().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let rt = Runtime::new().unwrap();
        Ok(PyInfluenceClient {
            inner: Arc::new(client),
            rt,
        })
    }
    
    fn get_asteroid(&self, asteroid_id: u64) -> PyResult<String> {
         // Returning JSON string for simplicity in Python for now
         self.rt.block_on(async {
             let asteroid = self.inner.get_asteroid(asteroid_id).await
                 .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
             serde_json::to_string(&asteroid).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
         })
    }
}

#[pymodule]
fn stark_pyrust_chain(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVault>()?;
    m.add_class::<PyStarknetClient>()?;
    m.add_class::<PySupplyChain>()?;
    m.add_class::<PyInfluenceClient>()?;
    Ok(())
}
