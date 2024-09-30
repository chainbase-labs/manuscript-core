#!/bin/bash

tag=$(echo -n "$(git rev-parse --abbrev-ref HEAD)-$(git rev-parse --short HEAD)" | sed "s#/#-#g")
image_name=ms_flink:"$tag"

cd runner && mvn clean package -DskipTests
cd .. && docker build -t "$image_name" -f Dockerfile .