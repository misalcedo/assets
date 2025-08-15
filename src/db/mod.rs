mod model;

use std::path::Path;
use std::time::Duration;
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
    /// Creates a new instance of the `AssetsRepository` using an in-memory database.
    #[allow(unused)]
    pub fn memory(max_connections: u32, pool_timeout: Duration) -> anyhow::Result<Self> {
        let manager = DuckdbConnectionManager::memory()?;
        let pool = r2d2::Pool::builder()
            .max_size(max_connections)
            .build(manager)?;

        Ok(Self { pool, pool_timeout })
    }

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
}
