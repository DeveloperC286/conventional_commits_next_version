FROM python:3.13.6-alpine3.21@sha256:52153b829bdf2df5c8c2892387d13a76ff076282e80d407928cdfb9c9e27fc73
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
