# lokaj-bot
[![Docker Image CI](https://github.com/mchelpa/lokaj-bot/actions/workflows/docker-image.yml/badge.svg?branch=main)](https://github.com/mchelpa/lokaj-bot/actions/workflows/docker-image.yml)
## Prerequisites
Create your bot with @BotFather on Telegram

## Running the project
1. Clone the repo
2. Copy `.env.template` to the same location and change name to `.env`
3. Paste telegram token and set database username & password
4. Run `source load_dotenv.sh`
5. Run `docker-compose up`
