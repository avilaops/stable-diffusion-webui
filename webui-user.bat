@echo off

set PYTHON=python
set GIT=
set VENV_DIR=
set COMMANDLINE_ARGS=--skip-torch-cuda-test --no-half --precision full --opt-sub-quad-attention --api --listen --port 7860

call webui.bat %*
