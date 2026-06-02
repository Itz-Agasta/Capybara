.PHONY: lint format typecheck check run-engine run-mcp clean

lint:
	uv run ruff check .

format:
	uv run ruff format .

typecheck:
	uv run ty check .

check: lint typecheck
	uv run ruff format --check .

run-engine:
	uv run -m engine.main

run-mcp:
	uv run -m mcp.main

clean:
	find . -type d -name '__pycache__' -exec rm -rf {} + 2>/dev/null
	find . -type d -name '*.egg-info' -exec rm -rf {} + 2>/dev/null
	rm -rf .ruff_cache/ .ty_cache/
	find . -name '*.pyc' -delete
