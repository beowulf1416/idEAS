FROM alpine:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

ENV TZ=Etc/UTC

RUN apk update && \
    apk add nginx && \
    adduser -D -g 'www' www && \
    mkdir -p /var/www/html/ideas/ui && \
    chown -R www:www /var/lib/nginx && \
    chown -R www:www /var/www/html/ideas/ui && \
    cp /etc/nginx/nginx.conf /etc/nginx/nginx.conf.orig

RUN sed -i 's/error_log \/var\/log\/nginx\/error.log warn;/error_log \/dev\/stderr debug;/' /etc/nginx/nginx.conf
RUN sed -i 's/access_log \/var\/log\/nginx\/access.log warn;/access_log \/dev\/stdout main;/' /etc/nginx/nginx.conf

COPY ./content/ui/* /var/www/html/ideas/ui/
COPY ./conf/ideas_ui.conf /etc/nginx/http.d/default.conf

EXPOSE 80/udp
EXPOSE 80/tcp

EXPOSE 443/udp
EXPOSE 443/tcp

CMD [ "nginx", "-g", "daemon off;" ]