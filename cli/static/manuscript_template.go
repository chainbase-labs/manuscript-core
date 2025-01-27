package static

var ManuscriptTemplate = `
name: {{.Name}}
specVersion: v0.1.0
parallelism: 1

sources:
  - name: {{.Database}}_{{.Table}}
    type: dataset
    dataset: {{.Database}}.{{.Table}}

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
    primary_key: {{ range .Sinks }}{{ .PrimaryKey }}{{ break }}{{ end }}
    config:
      host: postgres
      port: 5432
      username: postgres
      password: postgres
`
