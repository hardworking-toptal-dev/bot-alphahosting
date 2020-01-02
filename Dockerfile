FROM centos:centos8
# FROM ubuntu:18.04

RUN mkdir /etc/letsencrypt
RUN mkdir /etc/certbot-alfahosting

RUN dnf install -y wget
RUN wget https://dl.google.com/linux/direct/google-chrome-stable_current_x86_64.rpm
RUN dnf install -y ./google-chrome-stable_current_x86_64.rpm
RUN rm ./google-chrome-stable_current_x86_64.rpm

# RUN apt-get -y update && apt-get -y install chromium-browser

COPY target/x86_64-unknown-linux-gnu/release/certbot-alfahosting /usr/bin/certbot-alfahosting

VOLUME [ "/etc/letsencrypt", "/etc/certbot-alfahosting" ]

CMD [ "/usr/bin/certbot-alfahosting" ]