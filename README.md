# idEAS
Enterprise Application Software

Features
* Multi-tenant
* Permission and role based authorization

Components
* PostgreSQL database
* Apache Kafka enterprise bus
* Angular based front end



* Project Management
* Manufacturing
* Inventory Management / Supply Chain Management
* Asset Management
* Enterprise Resource Planning
  * Accounting
  * Human Resource
  * Sales
  * Procurement


# Installation and testing
The application can be tested using Docker. Navigate to the root folder containing the docker-compose.yml file and run:
docker-compose build
docker-compose up -d

That command will build the images and containers that is required to run a local copy of the application. It will run 4 containers: A zookeeper instance and a kafka instance, a postgres database and the application binary itself. An optional container can be built and run containing the swagger instance used for testing the endpoints provided by the application.