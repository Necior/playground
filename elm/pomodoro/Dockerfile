FROM nginx:1.21.1

# Please refer to the `Makefile`
# TODO: multi-stage build *inside* Docker

WORKDIR /usr/share/nginx/html
COPY ./index.html ./

ENTRYPOINT ["nginx", "-g", "daemon off;"]

