FROM quay.io/fedora/fedora:39 AS builder

ENV WORKERD_VERSION="v1.20231030.0"

WORKDIR /src
RUN set -o pipefail \
  && curl -sSL -o - "https://github.com/cloudflare/workerd/releases/download/$WORKERD_VERSION/workerd-linux-64.gz" | gunzip > workerd
RUN chmod +x workerd

FROM quay.io/fedora/fedora-minimal:39

WORKDIR /var/workerd
COPY --from=builder /src/workerd /usr/bin/workerd
COPY --chmod=555 /entrypoint.sh /entrypoint.sh
COPY /config.capnp /var/workerd/
COPY /workers /var/workerd/workers

STOPSIGNAL SIGKILL

VOLUME [ "/var/workerd" ]
EXPOSE 8080

USER nobody:nobody

ENTRYPOINT [ "/entrypoint.sh" ]
CMD [ "--verbose", "--watch", "serve", "config.capnp" ]
