#!/usr/bin/env bash

# Create docker image
docker build -t mq2kafka .

# Start image
docker run --name "mq2kafka" mq2kafka bash
# Copy file from container to folder
docker cp $(docker ps -a -q -f "name=mq2kafka"):/usr/local/cargo/bin/mq2kafka release/linux_x64
# Stop container
docker stop $(docker ps -a -q -f "name=v")
# Drop container
docker rm $(docker ps -a -q -f "name=mq2kafka")

# Save docker image to file with compression by gzip
# Uncomment if you want to save base image
#mkdir release/docker-image
#docker save audit-res:latest | gzip > release/docker-image/audit_mq2kafka.tar.gz