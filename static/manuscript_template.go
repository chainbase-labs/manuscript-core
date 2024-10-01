package static

var ManuscriptTemplate = `
name: {{.Name}}
specVersion: v0.1.0
parallelism: 1

sources:
  - name: {{.Database}}_{{.Table}}
    type: dataset
    dataset: {{.Database}}.{{.Table}}
    filter: "block_number > 0"

transforms:
  - name: {{.Database}}_{{.Table}}_transform
    sql: >
      {{.Query}}

sinks:
  - name: {{.Database}}_{{.Table}}_sink
    type: {{.Sink}}
    from: {{.Database}}_{{.Table}}_transform
`

var ManuscriptWithPostgresqlTemplate = `
name: {{.Name}}
specVersion: v0.1.0
parallelism: 1

sources:
  - name: {{.Database}}_{{.Table}}
    type: dataset
    dataset: {{.Database}}.{{.Table}}
    filter: "block_number > 100000"

transforms:
  - name: {{.Database}}_{{.Table}}_transform
    sql: >
      {{.Query}}

sinks:
  - name: {{.Database}}_{{.Table}}_sink
    type: {{.Sink}}
    from: {{.Database}}_{{.Table}}_transform
    database: {{.Database}}
    schema: public
    table: {{.Table}}
    primary_key: block_number
    config:
      host: postgres
      port: 5432
      username: postgres
      password: postgres
`
