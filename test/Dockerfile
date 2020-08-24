FROM alpine:3.12

COPY . .

RUN apk add --update --upgrade \
    rust=1.44.0-r0 \
    cargo=1.44.0-r0 \
    nodejs=12.18.3-r0 \
    npm=12.18.3-r0 && \
    (cd client && npm i --production)

ENTRYPOINT [ "sh", "integration_test.sh" ]
