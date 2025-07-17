rustlings:
	@echo "Starting Rustlings.."
	@cd ./rustlings && rustlings

gcp:
	@git add .
	@git commit -m $(ex)
	@git push origin HEAD


.PHONY: rustlings