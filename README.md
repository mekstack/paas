# SaaS

Service as a Service services

## Installation

Python dependencies are tracked with poetry. To install dependencies run

    poetry install
    poetry update

## Microservices

### Auth

Provides an HTTP endpoint "/login/<provider>" that authorizes user via OAuth provider and returns a signed JWT token.

## Development

To init a new service do

    poetry new <service name>
