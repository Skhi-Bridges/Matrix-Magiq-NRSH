//! Actor Token Convention for Matrix-Magiq
//! 
//! Implements the custom token convention that includes:
//! - Quantum security integration
//! - Permaweb.lib profile/zone
//! - Multi-currency wallet for each actor
//! - DEX integration

use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::crypto::{QuantumCrypto, QkdProtocol};

/// Actor Token representing an actor's identity, security, and financial capabilities
pub struct ActorToken {
    /// Unique identifier for this token
    id: String,
    
    /// Actor's friendly name
    name: String,
    
    /// Actor's zone (job/responsibility)
    zone: String,
    
    /// Permaweb profile information
    permaweb_profile: PermawebProfile,
    
    /// Multi-currency wallet
    wallet: MultiCurrencyWallet,
    
    /// Quantum security provider
    quantum_security: Arc<RwLock<QuantumCrypto>>,
    
    /// Whether this token has been activated
    is_active: bool,
    
    /// DEX integration
    dex_integration: DexIntegration,
    
    /// Token-specific metadata
    metadata: HashMap<String, String>,
    
    /// Quantum entanglement references (for coherence)
    entanglement_references: Vec<String>,
}

/// Permaweb profile for an actor
pub struct PermawebProfile {
    /// Permaweb address
    address: String,
    
    /// Profile hash
    profile_hash: String,
    
    /// Profile zone (categorizes the actor's function)
    zone: String,
    
    /// Profile visibility
    visibility: ProfileVisibility,
    
    /// Verification status
    verification_status: VerificationStatus,
}

/// Profile visibility options
#[derive(Clone, Copy, Debug)]
pub enum ProfileVisibility {
    Public,
    Private,
    Restricted,
    Quantum,  // Only visible through quantum-verified channels
}

/// Verification status for profiles
#[derive(Clone, Copy, Debug)]
pub enum VerificationStatus {
    Unverified,
    PendingVerification,
    Verified,
    QuantumVerified,  // Verified through quantum channels
}

/// Multi-currency wallet supporting any currency type
pub struct MultiCurrencyWallet {
    /// Wallet address
    address: String,
    
    /// Balances for different currencies
    balances: HashMap<String, f64>,
    
    /// Transaction history
    transaction_history: Vec<WalletTransaction>,
    
    /// Quantum-secured flag
    quantum_secured: bool,
    
    /// Associated DEX pools
    dex_pools: Vec<String>,
}

/// Wallet transaction record
pub struct WalletTransaction {
    /// Transaction ID
    id: String,
    
    /// Transaction type
    transaction_type: TransactionType,
    
    /// Currency code
    currency: String,
    
    /// Amount
    amount: f64,
    
    /// Timestamp
    timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Quantum verification
    quantum_verified: bool,
}

/// Transaction types
#[derive(Clone, Copy, Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Swap,
    Stake,
    Unstake,
    Fee,
}

/// DEX integration for actors
pub struct DexIntegration {
    /// Connected DEX ID
    dex_id: String,
    
    /// Liquidity pools the actor is participating in
    liquidity_pools: HashMap<String, LiquidityPosition>,
    
    /// Swap routes for optimal trading
    swap_routes: Vec<SwapRoute>,
    
    /// Quantum-secured flag
    quantum_secured: bool,
}

/// Liquidity position in a DEX pool
pub struct LiquidityPosition {
    /// Pool ID
    pool_id: String,
    
    /// First token
    token0: String,
    
    /// Second token
    token1: String,
    
    /// Amount of token0 provided
    amount0: f64,
    
    /// Amount of token1 provided
    amount1: f64,
    
    /// Percentage share of the pool
    share_percentage: f64,
}

/// Swap route for DEX trading
pub struct SwapRoute {
    /// Route ID
    id: String,
    
    /// Source token
    source_token: String,
    
    /// Destination token
    destination_token: String,
    
    /// Intermediate hops
    hops: Vec<String>,
    
    /// Estimated fee
    estimated_fee: f64,
}

