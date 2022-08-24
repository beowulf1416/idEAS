#FROM ubuntu:latest
FROM alpine:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

#ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

#RUN apt-get update && \
#    apt-get install -y nginx

RUN apk update && \
    apk add nginx && \
    adduser -D -g 'www' www && \
    mkdir -p /var/www/html/saas && \
    chown -R www:www /var/lib/nginx && \
    chown -R www:www /var/www/html/saas && \
    cp /etc/nginx/nginx.conf /etc/nginx/nginx.conf.orig
    # mv /etc/nginx/nginx.conf /etc/nginx/nginx.conf.orig

RUN sed -i 's/error_log \/var\/log\/nginx\/error.log warn;/error_log \/dev\/stderr debug;/' /etc/nginx/nginx.conf
RUN sed -i 's/access_log \/var\/log\/nginx\/access.log warn;/access_log \/dev\/stdout main;/' /etc/nginx/nginx.conf

COPY ./public/www/html/saas/* /var/www/html/saas/
COPY ./ui.conf /etc/nginx/http.d/saas.conf

# COPY ./nginx.conf /etc/nginx/nginx.conf
#COPY ./nginx.conf /etc/nginx/sites-available/erp.conf

#COPY ./nginx.conf /etc/nginx/sites-available/erp.conf
#RUN cd /etc/nginx/sites-enabled && \
#RUN ln -s /etc/nginx/sites-available/erp.conf /etc/nginx/sites-enabled/erp.conf

EXPOSE 80/udp
EXPOSE 80/tcp

EXPOSE 443/udp
EXPOSE 443/tcp

CMD [ "nginx", "-g", "daemon off;" ]