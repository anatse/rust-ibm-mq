#!/usr/bin/env bash
env "RUST_BACKTRACE=1" \
env "RUST_LOG=debug" \
env "mq2kafka.http.host=0.0.0.0" \
env "mq2kafka.http.port=8000" \
env "mq2kafka.http.keepAlive=10" \
env "mq2kafka.http.shutdownTimeout=30" \
env "mq2kafka.kafka.clientId='Audit MQ to Kafka'" \
env "mq2kafka.kafka.socketTimeoutMs=3000" \
env "mq2kafka.kafka.queueBufferingMaxMessages=1" \
env "mq2kafka.kafka.enableIdempotence=true" \
env "mq2kafka.kafka.messageSendMaxRetriesMs=3" \
env "mq2kafka.kafka.retryBackoffMs=100" \
env "mq2kafka.kafka.bootstrap.servers=localhost:9092" \
env "mq2kafka.kafka.security.protocol=PLAINTEXT" \
env "mq2kafka.kafka.messageTimeout=1000" \
env "mq2kafka.kafka.topic.events=audit-events-uncrit-in" \
env "mq2kafka.kafka.topic.events.critical=audit-events-crit-in" \
env "mq2kafka.kafka.topic.metaModel=audit-mm-in" \
env "mq2kafka.kafka.topic.eventsBatch=audit-batch-events-in" \
env "mq2kafka.kafka.ssl.enabled=false" \
env "mq2kafka.wmqcf.ufsAuditCF.qmgrHostname=localhost" \
env "mq2kafka.wmqcf.ufsAuditCF.qmgrPortNumber=1414" \
env "mq2kafka.wmqcf.ufsAuditCF.qmgrName=QM1" \
env "mq2kafka.wmqcf.ufsAuditCF.qmgrSvrconnChannel=DEV.APP.SVRCONN" \
env "mq2kafka.wmqcf.ufsAuditCF.queue=DEV.QUEUE.2" \
env "mq2kafka.wmqcf.ufsAuditCF.fipsRequired=false" \
env "mq2kafka.wmqcf.ufsAuditCF.ssl_enabled=false" \
env "mq2kafka.wmqcf.ufsAuditCF.suite_b1=1" \
env "mq2kafka.wmqcf.ufsAuditCF.suite_b2=0" \
env "mq2kafka.wmqcf.ufsAuditCF.suite_b3=0" \
env "mq2kafka.wmqcf.ufsAuditCF.suite_b4=0" \
env "mq2kafka.object.wmqcf_ufsAuditCF1=true" \
env "mq2kafka.wmqcf.ufsAuditCF1.qmgrHostname=localhost" \
env "mq2kafka.wmqcf.ufsAuditCF1.qmgrPortNumber=1414" \
env "mq2kafka.wmqcf.ufsAuditCF1.qmgrName=QM1" \
env "mq2kafka.wmqcf.ufsAuditCF1.qmgrSvrconnChannel=DEV.APP.SVRCONN" \
env "mq2kafka.wmqcf.ufsAuditCF1.queue=DEV.QUEUE.2" \
env "mq2kafka.wmqcf.ufsAuditCF1.fipsRequired=false" \
env "mq2kafka.wmqcf.ufsAuditCF1.ssl_enabled=false" \
env "mq2kafka.wmqcf.ufsAuditCF1.suite_b1=1" \
env "mq2kafka.wmqcf.ufsAuditCF1.suite_b2=0" \
env "mq2kafka.wmqcf.ufsAuditCF1.suite_b3=0" \
env "mq2kafka.wmqcf.ufsAuditCF1.suite_b4=0" \
env "mq2kafka.object.wmqcf_ufsAuditCF2=true" \
env "mq2kafka.wmqcf.ufsAuditCF2.qmgrHostname=localhost" \
env "mq2kafka.wmqcf.ufsAuditCF2.qmgrPortNumber=1414" \
env "mq2kafka.wmqcf.ufsAuditCF2.qmgrName=QM1" \
env "mq2kafka.wmqcf.ufsAuditCF2.qmgrSvrconnChannel=DEV.APP.SVRCONN" \
env "mq2kafka.wmqcf.ufsAuditCF2.queue=DEV.QUEUE.2" \
env "mq2kafka.wmqcf.ufsAuditCF2.fipsRequired=false" \
env "mq2kafka.wmqcf.ufsAuditCF2.ssl_enabled=false" \
env "mq2kafka.wmqcf.ufsAuditCF2.suite_b1=1" \
env "mq2kafka.wmqcf.ufsAuditCF2.suite_b2=0" \
env "mq2kafka.wmqcf.ufsAuditCF2.suite_b3=0" \
env "mq2kafka.wmqcf.ufsAuditCF2.suite_b4=0" bash