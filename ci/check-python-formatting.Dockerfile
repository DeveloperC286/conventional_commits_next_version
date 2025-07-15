FROM python:3.13.5-alpine3.21@sha256:6a5f50aba058538dc0098afd77fa264b7382972d8457df5d940355132e27253a
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
