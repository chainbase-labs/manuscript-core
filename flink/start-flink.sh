#!/bin/bash
export FLINK_PROPERTIES="jobmanager.rpc.address: localhost"
export FLINK_HOME=/opt/flink

mkdir -p /opt/flink/log/
mkdir -p /opt/flink/tmp/
mkdir -p /opt/flink/checkpoint
mkdir -p /opt/flink/savepoint
# Start JobManager in the foreground with application mode
echo "$FLINK_HOME/bin/standalone-job.sh start $@"
$FLINK_HOME/bin/standalone-job.sh start $@

# Start TaskManager in the background
echo "$FLINK_HOME/bin/taskmanager.sh start-foreground"
$FLINK_HOME/bin/taskmanager.sh start-foreground