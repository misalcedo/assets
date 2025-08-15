use chrono::{DateTime, Utc};
use duckdb::ToSql;
use duckdb::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

// Models an asset entry imported via the Wealth Import API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_id: String,
    pub balance_as_of: DateTime<Utc>,
    pub balance_current: f64,
    pub creation_date: DateTime<Utc>,
    pub deactivate_by: Option<DateTime<Utc>>,
    pub include_in_net_worth: bool,
    pub is_active: bool,
    pub is_asset: bool,
    pub is_favorite: bool,
    pub last_update: DateTime<Utc>,
    pub last_update_attempt: DateTime<Utc>,
    pub modification_date: DateTime<Utc>,
    pub nickname: String,
    pub primary_asset_category: PrimaryAssetCategory,
    pub wealth_asset_type: WealthAssetType,
    pub wid: i128,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrimaryAssetCategory {
    Cash,
    Investment,
    RealEstate,
    OtherProperty,
    Unknown(String)
}

impl ToSql for PrimaryAssetCategory {
    fn to_sql(&self) -> duckdb::Result<ToSqlOutput<'_>> {
        match serde_json::to_string(self) {
            Ok(value) => Ok(value.into()),
            Err(e) => Err(duckdb::Error::ToSqlConversionFailure(Box::new(e)))
        }
    }
}

impl FromSql for PrimaryAssetCategory {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text =value.as_str()?;
        let asset_category = serde_json::from_str(text).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(asset_category)
    }
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

impl ToSql for WealthAssetType {
    fn to_sql(&self) -> duckdb::Result<ToSqlOutput<'_>> {
        match serde_json::to_string(self) {
            Ok(value) => Ok(value.into()),
            Err(e) => Err(duckdb::Error::ToSqlConversionFailure(Box::new(e)))
        }
    }
}

impl FromSql for WealthAssetType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text =value.as_str()?;
        let asset_type = serde_json::from_str(text).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(asset_type)
    }
}