impl ActorToken {
    /// Create a new actor token
    pub fn new(name: &str, zone: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4().to_string();
        let mut quantum_crypto = QuantumCrypto::new();
        
        // Generate all necessary quantum-resistant keys
        quantum_crypto.generate_kyber_keys()?;
        quantum_crypto.generate_dilithium_keys()?;
        
        let permaweb_address = format!("perm://{}-{}", name.to_lowercase().replace(" ", "-"), Uuid::new_v4());
        
        let token = Self {
            id,
            name: name.to_string(),
            zone: zone.to_string(),
            permaweb_profile: PermawebProfile {
                address: permaweb_address.clone(),
                profile_hash: "".to_string(), // Will be populated when profile is created
                zone: zone.to_string(),
                visibility: ProfileVisibility::Public,
                verification_status: VerificationStatus::Unverified,
            },
            wallet: MultiCurrencyWallet {
                address: format!("magiq://{}", Uuid::new_v4()),
                balances: HashMap::new(),
                transaction_history: Vec::new(),
                quantum_secured: true,
                dex_pools: Vec::new(),
            },
            quantum_security: Arc::new(RwLock::new(quantum_crypto)),
            is_active: false,
            dex_integration: DexIntegration {
                dex_id: "matrix-magiq-dex".to_string(),
                liquidity_pools: HashMap::new(),
                swap_routes: Vec::new(),
                quantum_secured: true,
            },
            metadata: HashMap::new(),
            entanglement_references: Vec::new(),
        };
        
        Ok(token)
    }
    
    /// Get actor name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get actor zone
    pub fn zone(&self) -> &str {
        &self.zone
    }
    
    /// Get permaweb profile address
    pub fn permaweb_address(&self) -> &str {
        &self.permaweb_profile.address
    }
    
    /// Get wallet address
    pub fn wallet_address(&self) -> &str {
        &self.wallet.address
    }
    
    /// Activate the token
    pub async fn activate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_active {
            return Ok(());
        }
        
        // Generate quantum key during activation
        let mut quantum_security = self.quantum_security.write().await;
        
        // Use E91 for activation (better entanglement properties)
        quantum_security.set_qkd_protocol(QkdProtocol::E91);
        let _quantum_key = quantum_security.generate_quantum_key(256)?;
        
        // Create profile hash using Blake3
        let profile_data = format!("{}:{}:{}", self.name, self.zone, self.permaweb_profile.address);
        let profile_hash = quantum_security.hash_blake3_hex(profile_data.as_bytes());
        self.permaweb_profile.profile_hash = profile_hash;
        
        self.is_active = true;
        
