#!/usr/bin/env bash

export DYLD_LIBRARY_PATH=$HOME/CLionProjects/IBM-MQ-Toolkit-Mac-x64-9.1.3.0/lib64

cargo install bindgen

docker run -p 9443:9443 \
    --env MQ_APP_PASSWORD=123456 --env MQ_ADMIN_PASSWORD=123456 --env LICENSE=accept --env MQ_QMGR_NAME=MGR \
    --name ibm_mq \
    ibmcom/mq

# How-to extract RPM
brew install rpm2cpio
rpm2cpio freeswitch-1.6.17-7.mga6.src.rpm | cpio -idmv
