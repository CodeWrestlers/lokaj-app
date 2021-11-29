#!/bin/bash

# run with `source ./load_dotenv.sh`

if [ -f .env ]; then
  export $(echo $(cat .env | sed 's/#.*//g'| xargs) | envsubst)
fi

export DATABASE_URL=postgres://$DATABASE_USER:$DATABASE_PASSWORD@localhost/$DATABASE_NAME
