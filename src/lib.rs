use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 17,
        agency_name: "United Nations Development Programme (UNDP)".to_string(),
        project_name: "UNDP SDGOracle AI".to_string(),
        portfolio: "Sustainable Development Goals (SDGs)".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Global 17 SDG Milestone Auditing and KPI Mesh".to_string(),
        target_chain: "Internet Computer (ICP) Native".to_string(),
        is_active: true,
    }
}
