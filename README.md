# manuscript-core

![manuscript](./images/manuscript_logo.jpeg)

[![GitHub Version](https://img.shields.io/github/tag-pre/Liquidwe/rust-examples?label=Version&color=D4B68C)](https://github.com/chainbase-labs/manuscript-core/releases)
![PyPI License](https://img.shields.io/pypi/l/quixstreams?label=Licence&color=D4B68C)
[![Docs](https://img.shields.io/badge/docs-chainbase.com-0345b2?label=Docs&color=D4B68C)](https://docs.chainbase.com/core-concepts/manuscript/overview#core-values-and-capabilities-of-manuscript)
<a href="https://codecov.io/gh/kool-dev/kool"><img src="https://codecov.io/gh/kool-dev/kool/branch/main/graph/badge.svg" alt="codecov"></a>
[![Tests on Linux, MacOS and Windows](https://github.com/gohugoio/hugo/workflows/Test/badge.svg)](https://github.com)
[![Go Report Card](https://goreportcard.com/badge/github.com/gohugoio/hugo)](https://goreportcard.com)\
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fkubescape%2Fkubescape.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fkubescape%2Fkubescape?ref=badge_shield&issueType=license)
[![X](https://img.shields.io/twitter/url?&color=D4B68C&label=&style=social&url=https%3A%2F%2Fx.com%2FchainbaseHQ)](https://x.com/chainbaseHQ)
[![Community Slack](https://img.shields.io/badge/Community%20Slack-blueviolet?logo=slack)](https://join.slack.com/share/enQtNzc4NDI3Mzk2Njg3MS1hZjdhOGY0ZTU5ODk3ZmY0NDAzN2JiY2YxMjNmOTY5NmEwNWNhMDhiMWE0M2I1ZDc2YzI1NDQ3ZDhhMWQ4Zjg0?cdn_fallback=1)
[![Discord](https://img.shields.io/badge/Chainbase-0345b2?logo=Discord)](https://discord.gg/YnAavwwR)
[![Discord](https://img.shields.io/badge/Chainbase-0345b2?logo=Telegram)](https://te.me/ChainbaseNetwork)

# Build The World's Largest Omnichain Data Network
***
Chainbase is a global blockchain data network with an extensive dataset and cluster worldwide. If we compare Chainbase’s global data network to a country, then Manuscript would be the language of this data network nation. Manuscript plays a crucial role in the Chainbase ecosystem, serving as a bridge connecting various data, services, and users.
### what is manuscript?
![manuscript](./images/manuscript_pipeline.png)
Manuscript is not just a language specification; it’s a protocol, framework, and toolkit designed to simplify and unify data access and processing methods. Through Manuscript, developers and users can more easily interact with the vast amount of data in the Chainbase network, whether querying, analyzing, or applying this data.
The vision of Manuscript is to realize “data trade” within the Chainbase network, establishing a Chainbase ecosystem component that allows users to access any data through any means, across any service, using any language. This grand vision can be broken down into the following key aspects:

- Any language: We hope users can use scripts in any mainstream programming language to customize data, including but not limited to: Golang, Rust, Python, Node.js, Java, C/C++, Zig, WebAssembly (WASM)
- Any method: Different users are familiar with different forms of data access, we hope users can access data through various means, including but not limited to: SQL, DataFrames, HTTPS, gRPC, FTP, WebDAV, FUSE
- Any data: Users should be able to access data in any format, such as: JSON, CSV, ORC, XML, XLSX, BLOB
- Across any service: Users’ expected data storage services also vary, we hope users can access, transfer, and control data in any service, such as: RPC, S3, IPFS, Azblob, HDFS, Google Drive, BigQuery, WebDAV, MySQL, PostgreSQL
### Value of Manuscript
- **Programmability**: Manuscript provides powerful programmable interfaces that allow developers to customize data processing workflows according to their needs. This flexibility means that Manuscript can be used not only for simple data queries but also for building complex data analysis pipelines and applications. Through programmability, Manuscript opens up infinite possibilities for innovative applications of blockchain data.

- **Interoperability**: With the booming development of blockchain technology, it’s becoming increasingly difficult for different blockchains to understand and process each other’s data. Manuscript can solve the interoperability problem of multi-chain and off-chain data aggregation in any dimension. By providing unified interfaces and data processing methods, Manuscript enables seamless integration of data from different blockchains, greatly improving the development efficiency and feasibility of cross-chain applications.

- **Monetization**: Leveraging the data capabilities provided by Manuscript, combined with the dual-chain architecture CometBFT + DPoS high-performance instant transaction finality and proof-of-stake consensus features, Chainbase offers a fair and transparent data value exchange ecosystem. Creators can monetize their processed data through Manuscript, while data users can conveniently consume the data they need. This mechanism not only incentivizes the production of high-quality data but also promotes the positive development of the entire blockchain ecosystem.

## Getting Started 🏄
### Install Manuscript Client
```shell
# For macOs
curl -fsSL  https://github.com/Liquidwe/rust-examples/raw/main/install.sh | bash
```
### Requirements
Docker Desktop 25.1+, Docker Compose v2.20.0+

### Example

Here's an example of how to <b>process</b> data from a Kafka Topic with Quix Streams:

```python
from quixstreams import Application

# A minimal application reading temperature data in Celsius from the Kafka topic,
# converting it to Fahrenheit and producing alerts to another topic.

# Define an application that will connect to Kafka
app = Application(
    broker_address="localhost:9092",  # Kafka broker address
)

# Define the Kafka topics
temperature_topic = app.topic("temperature-celsius", value_deserializer="json")
alerts_topic = app.topic("temperature-alerts", value_serializer="json")

# Create a Streaming DataFrame connected to the input Kafka topic
sdf = app.dataframe(topic=temperature_topic)

# Convert temperature to Fahrenheit by transforming the input message (with an anonymous or user-defined function)
sdf = sdf.apply(lambda value: {"temperature_F": (value["temperature"] * 9/5) + 32})

# Filter values above the threshold
sdf = sdf[sdf["temperature_F"] > 150]

# Produce alerts to the output topic
sdf = sdf.to_topic(alerts_topic)

# Run the streaming application (app automatically tracks the sdf!)
app.run()
```

### Tutorials

To see Quix Streams in action, check out the Quickstart and Tutorials in the docs:

- [**Quickstart**](https://quix.io/docs/quix-streams/quickstart.html)
- [**Tutorial - Word Count**](https://quix.io/docs/quix-streams/tutorials/word-count/tutorial.html)
- [**Tutorial - Anomaly Detection**](https://quix.io/docs/quix-streams/tutorials/anomaly-detection/tutorial.html)
- [**Tutorial - Purchase Filtering**](https://quix.io/docs/quix-streams/tutorials/purchase-filtering/tutorial.html)


### Key Concepts
There are two primary objects:
- `StreamingDataFrame` - a predefined declarative pipeline to process and transform incoming messages.
- `Application` - to manage the Kafka-related setup, teardown and message lifecycle (consuming, committing). It processes each message with the dataframe you provide for it to run.

Under the hood, the `Application` will:
- Consume and deserialize messages.
- Process them with your `StreamingDataFrame`.
- Produce it to the output topic.
- Automatically checkpoint processed messages and state for resiliency.
- Scale using Kafka's built-in consumer groups mechanism.


### Deployment
You can run Quix Streams pipelines anywhere Python is installed.

Deploy to your own infrastructure or to [Quix Cloud](https://quix.io/product) on AWS, Azure, GCP or on-premise for a fully managed platform.  
You'll get self-service DevOps, CI/CD and monitoring, all built with best in class engineering practices learned from Formula 1 Racing.

Please see the [**Connecting to Quix Cloud**](https://quix.io/docs/quix-streams/quix-platform.html) page
to learn how to use Quix Streams and Quix Cloud together.

## Roadmap 📍

This library is being actively developed by a full-time team.

Here are some of the planned improvements:

- [x] [Windowed aggregations over Tumbling & Hopping windows](https://quix.io/docs/quix-streams/v2-0-latest/windowing.html)
- [x] [Stateful operations and recovery based on Kafka changelog topics](https://quix.io/docs/quix-streams/advanced/stateful-processing.html)
- [x] [Group-by operation](https://quix.io/docs/quix-streams/groupby.html)
- [x] ["Exactly Once" delivery guarantees for Kafka message processing (AKA transactions)](https://quix.io/docs/quix-streams/configuration.html#processing-guarantees)
- [x] Support for [Avro](https://quix.io/docs/quix-streams/advanced/serialization.html#avro) and [Protobuf](https://quix.io/docs/quix-streams/advanced/serialization.html#protobuf) formats
- [x] [Schema Registry support](https://quix.io/docs/quix-streams/advanced/schema-registry.html)
- [ ] Joins
- [ ] Windowed aggregations over Sliding windows

For a more detailed overview of the planned features, please look at [the Roadmap Board](https://github.com/orgs/quixio/projects/1).

## Get Involved 🤝

- Please use [GitHub issues](https://github.com/quixio/quix-streams/issues) to report bugs and suggest new features.
- Join the [Quix Community on Slack](https://quix.io/slack-invite), a vibrant group of Kafka Python developers, data engineers and newcomers to Apache Kafka, who are learning and leveraging Quix Streams for real-time data processing.
- Watch and subscribe to [@QuixStreams on YouTube](https://www.youtube.com/@QuixStreams) for code-along tutorials from scratch and interesting community highlights.
- Follow us on [X](https://x.com/Quix_io) and [LinkedIn](https://www.linkedin.com/company/70925173) where we share our latest tutorials, forthcoming community events and the occasional meme.
- If you have any questions or feedback - write to us at support@quix.io!


## License 📗

Quix Streams is licensed under the Apache 2.0 license.  
View a copy of the License file [here](https://github.com/quixio/quix-streams/blob/main/LICENSE).
