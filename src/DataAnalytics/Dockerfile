# syntax = docker/dockerfile:experimental
FROM python:3.12-alpine

ARG PORT
ENV PORT $PORT

WORKDIR ./DataAnalyticsMS

COPY . .

RUN --mount=type=cache,target=/root/.cache/pip pip install --upgrade -r ./requirements.txt

EXPOSE $PORT

CMD fastapi run main.py --port $PORT

