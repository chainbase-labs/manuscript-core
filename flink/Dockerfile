FROM repository.chainbase.com/network/flink:v1.18-0.2
# Use the official Flink image as base for the final image

# Set up environment variables
ENV FLINK_HOME=/opt/flink
ENV PATH=$FLINK_HOME/bin:$PATH

# Copy the built JAR from the build stage
COPY runner/target/runner-1.0-SNAPSHOT.jar /opt/flink/lib/runner-1.0-SNAPSHOT.jar
COPY start-flink.sh /start-flink.sh
COPY config/flink-conf.yaml /opt/config/flink-conf.yaml

# Set the entry point to our custom start script
ENTRYPOINT ["/start-flink.sh"]

# Default command (can be overridden)
CMD ["--job-classname", "com.chainbase.manuscript.ETLProcessor", "/opt/flink/manuscript.yaml"]