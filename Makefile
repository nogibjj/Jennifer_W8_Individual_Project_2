# Makefile for Python

install_py:
	@pip install --upgrade pip
	@pip install -r requirements.txt

test_py:
	@python -m pytest -vv --cov=main test_*.py

format_py:	
	@black *.py 

lint_py:
	# pylint --disable=R,C --ignore-patterns=test_.*?py *.py
	# ruff linting is 10-100X faster than pylint
	@ruff check *.py 

container-lint_py:
	@docker run --rm -i hadolint/hadolint < Dockerfile

refactor_py: 
	format_py lint_py

deploy_py:
	# deploy goes here
	@echo "Deploying Python project"

all_py: install_py lint_py test_py format_py deploy_py

