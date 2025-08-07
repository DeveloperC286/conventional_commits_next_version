FROM python:3.13.6-alpine3.21@sha256:cff80330401ab70400dea97cb2a20c501febeb7ba4c1b6a4c374aacce27c5a7d
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
