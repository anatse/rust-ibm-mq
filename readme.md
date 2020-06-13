# REST модуль для Аудита
Модуль выполняет отправку сообщений аудита в Kafka. Модуль реализован на языке Rust.
 - [Rust book (RUS)](https://doc.rust-lang.ru/book)
 - [Rustonomicon](https://doc.rust-lang.org/nomicon/)
 
## Компоненты и библиотеки
 - Для реализации рестов используется фреймворк [Actix-Web](https://github.com/actix/actix-web)
 - Для отправки сообщений в Kafka используется библиотека [rust-rdkafka](https://github.com/fede1024/rust-rdkafka)
 - Для работы с MQ используется C клиент MQ. Для MacOS скачать библиотеки можно отсюда [MacOS Ibm MQ Toolkit](https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/mactoolkit/9.1.5.0-IBM-MQ-Toolkit-MacX64.tar.gz) 
 - Параметры, которые в принципе можно установить при конфигурации kafka producer-а [LibRdKafka](https://github.com/edenhill/librdkafka/blob/master/CONFIGURATION.md)

## Запуск MQ в докере
```shell script
docker run -p 9443:9443 -p 1414:1414 \
    --env MQ_ADMIN_PASSWORD=123456 --env LICENSE=accept --env MQ_QMGR_NAME=QM1 \
    --name ibm_mq \
    ibmcom/mq
```

## Текущая конфигурация
 - [application.conf](application.conf)

## Запуск
### Подготовка

#### Установить C библиотеку для Kafka
```shell script
brew install librdkafka
```

#### Установить MQ Toolkit MacOS
```shell script
mkdir /usr/local/lib/9.1.5.0-IBM-MQ-Toolkit-MacX64
cd /usr/local/lib/9.1.5.0-IBM-MQ-Toolkit-MacX64
wget https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/mactoolkit/9.1.5.0-IBM-MQ-Toolkit-MacX64.tar.gz
tar -xvf 9.1.5.0-IBM-MQ-Toolkit-MacX64.tar.gz
rm -f 9.1.5.0-IBM-MQ-Toolkit-MacX64.tar.gz
```

#### Установить OpenSSL
```shell script
brew install openssl
```
Установить переменные среды
[environment](release/env.sh)

Переменная среды DYLD_LIBRARY_PATH устанавливается на папку lib64 библиотек MQ
```shell script
export DYLD_LIBRARY_PATH=/usr/local/lib/9.1.5.0-IBM-MQ-Toolkit-MacX64/lib64/
```

На компьютере должен быть установлен rust, см инструкцию по установке Rust
Для сборки и запуска используется cargo
```shell script
cargo install
cargo build
cargo run
```
После этого станет доступен сервис на порту 8000

### Выпуск релизной версии
```shell script
cargo build --release
```

### Сборка в докере
Необходимо отключить VPN, иначе cargo будет ругаться на сертификаты. Если же нельзя его отключить,
то необходимо выполнить следующие рекомендации. 

<span style="color:red">**Disclaimer**: Информация не проверена</span>

```text
I have run some tests in docker container. 
The problem is I have a self signed certificate behind my local machine proxy.

The solution is: 
 - I run container with -v /self-signed-CA-cert.pem:/in_docker_cert.pem, 
 - then export SSL_CERT_FILE=/in_docker_cert.pem.

Problem solved.
```

#### Сборка и вызгурзка docker image
[build-docker.sh](release/build-with-docker.sh)

 - Исполняемый файл [mq2kafka](release/linux_x64/mq2kafka)
 - Выгруженный образ будет находиться здесь [audit_mq2kafka.tar.gz](release/docker-image/audit_mq2kafka.tar.gz)

#### Удалить все докер контейнеры
```shell script
docker rm $(docker ps -a -q)
```

Удалить все image
```shell script
docker rmi $(docker images -a -q)
```

### Тестовые запросы
 - [event-send](event-send.rest) - Отправка тестового события аудита

