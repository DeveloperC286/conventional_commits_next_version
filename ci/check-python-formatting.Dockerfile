FROM python:3.13.6-alpine3.21@sha256:52153b829bdf2df5c8c2892387d13a76ff076282e80d407928cdfb9c9e27fc73
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
