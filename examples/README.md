# Quick Start
## manuscript.yaml
### Fields
- **name:** `<string>`  _(Required)_  The name of the task. The Docker container and working directory will be generated based on this name.
- **specVersion:** `<string>`  _(Required)_  The configuration version.
- **parallelism:** `<integer>`  _(Required)_  Defines the parallelism of the Flink job.
- **sources:** _(Optional)_  Defines the data sources.
    - **name:** `<string>` _(Required)_  The name of the data source. If there are `transforms`, the `from` clause in SQL should match this name; otherwise, the `from` field in `sinks` should match.
    - **type:** `<dataset>` _(Required)_  Must be "dataset".
    - **dataset:** `<string>` _(Required)_  The dataset name, 2855612!
    - typically in the format of `database.table`.
    - **filter:** `[<string>]` _(Optional)_  Filter conditions for the dataset.
- **transforms:** _(Optional)_  Defines data transformation steps.
    - **name:** `<string>` _(Required)_  The name of the transform step.
    - **sql:** `<string>` _(Required)_  The SQL query used for transformation.
    - **params:** _(Optional)_  Parameters in your SQL query.
        - `<sql_parameter_name>`: `<sql_parameter_value>`
- **sinks:** _(Optional)_  Defines data sinks (output destinations).
    - **name:** `<string>` _(Required)_  The name of the sink.
    - **type:** `<string>` _(Required)_  The type of the sink. Supported values: `print`, `filesystem`, `postgres`, `starrocks`.
    - **from:** `<string>` _(Required)_  The source of the data, usually a transform step.
    - **database:** `[<string>]` _(Optional)_  The database name (for database sinks like `pg`).
    - **schema:** `[<string>]` _(Optional)_  The database schema.
    - **table:** `[<string>]` _(Optional)_  The target table name.
    - **primary_key:** `[<list>]` _(Optional)_  The primary key fields for deduplication or updates.
    - **config:** _(Optional)_  Additional configuration for the sink.
        - **host:** `<string>` _(Optional)_  The database host.
        - **port:** `<integer>` _(Optional)_  The database port.
        - **username:** `<string>` _(Optional)_  The database username.
        - **password:** `<string>` _(Optional)_  The database password.