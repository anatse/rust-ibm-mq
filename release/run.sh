#!/usr/bin/env bash
for i in {1..1000}
do
   curl http://localhost:8000/mq/send_test
done