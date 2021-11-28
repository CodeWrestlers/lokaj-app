run:
	-docker container rm lokaj-bot
	docker run --env-file .env --name lokaj-bot lokaj-bot 

start:
	-docker container rm lokaj-bot
	docker run -d --env-file .env --name lokaj-bot lokaj-bot 

debug:
	docker run -it lokaj-bot bash

build:
	docker build -t lokaj-bot .

rebuild:
	docker build -t lokaj-bot . --no-cache

kill:
	docker container kill lokaj-bot

stop:
	docker container stop lokaj-bot &> /dev/null &