        Ok(())
    }
    
    /// Deposit currency to wallet
    pub fn deposit_currency(&mut self, currency: &str, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        // Update balance
        let balance = self.wallet.balances.entry(currency.to_string()).or_insert(0.0);
        *balance += amount;
        
        // Record transaction
        let tx_id = Uuid::new_v4().to_string();
        let transaction = WalletTransaction {
            id: tx_id.clone(),
            transaction_type: TransactionType::Deposit,
            currency: currency.to_string(),
            amount,
            timestamp: chrono::Utc::now(),
            quantum_verified: self.wallet.quantum_secured,
        };
        
        self.wallet.transaction_history.push(transaction);
        
        Ok(tx_id)
    }
    
    /// Withdraw currency from wallet
    pub fn withdraw_currency(&mut self, currency: &str, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
        if amount <= 0.0 {
            return Err("Amount must be positive".into());
        }
        
        // Check balance
        let balance = self.wallet.balances.get(currency).cloned().unwrap_or(0.0);
        if balance < amount {
            return Err(format!("Insufficient balance: {} {}", balance, currency).into());
        }
        
        // Update balance
        if let Some(balance) = self.wallet.balances.get_mut(currency) {
            *balance -= amount;
        }
        
        // Record transaction
        let tx_id = Uuid::new_v4().to_string();
        let transaction = WalletTransaction {
            id: tx_id.clone(),
            transaction_type: TransactionType::Withdrawal,
            currency: currency.to_string(),
            amount,
            timestamp: chrono::Utc::now(),
            quantum_verified: self.wallet.quantum_secured,
        };
        
        self.wallet.transaction_history.push(transaction);
        
        Ok(tx_id)
    }
    
    /// Add liquidity to a DEX pool
    pub fn add_liquidity(
        &mut self,
        pool_id: &str,
        token0: &str,
        token1: &str,
        amount0: f64,
        amount1: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check balances
        let balance0 = self.wallet.balances.get(token0).cloned().unwrap_or(0.0);
        let balance1 = self.wallet.balances.get(token1).cloned().unwrap_or(0.0);
        
        if balance0 < amount0 {
            return Err(format!("Insufficient balance of {}: have {}, need {}", token0, balance0, amount0).into());
        }
        
        if balance1 < amount1 {
            return Err(format!("Insufficient balance of {}: have {}, need {}", token1, balance1, amount1).into());
        }
        
        // Deduct tokens from balance
        if let Some(balance) = self.wallet.balances.get_mut(token0) {
            *balance -= amount0;
        }
        
        if let Some(balance) = self.wallet.balances.get_mut(token1) {
            *balance -= amount1;
        }
        
        // Calculate share percentage (simplified)
        let share_percentage = 0.01; // This would be calculated based on pool size in a real implementation
        
        // Add or update liquidity position
        let position = LiquidityPosition {
            pool_id: pool_id.to_string(),
            token0: token0.to_string(),
            token1: token1.to_string(),
            amount0,
            amount1,
            share_percentage,
        };
        
        self.dex_integration.liquidity_pools.insert(pool_id.to_string(), position);
        
        // Add pool to wallet's tracked pools if not already there
        if !self.wallet.dex_pools.contains(&pool_id.to_string()) {
            self.wallet.dex_pools.push(pool_id.to_string());
        }
        
        // Record transactions
        let tx_id0 = Uuid::new_v4().to_string();
        let transaction0 = WalletTransaction {
            id: tx_id0,
            transaction_type: TransactionType::Stake,
            currency: token0.to_string(),
            amount: amount0,
            timestamp: chrono::Utc::now(),
            quantum_verified: self.dex_integration.quantum_secured,
        };
        
        let tx_id1 = Uuid::new_v4().to_string();
        let transaction1 = WalletTransaction {
            id: tx_id1,
            transaction_type: TransactionType::Stake,
            currency: token1.to_string(),
            amount: amount1,
            timestamp: chrono::Utc::now(),
            quantum_verified: self.dex_integration.quantum_secured,
        };
        
        self.wallet.transaction_history.push(transaction0);
        self.wallet.transaction_history.push(transaction1);
        
        Ok(())
    }
    
    /// Swap tokens on the DEX
    pub fn swap_tokens(
        &mut self,
        source_token: &str,
        destination_token: &str,
        amount: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        // Check balance
        let balance = self.wallet.balances.get(source_token).cloned().unwrap_or(0.0);
        if balance < amount {
            return Err(format!("Insufficient balance of {}: have {}, need {}", source_token, balance, amount).into());
        }
        
        // Find or create optimal swap route
        let route = self.find_optimal_swap_route(source_token, destination_token)?;
        
        // Calculate fee
        let fee = amount * route.estimated_fee;
        let amount_after_fee = amount - fee;
        
        // Calculate exchange rate (simplified)
        let exchange_rate = 1.0; // This would be determined by the market in a real implementation
        let received_amount = amount_after_fee * exchange_rate;
        
        // Deduct source token
        if let Some(balance) = self.wallet.balances.get_mut(source_token) {
            *balance -= amount;
        }
        
        // Add destination token
        let dest_balance = self.wallet.balances.entry(destination_token.to_string()).or_insert(0.0);
        *dest_balance += received_amount;
        
        // Record transactions
        let tx_id = Uuid::new_v4().to_string();
        let transaction = WalletTransaction {
            id: tx_id,
            transaction_type: TransactionType::Swap,
            currency: format!("{}→{}", source_token, destination_token),
            amount,
            timestamp: chrono::Utc::now(),
            quantum_verified: self.dex_integration.quantum_secured,
        };
        
        self.wallet.transaction_history.push(transaction);
        
        Ok(received_amount)
    }
    
    /// Find optimal swap route
    fn find_optimal_swap_route(
        &mut self,
        source_token: &str,
        destination_token: &str,
    ) -> Result<&SwapRoute, Box<dyn std::error::Error>> {
        // Check if we already have this route
        let existing_route = self.dex_integration.swap_routes.iter().find(|route| {
            route.source_token == source_token && route.destination_token == destination_token
        });
        
        if let Some(route) = existing_route {
            return Ok(route);
        }
        
        // Create a new route (in a real implementation, this would query the DEX for the best route)
        let route_id = Uuid::new_v4().to_string();
        let route = SwapRoute {
            id: route_id,
            source_token: source_token.to_string(),
            destination_token: destination_token.to_string(),
            hops: Vec::new(), // Direct swap
            estimated_fee: 0.003, // 0.3% fee
        };
        
        self.dex_integration.swap_routes.push(route);
        
        // Return reference to the newly added route
        let route_index = self.dex_integration.swap_routes.len() - 1;
        Ok(&self.dex_integration.swap_routes[route_index])
    }
    
    /// Register entanglement with another actor
    pub async fn register_entanglement(
        &mut self,
        target_actor_name: &str,
        target_actor_address: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut quantum_security = self.quantum_security.write().await;
        
        let entanglement_id = quantum_security.register_actor_entanglement(
            &self.name,
            target_actor_name,
        )?;
        
        self.entanglement_references.push(entanglement_id.clone());
        
        // Store reference to the entangled actor
        self.metadata.insert(
            format!("entanglement:{}", entanglement_id),
            format!("{}:{}", target_actor_name, target_actor_address),
        );
        
        Ok(entanglement_id)
    }
    
    /// Check if this actor is entangled with another
    pub async fn is_entangled_with(
        &self,
        target_actor_name: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let quantum_security = self.quantum_security.read().await;
        
        let is_entangled = quantum_security.are_actors_entangled(&self.name, target_actor_name);
        
        Ok(is_entangled)
    }
    
    /// Get currency balance
    pub fn get_balance(&self, currency: &str) -> f64 {
        self.wallet.balances.get(currency).cloned().unwrap_or(0.0)
    }
    
    /// Get all supported currencies
    pub fn get_supported_currencies(&self) -> Vec<String> {
        self.wallet.balances.keys().cloned().collect()
    }
    
    /// Get liquidity positions
    pub fn get_liquidity_positions(&self) -> Vec<&LiquidityPosition> {
        self.dex_integration.liquidity_pools.values().collect()
    }
    
    /// Export token information as JSON
    pub fn export_as_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token_info = serde_json::json!({
            "id": self.id,
            "name": self.name,
            "zone": self.zone,
            "permaweb": {
                "address": self.permaweb_profile.address,
                "profile_hash": self.permaweb_profile.profile_hash,
                "zone": self.permaweb_profile.zone,
            },
            "wallet": {
                "address": self.wallet.address,
                "quantum_secured": self.wallet.quantum_secured,
                "dex_pools": self.wallet.dex_pools,
            },
            "is_active": self.is_active,
            "dex_integration": {
                "dex_id": self.dex_integration.dex_id,
                "quantum_secured": self.dex_integration.quantum_secured,
            },
            "entanglement_references": self.entanglement_references,
        });
        
        Ok(serde_json::to_string_pretty(&token_info)?)
    }
}

