# Hailstone

Project Hailstone is a system to test benchmark different middleware.
At its core, there is a hailstone sequence generator.
Those numbers are pushed through a NATS connection.
Both the sender and the receiver log their times of sending and receiving.
It exports these times as well as a basic heartbeat to Prometheus via the C API.

The hailstone sequence is tweakable with arguments
The client can be controlled via signals once launched.
