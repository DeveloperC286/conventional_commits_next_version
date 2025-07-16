FROM python:3.13.5-alpine3.21@sha256:716e13ae565fb303ab17959bba89fabce3e69c41ddc59985a9dd941c42baa517
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