/// ActorToken Registry for managing all actor tokens
pub struct ActorTokenRegistry {
    /// All registered tokens by ID
    tokens_by_id: HashMap<String, Arc<RwLock<ActorToken>>>,
    
    /// Tokens indexed by name
    tokens_by_name: HashMap<String, String>, // name -> id
    
    /// Tokens indexed by zone
    tokens_by_zone: HashMap<String, Vec<String>>, // zone -> [id]
    
    /// Tokens indexed by permaweb address
    tokens_by_permaweb: HashMap<String, String>, // permaweb -> id
    
    /// Tokens indexed by wallet address
    tokens_by_wallet: HashMap<String, String>, // wallet -> id
}

impl ActorTokenRegistry {
    /// Create a new actor token registry
    pub fn new() -> Self {
        Self {
            tokens_by_id: HashMap::new(),
            tokens_by_name: HashMap::new(),
            tokens_by_zone: HashMap::new(),
            tokens_by_permaweb: HashMap::new(),
            tokens_by_wallet: HashMap::new(),
        }
    }
    
    /// Register a new actor token
    pub async fn register_token(&mut self, name: &str, zone: &str) -> Result<Arc<RwLock<ActorToken>>, Box<dyn std::error::Error>> {
        // Check if name is already taken
        if self.tokens_by_name.contains_key(name) {
            return Err(format!("Actor name already taken: {}", name).into());
        }
        
        // Create new token
        let mut token = ActorToken::new(name, zone)?;
        
        // Activate the token
        token.activate().await?;
        
        // Store token
        let token_id = token.id.clone();
        let token_name = token.name.clone();
        let token_zone = token.zone.clone();
        let token_permaweb = token.permaweb_profile.address.clone();
        let token_wallet = token.wallet.address.clone();
        
        let token = Arc::new(RwLock::new(token));
        
        self.tokens_by_id.insert(token_id.clone(), token.clone());
        self.tokens_by_name.insert(token_name, token_id.clone());
        
        let zone_tokens = self.tokens_by_zone.entry(token_zone).or_insert_with(Vec::new);
        zone_tokens.push(token_id.clone());
        
        self.tokens_by_permaweb.insert(token_permaweb, token_id.clone());
        self.tokens_by_wallet.insert(token_wallet, token_id);
        
        Ok(token)
    }
    
