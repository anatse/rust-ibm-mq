#!/usr/bin/env bash
# Уровень логирования
RUST_LOG=debug

# параметры HTTP сервера
mq2kafka.http.host=0.0.0.0
mq2kafka.http.port=8000
mq2kafka.http.keepAlive=10
mq2kafka.http.shutdownTimeout=30

# Подключение к Kafka
mq2kafka.kafka.clientId="Audit MQ to Kafka"
mq2kafka.kafka.socketTimeoutMs=3000
mq2kafka.kafka.queueBufferingMaxMessages=1
mq2kafka.kafka.enableIdempotence=true
mq2kafka.kafka.messageSendMaxRetriesMs=3
mq2kafka.kafka.retryBackoffMs=100
mq2kafka.kafka.bootstrap.servers=localhost:9092
mq2kafka.kafka.security.protocol=PLAINTEXT
mq2kafka.kafka.messageTimeout=1000

# Kafka Топики
mq2kafka.kafka.topic.events=audit-events-uncrit-in
mq2kafka.kafka.topic.events.critical=audit-events-crit-in
mq2kafka.kafka.topic.metaModel=audit-mm-in
mq2kafka.kafka.topic.eventsBatch=audit-batch-events-in

# Kafka SSL
mq2kafka.kafka.ssl.enabled=false
# Optional parameters
# mq2kafka.kafka.messageTimeout=
# mq2kafka.kafka.messageTimeout=
# mq2kafka.kafka.ssl.truststore.storeType=
# mq2kafka.kafka.ssl.truststore.location=
# mq2kafka.kafka.ssl.truststore.password=
# mq2kafka.ssl.kafka.keystore.storeType=
# mq2kafka.ssl.kafka.keystore.location=
# mq2kafka.ssl.kafka.keystore.password=
# mq2kafka.ssl.kafka.keystore.keyPassword=

# Audit MQ Server to send
mq2kafka.wmqcf.ufsAuditCF.qmgrHostname=localhost
mq2kafka.wmqcf.ufsAuditCF.qmgrPortNumber=1414
mq2kafka.wmqcf.ufsAuditCF.qmgrName=QM1
mq2kafka.wmqcf.ufsAuditCF.qmgrSvrconnChannel=DEV.APP.SVRCONN
mq2kafka.wmqcf.ufsAuditCF.queue=DEV.QUEUE.2
mq2kafka.wmqcf.ufsAuditCF.fipsRequired=false
mq2kafka.wmqcf.ufsAuditCF.ssl_enabled=false
# mq2kafka.wmqcf.ufsAuditCF.user=mqm
# mq2kafka.wmqcf.ufsAuditCF.user=mqm
# mq2kafka.wmqcf.ufsAuditCF.ssl_key_repos_stem
# mq2kafka.wmqcf.ufsAuditCF.cipher_spec=TLS_RSA_WITH_AES_128_GCM_SHA256
# mq2kafka.wmqcf.ufsAuditCF.certificate_label
# mq2kafka.wmqcf.ufsAuditCF.oscp_url
# Suite_b
mq2kafka.wmqcf.ufsAuditCF.suite_b1=1
mq2kafka.wmqcf.ufsAuditCF.suite_b2=0
mq2kafka.wmqcf.ufsAuditCF.suite_b3=0
mq2kafka.wmqcf.ufsAuditCF.suite_b4=0

# MQ Servers for consume
mq2kafka.object.wmqcf_ufsAuditCF1=true
mq2kafka.wmqcf.ufsAuditCF1.qmgrHostname=localhost
mq2kafka.wmqcf.ufsAuditCF1.qmgrPortNumber=1414
mq2kafka.wmqcf.ufsAuditCF1.qmgrName=QM1
mq2kafka.wmqcf.ufsAuditCF1.qmgrSvrconnChannel=DEV.APP.SVRCONN
mq2kafka.wmqcf.ufsAuditCF1.queue=DEV.QUEUE.2
mq2kafka.wmqcf.ufsAuditCF1.fipsRequired=false
mq2kafka.wmqcf.ufsAuditCF1.ssl_enabled=false
# mq2kafka.wmqcf.ufsAuditCF1.user=mqm
# mq2kafka.wmqcf.ufsAuditCF1.user=mqm
# mq2kafka.wmqcf.ufsAuditCF1.ssl_key_repos_stem
# mq2kafka.wmqcf.ufsAuditCF1.cipher_spec=TLS_RSA_WITH_AES_128_GCM_SHA256
# mq2kafka.wmqcf.ufsAuditCF1.certificate_label
# mq2kafka.wmqcf.ufsAuditCF1.oscp_url
# Suite_b
mq2kafka.wmqcf.ufsAuditCF1.suite_b1=1
mq2kafka.wmqcf.ufsAuditCF1.suite_b2=0
mq2kafka.wmqcf.ufsAuditCF1.suite_b3=0
mq2kafka.wmqcf.ufsAuditCF1.suite_b4=0

mq2kafka.object.wmqcf_ufsAuditCF2=true
mq2kafka.wmqcf.ufsAuditCF2.qmgrHostname=localhost
mq2kafka.wmqcf.ufsAuditCF2.qmgrPortNumber=1414
mq2kafka.wmqcf.ufsAuditCF2.qmgrName=QM1
mq2kafka.wmqcf.ufsAuditCF2.qmgrSvrconnChannel=DEV.APP.SVRCONN
mq2kafka.wmqcf.ufsAuditCF2.queue=DEV.QUEUE.2
mq2kafka.wmqcf.ufsAuditCF2.fipsRequired=false
mq2kafka.wmqcf.ufsAuditCF2.ssl_enabled=false
# mq2kafka.wmqcf.ufsAuditCF2.user=mqm
# mq2kafka.wmqcf.ufsAuditCF2.user=mqm
# mq2kafka.wmqcf.ufsAuditCF2.ssl_key_repos_stem
# mq2kafka.wmqcf.ufsAuditCF2.cipher_spec=TLS_RSA_WITH_AES_128_GCM_SHA256
# mq2kafka.wmqcf.ufsAuditCF2.certificate_label
# mq2kafka.wmqcf.ufsAuditCF2.oscp_url
# Suite_b
mq2kafka.wmqcf.ufsAuditCF2.suite_b1=1
mq2kafka.wmqcf.ufsAuditCF2.suite_b2=0
mq2kafka.wmqcf.ufsAuditCF2.suite_b3=0
mq2kafka.wmqcf.ufsAuditCF2.suite_b4=0
mq2kafka.mq.workersPerFactory=2

# Переменные среду для поиска системных библиотек
OPENSSL_ROOT_DIR=/usr/local/opt/openssl
OPENSSL_LIBRARIES=/usr/local/opt/openssl/lib
RUST_BACKTRACE=full
DYLD_LIBRARY_PATH=/Users/asementsov/projects/9.1.5.0-IBM-MQ-Toolkit-MacX64/lib64/