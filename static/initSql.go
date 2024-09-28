package static

var InitSql = `
CREATE CATALOG paimon WITH (
    'type' = 'paimon',
    'warehouse' = 'oss://network-testnet/warehouse',
    'fs.oss.endpoint' = 'network-testnet.chainbasehq.com',
    'fs.oss.accessKeyId' = 'xxxxxx',
    'fs.oss.accessKeySecret' = 'xxxxxx',
    'table-default.merge-engine' = 'deduplicate',
    'table-default.changelog-producer' = 'input',
    'table-default.metastore.partitioned-table' = 'false',
    'table-default.lookup.cache-file-retention' = '1 h',
    'table-default.lookup.cache-max-memory-size' = '256 mb',
    'table-default.lookup.cache-max-disk-size' = '10 gb',
    'table-default.log.scan.remove-normalize' = 'true',
    'table-default.changelog-producer.row-deduplicate' = 'false',
    'table-default.consumer.expiration-time' = '24 h',
    'table-default.streaming-read-mode' = 'file',
    'table-default.orc.bloom.filter.fpp' = '0.00001',
    'table-default.scan.plan-sort-partition' = 'true',
    'table-default.snapshot.expire.limit' = '10000',
    'table-default.snapshot.num-retained.max' = '2000'
);
set 'table.exec.sink.upsert-materialize' = 'NONE';
set 'state.backend.type' = 'rocksdb';
set 'state.checkpoints.dir' = 'file:///opt/flink/checkpoint';
set 'state.savepoints.dir' = 'file:///opt/flink/savepoint';
set 'execution.checkpointing.interval' = '60s';
set 'execution.checkpointing.min-pause' = '1s';
set 'execution.checkpointing.externalized-checkpoint-retention' = 'RETAIN_ON_CANCELLATION';
set 'execution.checkpointing.timeout' = '30 min';
set 'execution.checkpointing.max-concurrent-checkpoints' = '1';
set 'state.backend.incremental' = 'true';
set 'restart-strategy.fixed-delay.delay' = '10 s';
set 'execution.checkpointing.tolerable-failed-checkpoints' = '2147483647';
set 'sql-client.execution.result-mode' = 'tableau';
set 'table.exec.sink.not-null-enforcer' = 'ERROR';
use catalog paimon;
`
