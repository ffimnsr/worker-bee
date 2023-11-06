#!/usr/bin/env bash

set -eu

podman build \
  --label org.opencontainers.image.created=$(date --iso-8601=ns) \
  --label org.opencontainers.image.authors=gh:@ffimnsr \
  --label org.opencontainers.image.description="Cloudflare Workers Runtime" \
  --label org.opencontainers.image.revision=$(git rev-parse HEAD) \
  --label org.opencontainers.image.source=$(git remote get-url origin) \
  --label org.opencontainers.image.title=worker-bee \
  --label org.opencontainers.image.url=https://github.com/ffimnsr/worker-bee \
  -t quay.vastorigins.org/library/worker-bee:test-build .
