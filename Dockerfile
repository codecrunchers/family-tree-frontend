FROM nginx:latest
ARG RELEASE
WORKDIR /usr/share/nginx/html
RUN curl -S -L "https://github.com/codecrunchers/family-tree-frontend/archive/v${RELEASE}.tar.gz"   | tar -xz --strip 1
COPY static/ .

