# Celery-RS Worker

A rust celery worker for building tasks around other celery implementations.

## Current Support

A Kombu-like port will need to be written for this library. Until then, RabbitMQ and 
anything supporting AMQP will work as a broker. Backends use the BackendConfig.

I really need this library for an existing time-sensitive project though.

## Features

Existing features included in release 0.1:

    - SSL support
    - AMQP/RabbitMQ broker support
    - Elastic Search backend Support
    - Redis backend support
    - Client and Workers
    - Routing Key Support
    - Message protocol support and serialization
    - Identification matching Celery
    - Threadable connections in worker ;)
    - Tolio support in the clinet ;)
    
Features to include later (0.2+):

    - Redis Broker support
    - SQS Broker Support
    - Creation of a messaging framework like Kombu
    - All other backends
    - OAuth2.0 support (RabbitMQ, Elasticsearch)
    - monitoring support (PRIORITY)
    - celerybeat support (PRIORITY)
    - Rust implemented LevelDB Broker
    - 1 to 1 feature matching with celery and maybe some extras

Sorry guys, I am one man on a very specific mission.

## License

Copyright 2019- Andrew Evans

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.