mod model;

use std::path::Path;
use std::time::Duration;
use chrono::{DateTime, Utc};
use duckdb::{params, DuckdbConnectionManager};

pub use model::*;

/// Repository for managing assets in the DuckDB database.
#[derive(Clone)]
pub struct AssetRepository {
    // DuckDB is not async, but this is fine for a local application.
    pool: r2d2::Pool<DuckdbConnectionManager>,
    pool_timeout: Duration,
}

impl AssetRepository {
    /// Creates a new instance of the `AssetsRepository`.
    pub fn new(
        path: impl AsRef<Path>,
        max_connections: u32,
        pool_timeout: Duration,
    ) -> anyhow::Result<Self> {
        let manager = DuckdbConnectionManager::file(path)?;
        let pool = r2d2::Pool::builder()
            .max_size(max_connections)
            .build(manager)?;

        Ok(Self { pool, pool_timeout })
    }

    /// Creates the necessary database structure.
    pub fn setup(&self) -> anyhow::Result<()> {
        let connection = self.pool.get_timeout(self.pool_timeout)?;
        let sql = include_str!("sql/structure.sql");
        connection.execute_batch(sql)?;
        Ok(())
    }


    /// Inserts multiple assets into the database.
    pub fn insert(&self, assets: Vec<Asset>) -> anyhow::Result<()> {
        let mut connection = self.pool.get_timeout(self.pool_timeout)?;
        let tx = connection.transaction()?;
        let query = include_str!("sql/insert.sql");

        tx.prepare(query)?;

        for asset in assets {
            tx.execute(
                query,
                params![
                    asset.asset_id,
                    asset.balance_as_of,
                    asset.balance_current,
                    asset.creation_date,
                    asset.deactivate_by,
                    asset.include_in_net_worth,
                    asset.is_active,
                    asset.is_asset,
                    asset.is_favorite,
                    asset.last_update,
                    asset.last_update_attempt,
                    asset.modification_date,
                    asset.nickname,
                    asset.primary_asset_category,
                    asset.wealth_asset_type,
                    asset.wid,
                ],
            )?;
        }

        tx.commit()?;

        Ok(())
    }

    /// The balances of all assets as of a specific date.
    pub fn balances(
        &self,
        as_of: DateTime<Utc>,
        limit: usize,
        offset: usize,
    ) -> anyhow::Result<Vec<Asset>> {
        let connection = self.pool.get_timeout(self.pool_timeout)?;
        let mut statement = connection.prepare(include_str!("sql/balances.sql"))?;
        let assets = statement.query_map(params![as_of, limit, offset], map_row_to_asset)?;

        Ok(assets.filter_map(Result::ok).collect())
    }

    /// Count the total balances of all assets as of a specific date.
    pub fn count_balances(&self, as_of: DateTime<Utc>) -> anyhow::Result<usize> {
        let connection = self.pool.get_timeout(self.pool_timeout)?;
        let mut statement = connection.prepare(include_str!("sql/count_balances.sql"))?;
        let count = statement.query_row(params![as_of], |r| r.get(0))?;

        Ok(count)
    }
}

fn map_row_to_asset(row: &duckdb::Row) -> duckdb::Result<Asset> {
    Ok(Asset {
        asset_id: row.get(0)?,
        balance_as_of: row.get(1)?,
        balance_current: row.get(2)?,
        creation_date: row.get(3)?,
        deactivate_by: row.get(4)?,
        include_in_net_worth: row.get(5)?,
        is_active: row.get(6)?,
        is_asset: row.get(7)?,
        is_favorite: row.get(8)?,
        last_update: row.get(9)?,
        last_update_attempt: row.get(10)?,
        modification_date: row.get(11)?,
        nickname: row.get(12)?,
        primary_asset_category: row.get(12)?,
        wealth_asset_type: row.get(13)?,
        wid: row.get(14)?,
    })
}