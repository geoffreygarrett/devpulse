#!/bin/sh

# Export all environment variables for substitution
export API_DOMAIN
export REST_SERVICE_HOST
export REST_SERVICE_PORT
export GRPC_DOMAIN
export GRPC_SERVICE_HOST
export GRPC_SERVICE_PORT
export PLAYGROUND_DOMAIN
export PLAYGROUND_SERVICE_HOST
export PLAYGROUND_SERVICE_PORT

# Substitute environment variables in the template and write to nginx.conf
envsubst '${API_DOMAIN} ${REST_SERVICE_HOST} ${REST_SERVICE_PORT} ${GRPC_DOMAIN} ${GRPC_SERVICE_HOST} ${GRPC_SERVICE_PORT} ${PLAYGROUND_DOMAIN} ${PLAYGROUND_SERVICE_HOST} ${PLAYGROUND_SERVICE_PORT}' < /etc/nginx/templates/nginx.conf.conf > /etc/nginx/nginx.conf

# Execute the given command (nginx) with provided arguments
exec "$@"
