FROM python:3.13.5-alpine3.21@sha256:fd94400585cb7f1ab2be176e84c7388036d6fd2f6e1240711707e519d3bea5aa
RUN apk add --no-cache \
	git=2.47.3-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
