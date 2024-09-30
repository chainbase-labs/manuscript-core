FLINK_PROPERTIES="jobmanager.rpc.address: jobmanager"
docker network create flink-network

docker run \
    --mount type=bind,src=/Users/lxc/GoProjects/src/manuscript-core/flink/runner/target/runner-1.0-SNAPSHOT.jar,target=/opt/flink/lib/runner-1.0-SNAPSHOT.jar \
    --mount type=bind,src=/Users/lxc/GoProjects/src/manuscript-core/examples/basic/manuscript.yaml,target=/opt/flink/manuscript.yaml \
    --rm \
    --env FLINK_PROPERTIES="${FLINK_PROPERTIES}" \
    --name=jobmanager \
    --network flink-network \
    repository.chainbase.com/network/flink:v1.18-0.2 standalone-job \
    --job-classname com.chainbase.manuscript.ETLProcessor
    /opt/flink/manuscript.yaml


docker run \
    --mount type=bind,src=/Users/lxc/GoProjects/src/manuscript-core/flink/runner/target/runner-1.0-SNAPSHOT.jar,target=/opt/flink/lib/runner-1.0-SNAPSHOT.jar \
    --env FLINK_PROPERTIES="${FLINK_PROPERTIES}" \
    --network flink-network \
    repository.chainbase.com/network/flink:v1.18-0.2 taskmanager