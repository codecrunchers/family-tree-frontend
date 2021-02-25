FROM nginx:latest
ARG RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -SL https://codeload.github.com/codecrunchers/family-tree-frontend/tar.gz/v${RELEASE} \
    | tar -xz \
    && cp -r ./family-tree-frontend-${RELEASE}/static/* .