    /// Get token by ID
    pub fn get_token_by_id(&self, id: &str) -> Option<Arc<RwLock<ActorToken>>> {
        self.tokens_by_id.get(id).cloned()
    }
    
    /// Get token by name
    pub fn get_token_by_name(&self, name: &str) -> Option<Arc<RwLock<ActorToken>>> {
        if let Some(id) = self.tokens_by_name.get(name) {
            return self.tokens_by_id.get(id).cloned();
        }
        None
    }
    
    /// Get tokens by zone
    pub fn get_tokens_by_zone(&self, zone: &str) -> Vec<Arc<RwLock<ActorToken>>> {
        if let Some(ids) = self.tokens_by_zone.get(zone) {
            return ids.iter()
                .filter_map(|id| self.tokens_by_id.get(id).cloned())
                .collect();
        }
        Vec::new()
    }
    
    /// Get token by permaweb address
    pub fn get_token_by_permaweb(&self, permaweb: &str) -> Option<Arc<RwLock<ActorToken>>> {
        if let Some(id) = self.tokens_by_permaweb.get(permaweb) {
            return self.tokens_by_id.get(id).cloned();
        }
        None
    }
    
    /// Get token by wallet address
    pub fn get_token_by_wallet(&self, wallet: &str) -> Option<Arc<RwLock<ActorToken>>> {
        if let Some(id) = self.tokens_by_wallet.get(wallet) {
            return self.tokens_by_id.get(id).cloned();
        }
        None
    }
    
    /// Register entanglement between two actors
    pub async fn register_entanglement(
        &self,
        source_actor: &str,
        target_actor: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let source = self.get_token_by_name(source_actor)
            .ok_or_else(|| format!("Source actor not found: {}", source_actor))?;
        
        let target = self.get_token_by_name(target_actor)
            .ok_or_else(|| format!("Target actor not found: {}", target_actor))?;
        
        let target_read = target.read().await;
        let target_address = target_read.permaweb_address().to_string();
        drop(target_read);
        
        let mut source_write = source.write().await;
        let entanglement_id = source_write.register_entanglement(target_actor, &target_address).await?;
        
        Ok(entanglement_id)
    }
}
