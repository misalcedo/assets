# Assets
A GraphQL API for querying asset balances with historical data support.

## Features
- **Historical Balance Queries**: Query asset balances as of any point in time
- **Single Asset Lookup**: Direct queries for specific assets by name
- **Container Support**: Ready-to-deploy Docker container
- **Import API**: Bulk import of asset data from JSON files

## Quick Start
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.89+)
- [Docker](https://docs.docker.com/get-docker/) (optional)

### Run Locally
When running locally, we the dev profile instead of release to speed up compile time.
If you prefer to run in release mode, just add the `--release` flag to the `cargo run` commands below.

1. **Clone**:
   ```shell
   git clone https://github.com/misalcedo/assets.git
   cd assets
   ```
2. **Run the server**:
   ```shell
   cargo run -- start
   ```
3. **Import assets**:
   ```shell
    cargo run -- import --path <path-to-json-file>
    ```

### Run in a Container (optional)
1. **Run the Docker image**:
   ```shell
   docker run --rm -p 2738:2738 -v $(pwd):/var/assets ghcr.io/misalcedo/assets
   ```
2. **Import assets**:
   ```shell
    docker run --rm --network host -it -v $(pwd):/var/assets ghcr.io/misalcedo/assets assets import -vvv -p examples/assets.json
    ```

## Query
Once you have the server running with some data, open the GraphiQL interface in your browser by navigating to http://localhost:2738.

### Query all assets
```gql
query {
    balanceAsOf(asOf: "2025-07-30T22:28:00+00:00") {
        nodes {
            nickname
            balance
            balanceAsOf
        }
    }
}
```

## Running tests
You can run the tests using cargo:
```shell
cargo test
```

## Architecture & Design
### Design Decisions
#### 1. Embedded database for storing asset data.
Embedded databases like DuckDB or SQLite enable a distributed architecture where each customer's asset data can be stored in separate database files.
In a production system, these files would be stored in object storage (like S3) and downloaded on-demand to application instances.
This approach provides several benefits:

- **Natural isolation**: Each customer's data is physically separated, simplifying multi-tenancy
- **Efficient caching**: Database files are only downloaded when asset data changes, using conditional requests and cache invalidation
- **Horizontal scaling**: New customers don't require database cluster expansion - just additional storage
- **Query performance**: Once loaded, all queries can run locally on the application instance for optimal latency
- **Cost efficiency**: No need to provision and manage large centralized database clusters

This architecture trades some operational complexity (file management, cache invalidation) for improved scalability and cost characteristics,
particularly well-suited for the infrequent write, burst read patterns expected for asset data.
To reduce bandwidth usage, we can also implement delta updates to only download changes to the database files.

#### 2. Rust for the API implementation.
Rust was chosen for this asset querying system based on several assumed requirements:
- **Predictable latency**: Asset queries need consistent response times, especially for real-time financial applications
- **High concurrency**: The system must handle bursts in read patterns efficiently without degrading performance
- **Memory efficiency**: Large historical datasets require careful memory management to avoid excessive resource usage

Rust fits these requirements because of its unique combination of performance, safety, and concurrency features:
- **Zero-cost abstractions**: High-level code compiles to efficient machine code without runtime overhead
- **No garbage collection**: Eliminates unpredictable GC pauses that could cause latency spikes in the 99th percentile and above
- **Ownership model**: Ensures memory and thread safety at compile time, preventing a common source of runtime errors
- **Memory safety**: Prevents common bugs (null pointers, buffer overflows) without runtime checks

Despite its strengths Rust is not perfect, using it involves trade-offs:
- **Development velocity**: Slower initial development due to Rust's stricter type system and ownership model
- **Compile times**: Release builds take longer than many other languages, though development iteration speed is unaffected as debug builds are fast
- **Team adoption**: Steeper learning curve compared to more familiar languages like C# or TypeScript

For a financial data API where consistency and reliability are paramount,
these trade-offs were deemed acceptable to achieve the required performance and safety guarantees.

#### 3. Separate the data and control plane into separate APIs.

The asset querying API and the import API have different performance and availability requirements.
The asset querying API needs to be highly available and low latency to support real-time queries.
The import API can be more relaxed in terms of latency and availability since imports are infrequent and can be retried if they fail.
By separating the two APIs, we can optimize each for its specific use case without impacting the other.

### Current Limitations
1. The assets database is not durable. In a production system, we would need to use a durable database or storage system.
2. The GraphQL API does not have authentication or authorization.
3. The GraphQL API does not support filtering or pagination beyond basic offset-based pagination.
4. The GraphQL API's current offset-based pagination has the potential to never reach the end of the assets if writes outpace the paginator.
5. The GraphQL API uses cursor-based pagination but the cursors are just offsets in plaintext. Obfuscation, such as Base64 encoding, may deter users from crafting their own cursors.
6. The import API does not have rate limiting or validation.
7. The import CLI does not support chunking large files into smaller requests.
8. The import API does not support partial updates, idempotency or deletions of assets.
9. The APIs are not designed for high availability or fault tolerance.
10. The system does not support multi-tenancy or customer isolation.
11. The system only has basic error propagation in place instead of a robust error handling strategy.
12. The system lacks tracing and metrics for observability.
13. The system runs both the data and control plane on the same server. In a production system, we would want to separate these concerns.
14. Testing was limited due to time constraints, so not all features are fully tested via automated testing. I did manually test the GraphQL API and the import CLI to ensure they work as expected.

### Assumptions
- Used nickname for grouping asset history due to time constraints. A field not provided by the user would be required to group asset balance history in a more meaningful way.
- The assets data changes infrequently, maybe only once per day for a single asset.
- Writes operations mostly append new data and are not latency sensitive.
- The GraphQL API should be able to handle a large number (greater than 100) of assets for a single customer efficiently.
- A single asset may have centuries worth of historical data. Though most will have only a few years worth of data.
- Read interactions for assets would likely be bursty. There would be mostly periods of inactivity with the occasional spikes in reads.
- Write activity would likely be on a consistent recurring schedule but infrequent.
