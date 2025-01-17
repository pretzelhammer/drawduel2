same as axum-ws except no splitting of websocket connection and also server sends periodic pings and auto-closes stale connections (no pongs after a while). also checks for `name` and `pass` query string on ws upgrade request, returns 400 if either is missing.

