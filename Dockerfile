FROM nginx:latest
ARG RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -SL https://github.com/codecrunchers/family-tree-frontend/releases/download/v${RELEASE}/web.tar.gz \
    | tar -xz \
    && cp -r ./static/* .





