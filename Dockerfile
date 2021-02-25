FROM nginx:latest
ARG RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -S -L  "https://codeload.github.com/codecrunchers/family-tree-frontend/tar.gz/v${RELEASE}"  | tar -xz --strip 1
COPY ./static .




