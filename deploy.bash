#!/bin/bash

git pull
docker compose down -v

docker compose up -d --build