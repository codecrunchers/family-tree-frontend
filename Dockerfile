FROM nginx:latest
ARG FTFE_RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -SL https://github.com/codecrunchers/family-tree-frontend/releases/download/${FTFE_RELEASE}/web.tar.gz \
    | tar -xz \
    && cp -r ./static/* .





