FROM registry.gitlab.com/wyrcan/debian
RUN apt-get update && apt-get -y install sudo
RUN useradd -m john && echo "john:john" | chpasswd && adduser john sudo
