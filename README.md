# expenses-bot

Currently WIP.

## Setup

To run an instance of this bot, you need a docker.

```docker-compose -p develop --env-file ./docker-compose/.env up -d```

To stop environment run:

```docker-compose -p develop down```

## Test-Setup

To run integration tests run this command.

```docker-compose -p it --env-file ./docker-compose/.env.test up -d```

To stop environment run:

```docker-compose -p it down```
