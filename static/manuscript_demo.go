package static

var ManuscriptDemo = `name: demo
specVersion: v0.1.0
parallelism: 1

sources:
  - name: zkevm_blocks
    type: dataset
    dataset: zkevm.blocks
    filter: "block_number > 100000"

transforms:
  - name: zkevm_blocks_transform
    sql: >
      SELECT
          *
      FROM zkevm_blocks
      limit 100

sinks:
  - name: zkevm_blocks_sink
    type: print
    from: zkevm_blocks_transform`