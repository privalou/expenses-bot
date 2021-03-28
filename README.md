# bot

Currently WIP.

## Setup

To run an instance of this bot, you need a docker.

```docker-compose -p develop --env-file ./.env up -d```

To stop environment run:

```docker-compose -p develop down```

## Test-Setup

To run environment for integration tests run this command.

```docker-compose -p it --env-file ./env.test up -d```

To stop environment run:

```docker-compose -p it down```
