# Celery-RS

A rust celery worker and client for building projects with using the celery protocol.

## Current Support

A Kombu-like port will need to be written for this library. Until then, RabbitMQ and 
anything supporting AMQP will work as a broker. Backends use the BackendConfig.

I really need this library for an existing time-sensitive project though.

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