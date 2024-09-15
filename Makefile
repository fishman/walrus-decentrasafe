SHELL := bash
.ONESHELL:
.SHELLFLAGS := -e -u -c -o pipefail

USER_ID := $(shell id -u)

DOCKER_COMPOSE := USER_ID=${USER_ID} docker compose

start: ## Start the docker containers
	@echo "Starting the docker containers"
	${DOCKER_COMPOSE} up
	@echo "Containers started - http://localhost:8000"

stop: ## Stop Containers
	${DOCKER_COMPOSE} down

restart: stop start ## Restart Containers

start-bg: ## Run containers in the background
	${DOCKER_COMPOSE} up -d

build: ## Build Containers
	${DOCKER_COMPOSE} build

migrate: ## Run DB migrations in the container
	${DOCKER_COMPOSE} exec walrus-registry diesel migration run

test: ## Run Django tests
	${DOCKER_COMPOSE} exec walrus-registry cargo test

shell: ## Shell into the container
	@docker compose exec walrus-registry sh

init: start-bg migrate  ## Quickly get up and running (start containers and migrate DB)

.PHONY: help
	.DEFAULT_GOAL := help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
