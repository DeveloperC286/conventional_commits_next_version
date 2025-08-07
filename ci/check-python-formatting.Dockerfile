FROM python:3.13.6-alpine3.21@sha256:cff80330401ab70400dea97cb2a20c501febeb7ba4c1b6a4c374aacce27c5a7d
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
