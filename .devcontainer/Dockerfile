FROM rust:1.64.0-alpine3.16
RUN apk --no-cache add make gcc musl-dev git bash openssh sudo

# add local user
ARG USERNAME=wyubin
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN adduser -D -u $USER_UID $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
