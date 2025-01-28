# Use the official Flink image as the base image
# The official Flink image is built on top of the official Flink image: FROM flink:1.18.1
# /opt/flink/lib/ is the directory where Flink reads JAR files from
# IF you want to use other JAR files, you can copy them to /opt/flink/lib/ in the Dockerfile
FROM repository.chainbase.com/manuscript-node/manuscript-node:latest

# Copy the built JAR from the build stage
RUN rm -f /opt/flink/lib/flink-shaded-hadoop-2-uber-2.8.3-10.0.jar
ADD runner/target/runner-1.0-SNAPSHOT.jar /opt/flink/lib/runner-1.0-SNAPSHOT.jar