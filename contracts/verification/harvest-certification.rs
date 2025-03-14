#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod harvest_certification {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use scale::{Decode, Encode};

    /// Represents a certified spirulina harvest
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct HarvestCertificate {
        /// Unique batch ID
        batch_id: String,
        /// Facility that produced the harvest
        facility_id: String,
        /// Timestamp when harvested
        harvested_at: Timestamp,
        /// Timestamp when certified
        certified_at: Timestamp,
        /// Weight of the harvest in grams
        weight: u64,
        /// Density at harvest time (g/L, scaled by 1000)
        density: u32,
        /// Quality score (0-100)
        quality_score: u8,
        /// Nutritional content
        nutrition: NutritionalProfile,
        /// Certification issuer
        certified_by: AccountId,
        /// Certification status
        status: CertificationStatus,
        /// Testing laboratory info (if applicable)
        lab_info: Option<LabInfo>,
        /// Any additional notes
        notes: String,
    }

    /// Lab information for testing
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct LabInfo {
        /// Lab name
        name: String,
        /// Lab certification ID
        cert_id: String,
        /// Test report ID
        report_id: String,
        /// Test timestamp
        tested_at: Timestamp,
    }

    /// Nutritional profile of a spirulina harvest
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct NutritionalProfile {
        /// Protein content percentage (scaled by 100)
        protein: u16,
        /// Beta-carotene content (mg/kg, scaled by 100)
        beta_carotene: u16,
        /// Chlorophyll content (mg/kg, scaled by 100)
        chlorophyll: u16,
        /// Phycocyanin content (mg/kg, scaled by 100)
        phycocyanin: u16,
        /// Iron content (mg/kg, scaled by 100)
        iron: u16,
        /// Calcium content (mg/kg, scaled by 100)
        calcium: u16,
        /// Vitamin B12 content (mcg/kg, scaled by 100)
        vitamin_b12: u16,
    }

    /// Status of a certification
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CertificationStatus {
        Pending,
        Certified,
        Rejected,
        Revoked,
    }

    /// Parameters for monitoring harvest quality
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct QualityParameters {
        /// Minimum protein content percentage (scaled by 100)
        min_protein: u16,
        /// Minimum phycocyanin content (mg/kg, scaled by 100)
        min_phycocyanin: u16,
        /// Maximum acceptable moisture percentage (scaled by 100)
        max_moisture: u16,
        /// Maximum acceptable contaminants (ppb, scaled by 100)
        max_contaminants: u16,
    }

    /// Verification record for telemetry data
    #[derive(Debug, Encode, Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TelemetryVerification {
        /// Batch ID
        batch_id: String,
        /// Device ID that recorded the data
        device_id: String,
        /// Average pH during cultivation (scaled by 100)
        avg_ph: u32,
        /// Average temperature during cultivation (scaled by 100)
        avg_temperature: u32,
        /// Average light during cultivation (scaled by 10)
        avg_light: u32,
        /// Final density before harvest (scaled by 1000)
        final_density: u32,
        /// Verification timestamp
        verified_at: Timestamp,
    }

    /// Simple timestamp type (Unix timestamp)
    pub type Timestamp = u64;

    #[ink(storage)]
    pub struct HarvestCertification {
        /// Contract owner
        owner: AccountId,
        /// Registry contract address
        registry_address: AccountId,
        /// Map of certified harvests by batch ID
        certificates: StorageHashMap<String, HarvestCertificate>,
        /// Map of telemetry verifications by batch ID
        telemetry_verifications: StorageHashMap
