use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use dilithium::{DilithiumSecretKey, DilithiumPublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    gates: Vec<Gate>,
    qubits: Vec<Qubit>,
    depth: usize,
    error_rates: Vec<f64>,
    signature: Option<Vec<u8>>,  // Dilithium signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub gate_type: GateType,
    pub qubits: Vec<usize>,
    pub parameters: Vec<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GateType {
    Hadamard,
    CNOT,
    PauliX,
    PauliY,
    PauliZ,
    Phase(f64),
    Rotation(Axis, f64),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    pub index: usize,
    pub error_rate: f64,
    pub connectivity: Vec<usize>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            qubits: Vec::new(),
            depth: 0,
            error_rates: Vec::new(),
            signature: None,
        }
    }

    pub fn add_qubit(&mut self, error_rate: f64, connectivity: Vec<usize>) -> usize {
        let index = self.qubits.len();
        self.qubits.push(Qubit {
            index,
            error_rate,
            connectivity,
        });
        self.error_rates.push(error_rate);
        index
    }

    pub fn add_gate(&mut self, gate: Gate) -> Result<()> {
        // Validate qubit indices
        for &qubit_idx in &gate.qubits {
            if qubit_idx >= self.qubits.len() {
                anyhow::bail!("Invalid qubit index: {}", qubit_idx);
            }
        }

        self.gates.push(gate);
        self.update_depth();
        Ok(())
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn get_error_rates(&self) -> Result<Vec<f64>> {
        Ok(self.error_rates.clone())
    }

    pub fn reduce_gates(&mut self, gates_to_remove: &[Gate]) -> Result<()> {
        self.gates.retain(|gate| !gates_to_remove.contains(gate));
        self.update_depth();
        Ok(())
    }

    pub fn reduce_depth(&mut self, max_depth: usize) -> Result<()> {
        if self.depth <= max_depth {
            return Ok(());
        }

        // Simple depth reduction by removing gates from the end
        while self.depth > max_depth && !self.gates.is_empty() {
            self.gates.pop();
            self.update_depth();
        }

        Ok(())
    }

    pub fn optimize_layout(&mut self, new_layout: &[Qubit]) -> Result<()> {
        if new_layout.len() != self.qubits.len() {
            anyhow::bail!("New layout must have the same number of qubits");
        }

        self.qubits = new_layout.to_vec();
        self.error_rates = self.qubits.iter().map(|q| q.error_rate).collect();
        Ok(())
    }

    pub fn sign(&mut self, key: &DilithiumSecretKey) -> Result<()> {
        let circuit_bytes = serde_json::to_vec(&self)?;
        self.signature = Some(key.sign(&circuit_bytes));
        Ok(())
    }

    pub fn verify(&self, key: &DilithiumPublicKey) -> Result<bool> {
        let circuit_bytes = serde_json::to_vec(&self)?;
        if let Some(sig) = &self.signature {
            Ok(key.verify(&circuit_bytes, sig))
        } else {
            Ok(false)
        }
    }

    fn update_depth(&mut self) {
        // Simple depth calculation - can be improved with parallel gate analysis
        self.depth = self.gates.len();
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quantum Circuit:")?;
        writeln!(f, "Number of qubits: {}", self.qubits.len())?;
        writeln!(f, "Circuit depth: {}", self.depth)?;
        writeln!(f, "\nGates:")?;
        
        for (i, gate) in self.gates.iter().enumerate() {
            writeln!(
                f,
                "{}. {:?} on qubits {:?} with parameters {:?}",
                i + 1,
                gate.gate_type,
                gate.qubits,
                gate.parameters
            )?;
        }

        writeln!(f, "\nError rates:")?;
        for (i, rate) in self.error_rates.iter().enumerate() {
            writeln!(f, "Qubit {}: {:.6}", i, rate)?;
        }

        Ok(())
    }
}
