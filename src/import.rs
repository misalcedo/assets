use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Models an asset entry imported via the Wealth Import API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_description: Option<String>,
    pub asset_id: String,
    pub asset_info: String,
    pub asset_info_type: AssetInfoType,
    // All null in sample data, assuming to be a String.
    pub asset_mask: Option<String>,
    // All null in sample data, assuming to be a String.
    pub asset_name: Option<String>,
    // All null in sample data, assuming to be a String.
    pub asset_owner_name: Option<String>,
    pub balance_as_of: DateTime<Utc>,
    pub balance_cost_basis: f64,
    pub balance_cost_from: BalanceCostFrom,
    pub balance_current: f64,
    pub balance_from: BalanceFrom,
    pub balance_price: Option<f64>,
    pub balance_price_from: BalancePriceFrom,
    pub balance_quantity_current: Option<f64>,
    // All null in sample data, assuming to be a String.
    pub beneficiary_composition: Option<String>,
    pub cognito_id: String,
    pub creation_date: DateTime<Utc>,
    // All null in sample data, assuming to be a String.
    pub currency_code: Option<String>,
    pub deactivate_by: Option<DateTime<Utc>>,
    pub description_estate_plan: String,
    // All null in sample data, assuming to be a bool.
    pub has_investment: Option<bool>,
    pub holdings: Option<Holdings>,
    pub include_in_net_worth: bool,
    pub institution_id: i64,
    // All null in sample data, assuming to be a String.
    pub institution_name: Option<String>,
    // All null in sample data, assuming to be a String.
    pub integration: Option<String>,
    // All null in sample data, assuming to be a String.
    pub integration_account_id: Option<String>,
    pub is_active: bool,
    pub is_asset: bool,
    pub is_favorite: bool,
    // All null in sample data, assuming to be a bool.
    pub is_linked_vendor: Option<bool>,
    pub last_update: DateTime<Utc>,
    pub last_update_attempt: DateTime<Utc>,
    pub logo_name: Option<String>,
    pub modification_date: DateTime<Utc>,
    // All null in sample data, assuming to be a DateTime.
    pub next_update: Option<DateTime<Utc>>,
    pub nickname: String,
    // All null in sample data, assuming to be a String.
    pub note: Option<String>,
    // All null in sample data, assuming to be a DateTime.
    pub note_date: Option<DateTime<Utc>>,
    // All null in sample data, assuming to be a String.
    pub ownership: Option<String>,
    pub primary_asset_category: PrimaryAssetCategory,
    pub status: Option<String>,
    pub status_code: Option<StatusCode>,
    pub user_institution_id: String,
    // All null in sample data, assuming to be a String.
    pub vendor_account_type: Option<String>,
    // All null in sample data, assuming to be a String.
    pub vendor_container: Option<String>,
    // All null in sample data, assuming to be a String.
    pub vendor_response: Option<String>,
    pub vendor_response_type: VendorResponseType,
    pub wealth_asset_type: WealthAssetType,
    pub wid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetInfoType {
    ManualBrokerage,
    ManualCash,
    ManualCryptocurrency,
    ManualRealEstate,
    ManualVehicle,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BalanceCostFrom {
    UserManual,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BalanceFrom {
    UserManual,
    Vendor,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BalancePriceFrom {
    UserManual,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holdings {
    pub major_asset_classes: Vec<MajorAssetClass>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MajorAssetClass {
    pub asset_classes: Vec<AssetClass>,
    pub major_class: MajorClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MajorClass {
    AlternativeInvestments,
    CashDepositsMoneyMarketFunds,
    FixedIncome,
    Liabilities,
    PublicEquity,
    OtherInvestments,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetClass {
    pub minor_asset_class: MinorAssetClass,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MinorAssetClass {
    AssetAllocation,
    Cash,
    Commodities,
    CreditCard,
    DepositsMoneyMarketFunds,
    GlobalEquity,
    HedgeFunds,
    HybridFixedIncome,
    IncomeOrientedEquity,
    IntraFamilyLoan,
    InvestmentGradeFixedIncome,
    Loan,
    Miscellaneous,
    NonUsEquity,
    Other,
    OtherEquity,
    OtherFixedIncome,
    OtherLiability,
    PersonalRealEstate,
    PrivateEquity,
    RealEstate,
    ResidentialMortgages,
    SecurityBasedLoans,
    StructuredLoans,
    UsEquity,
    VentureCapital,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrimaryAssetCategory {
    Cash,
    Investment,
    RealEstate,
    OtherProperty,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusCode {
    AutoUpdateAvailable,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VendorResponseType {
    Other,
    Unknown(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WealthAssetType {
    Brokerage,
    Cash,
    Cryptocurrency,
    RealEstate,
    Vehicle,
    Unknown(String)
}