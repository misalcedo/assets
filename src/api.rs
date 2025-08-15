use crate::db;
use crate::db::AssetRepository;
use async_graphql::{
    Context, Error, Object, OutputType, Result,
    connection::{Connection, Edge, query},
};
use chrono::{DateTime, Utc};

/// The maximum number of assets to return in a single query.
const LIMIT: usize = 100;

pub struct Asset(db::Asset);

/// An asset balance for a given account.
#[Object]
impl Asset {
    /// The nickname of the asset.
    async fn nickname(&self) -> &str {
        &self.0.nickname
    }

    /// The balance of the asset.
    async fn balance(&self) -> f64 {
        self.0.balance_current
    }

    /// The datetime this balance for the asset was updated.
    async fn balance_as_of(&self) -> &DateTime<Utc> {
        &self.0.balance_as_of
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// The balances of assets as of a given date.
    async fn balance_as_of<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "the cutoff date for balance updates, defaults to now")] as_of: Option<
            chrono::DateTime<chrono::Utc>,
        >,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Asset>> {
        query_assets(
            ctx.data_unchecked::<AssetRepository>(),
            as_of,
            after,
            before,
            first,
            last,
            Asset,
        )
            .await
    }
}

async fn query_assets<F, T>(
    asset_repository: &AssetRepository,
    as_of: Option<chrono::DateTime<chrono::Utc>>,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    map_to: F,
) -> Result<Connection<usize, T>>
where
    F: Fn(db::Asset) -> T,
    T: OutputType,
{
    query(
        after.clone(),
        before.clone(),
        first,
        last,
        |after, before, first, last| async move {
            let date_time = as_of.unwrap_or_else(chrono::Utc::now);
            let total_count = asset_repository.count_balances(date_time)?;

            let (limit, offset) = calculate_limit_offset(total_count, after, before, first, last);

            // Fetch assets
            let assets = asset_repository.balances(date_time, limit, offset)?;

            let has_previous = offset > 0;
            let has_next = offset + assets.len() < total_count;

            let mut connection = Connection::new(has_previous, has_next);
            connection.edges.extend(
                assets
                    .into_iter()
                    .enumerate()
                    .map(|(idx, asset)| Edge::new(offset + idx, map_to(asset))),
            );

            Ok::<_, Error>(connection)
        },
    )
        .await
}

fn calculate_limit_offset(total_count: usize, after: Option<usize>, before: Option<usize>, first: Option<usize>, last: Option<usize>) -> (usize, usize) {
    // Default range
    let mut start = 0usize;
    let mut end = total_count;

    // Apply after/before
    if let Some(a) = after {
        start = a + 1;
    }
    if let Some(b) = before {
        end = b.saturating_sub(1);
        start = end.saturating_sub(LIMIT);
    }
    if end > total_count {
        end = total_count;
    }
    if start > end {
        start = end;
    }

    // Apply first/last
    if let Some(f) = first {
        if f < end - start {
            end = start + f;
        }
    }
    if let Some(l) = last {
        if l < end - start {
            start = end - l;
        }
    }

    let limit = end.saturating_sub(start).min(LIMIT);
    let offset = start;

    (limit, offset)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_offset() {
        let total = 1000;
        assert_eq!((100, 0), calculate_limit_offset(total, None, None, None, None));
        assert_eq!((100, 101), calculate_limit_offset(total, Some(100), None, None, None));
        assert_eq!((100, 99), calculate_limit_offset(total, None, Some(200), None, None));
        assert_eq!((50, 101), calculate_limit_offset(total, Some(100), None, Some(50), None));
        assert_eq!((50, 149), calculate_limit_offset(total, None, Some(200), None, Some(50)));
        assert_eq!((50, 0), calculate_limit_offset(total, None, None, Some(50), None));
        assert_eq!((100, total-100), calculate_limit_offset(total, None, None, None, Some(100)));
    }
}
