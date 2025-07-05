from nginx:1.28-alpine-slim

copy ./appended_hosts /etc/appended_hosts
run cat /etc/appended_hosts >> /etc/hosts
