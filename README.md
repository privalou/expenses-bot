# expenses-bot

Currently WIP.

## Setup

To run an instance of this bot, you need a docker.

```docker-compose -f docker-compose/docker-compose.yml -p develop --env-file ./docker-compose/.env up```

To stop environment run:

```docker-compose -f docker-compose/docker-compose.yml -p develop down```

## Test-Setup

To run integration tests run this command.

```docker-compose -f docker-compose/docker-compose.yml -p it --env-file ./docker-compose/.env.test up```

To stop environment run:

```docker-compose -f docker-compose/docker-compose.yml -p it down```
