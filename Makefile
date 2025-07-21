rustlings:
	@echo "Starting Rustlings.."
	@cd ./rustlings && rustlings

web:
	@echo "Starting custom minimal web server"
	@cd ./web-server/ && cargo run 

.PHONY: rustlings
