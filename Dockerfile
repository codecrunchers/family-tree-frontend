FROM nginx:latest
ENV FT_VER=0.0.1
WORKDIR /usr/share/nginx/html
#RUN curl -S -L https://github.com/codecrunchers/family-tree-frontend/releases/download/v0.0.1-alpha/family-tree-frontend_v0.0.1.tar.gz  | tar -xz --strip 1
COPY static/ .

