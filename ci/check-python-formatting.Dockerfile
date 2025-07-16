FROM python:3.13.5-alpine3.21@sha256:fd94400585cb7f1ab2be176e84c7388036d6fd2f6e1240711707e519d3bea5aa
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--exit-code", "--diff", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
