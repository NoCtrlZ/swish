FROM alpine:3.12

COPY . .

RUN apk add --update --upgrade --no-cache \
    rust=1.44.0-r0 \
    cargo=1.44.0-r0 \
    nodejs \
    npm &&\
    (cd client && npm i --production)

ENTRYPOINT [ "sh", "integration_test.sh" ]
