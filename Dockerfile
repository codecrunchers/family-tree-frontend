FROM nginx:latest
ARG FTFE_FTFE_RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -SL https://github.com/codecrunchers/family-tree-frontend/releases/download/v${FTFE_RELEASE}/web.tar.gz \
    | tar -xz \
    && cp -r ./static/* .





