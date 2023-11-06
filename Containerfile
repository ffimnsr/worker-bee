FROM quay.io/fedora/fedora:39 AS builder

WORKDIR /src
RUN curl -sSL -o - "https://github.com/cloudflare/workerd/releases/download/v1.20231030.0/workerd-linux-64.gz" | gunzip > workerd
RUN chmod +x workerd

FROM quay.io/fedora/fedora-minimal:39

WORKDIR /etc/workerd
COPY --from=builder /src/workerd /usr/bin/workerd
COPY /config.capnp /etc/workerd/
COPY /workers /etc/workerd/workers

STOPSIGNAL SIGINT

VOLUME [ "/etc/workerd" ]
EXPOSE 8080

ENTRYPOINT [ "/usr/bin/workerd" ]
CMD [ "--verbose", "serve", "config.capnp" ]

