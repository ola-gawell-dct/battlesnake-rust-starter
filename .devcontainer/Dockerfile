FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

COPY . /usr/app
WORKDIR /usr/app

COPY --from=golang:1.23.5-bullseye /usr/local/go/ /usr/local/go/
 
ENV PATH="/usr/local/go/bin:${PATH}"

RUN apt-get update && apt-get install -y wget tar
RUN wget https://github.com/BattlesnakeOfficial/rules/releases/download/v1.2.3/battlesnake_1.2.3_Linux_arm64.tar.gz -O /tmp/battlesnake.tar.gz \
    && tar -xzf /tmp/battlesnake.tar.gz -C /usr/local/bin \
    && rm /tmp/battlesnake.tar.gz

#COPY bin/battlesnake /usr/local/bin/bsnake
RUN chmod +x /usr/local/bin/battlesnake

