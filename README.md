# expenses-bot

Currently WIP.

## Setup

To run an instance of this bot, you need a docker.

```docker-compose -f docker-compose/app-setup.yml -p develop up -d postgres```

To stop environment run:

```docker-compose -f docker-compose/app-setup.yml -p develop down```

## Test-Setup

To run integration tests run this command.

```docker-compose -f docker-compose/integration-tests-setup.yml -p it up -d postgres```

To stop environment run:

```docker-compose -f docker-compose/integration-tests-setup.yml -p it down```
