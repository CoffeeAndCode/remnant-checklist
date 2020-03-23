#!/usr/bin/env sh

wasm-pack build --target web

cd pkg
echo "Server started at http://localhost:8000 from $PWD"
docker run -p 8000:80 --rm -v "$HOME/.profile_sources/server/custom-types.conf":/etc/nginx/conf.d/custom-types.conf:ro -v "$PWD":/usr/share/nginx/html:ro nginx:alpine
