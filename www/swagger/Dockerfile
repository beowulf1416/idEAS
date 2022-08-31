FROM swaggerapi/swagger-ui:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

COPY ./auth.json /usr/share/nginx/html/definitions/auth.json

EXPOSE 80/tcp
EXPOSE 80/udp

ENV SWAGGER_JSON_URL=/definitions/auth.json