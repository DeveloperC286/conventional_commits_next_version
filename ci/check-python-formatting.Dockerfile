FROM python:3.14.0-alpine3.21@sha256:3f44f9ba780fafb41519c38ce526d4fbacb1ff49a146ba2c76b34897709a5b58
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
