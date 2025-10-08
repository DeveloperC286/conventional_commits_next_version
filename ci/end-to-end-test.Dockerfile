FROM python:3.14.0-alpine3.21@sha256:3f44f9ba780fafb41519c38ce526d4fbacb1ff49a146ba2c76b34897709a5b58
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
