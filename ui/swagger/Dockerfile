FROM swaggerapi/swagger-ui:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

COPY ./swagger.json /usr/share/nginx/html/definitions/swagger.json

#ENV SWAGGER_JSON=./swagger.json
#ENV BASE_URL=/swagger

EXPOSE 80/tcp
EXPOSE 80/udp

#RUN apt-get update && \
#    apt-get install -y nginx

#COPY ./swagger.json /opt/erp/ui/public/www/html/swagger/swagger.json
#COPY ./nginx.conf /etc/nginx/sites-available/erpswagger.conf

#COPY ./nginx.conf /etc/nginx/sites-available/erp.conf
#RUN cd /etc/nginx/sites-enabled && \
#    ln -s /etc/nginx/sites-available/erp.conf

#CMD [ "nginx", "-g", "daemon off;" ]