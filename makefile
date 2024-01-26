DEPLOY_DIR = ./docker
TEST_COMPOSE = $(DEPLOY_DIR)/compose.yaml

###################
#### LOCAL ########
###################
test_log:
	cargo test -- --nocapture > ./logs/test_logs.log


###################
#### TESTING ######
###################
compose_build_dev:
	docker-compose -f $(TEST_COMPOSE) build dev

compose_build_test:
	docker-compose -f $(TEST_COMPOSE) build test

compose_run_dev_it:
	docker-compose -f $(TEST_COMPOSE) run -it --rm

compose_run_dev: compose_build_dev compose_run_dev_it

action_compose_test: ## Runs the tests in a container.
	docker-compose -f $(TEST_COMPOSE) run --rm test

compose_remove: ## Stops and removes the testing containers, images, volumes.
	docker-compose -f $(TEST_COMPOSE) down --volumes --remove-orphans

compose_test: compose_build_test action_compose_test compose_remove

.PHONY: help
help: ## Show available make targets.
	@awk '/^[^\t ]*:.*?##/{sub(/:.*?##/, ""); printf "\033[36m%-30s\033[0m %s\n", $$1, substr($$0, index($$0,$$2))}' $(MAKEFILE_LIST)
