FROM swaggerapi/swagger-ui:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

COPY ./countries.json /usr/share/nginx/html/definitions/countries.json

COPY ./clients.json /usr/share/nginx/html/definitions/clients.json
COPY ./auth.json /usr/share/nginx/html/definitions/auth.json
COPY ./user.json /usr/share/nginx/html/definitions/user.json

EXPOSE 80/tcp
EXPOSE 80/udp

ENV SWAGGER_JSON_URL=/definitions/auth.json