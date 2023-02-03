run:
	maturin build
	pip install /workspaces/umbrella/target/wheels/umbrella-0.1.0-cp310-cp310-manylinux_2_28_x86_64.whl --force-reinstall
	python '/workspaces/umbrella/test.py'