FROM python:3.8-alpine

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

EXPOSE 8000

COPY requirements.txt .

RUN \
    apk add --no-cache postgresql-libs && \
    apk add --no-cache --virtual .build-deps gcc musl-dev postgresql-dev && \
    pip install -r requirements.txt && \
    apk --purge del .build-deps

WORKDIR /lokajapi
COPY ./lokajapi/ /lokajapi

CMD ["python", "manage.py", "runserver", "0.0.0.0:8000"]
