FROM alpine:latest

RUN apk --no-cache add dnsmasq

EXPOSE 53 53/udp

ENTRYPOINT ["dnsmasq", "-k"]
