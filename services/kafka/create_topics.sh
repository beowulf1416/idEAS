#! /bin/sh
cd kafka_2.13-3.2.1

# create topics
bin/kafka-topics.sh --create --topic mailer --bootstrap-server localhost:9093