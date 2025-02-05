FROM --platform=$BUILDPLATFORM debian:12.9-slim as build
ARG TARGETPLATFORM
ARG BUILDPLATFORM
COPY ./copy-binary.sh /opt/evt/scripts/copy-binary.sh
COPY ./artifact-binaries /opt/evt/binaries
RUN chmod ug+x /opt/evt/scripts/copy-binary.sh
RUN /opt/evt/scripts/copy-binary.sh

FROM alpine:3.21.2
ARG UID=1000
ARG GID=1000
RUN addgroup --gid "${GID}" evt \
  && adduser --uid "${UID}" --ingroup evt --shell /sbin/nologin --disabled-password --no-create-home evt
USER evt
COPY --from=build --chown=evt /usr/local/bin/email-view-tracker /usr/local/bin/email-view-tracker
ENTRYPOINT [ "/usr/local/bin/email-view-tracker" ]
