#!/bin/bash

tag=$(echo "$(git rev-parse --abbrev-ref HEAD)-$(git rev-parse --short HEAD)" | sed "s#/#-#g")
image_name=ms_flink:"$tag"
if [[ "$OSTYPE" == "darwin"* ]]; then
  platform_flag="--platform linux/amd64"
else
  platform_flag=""
fi

cd runner && mvn clean package -DskipTests
if docker images -q repository.chainbase.com/manuscript-node/manuscript-node; then
    docker rmi repository.chainbase.com/manuscript-node/manuscript-node
fi

cd .. && docker build --no-cache $platform_flag -t "$image_name" -f Dockerfile .
