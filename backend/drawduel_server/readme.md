# backend

i tried flatbuffers and i tried protobufs and protobufs won:
- smaller messages, easier api to work with

i tried axum ws (tokio-tungstenite under the hood), but have yet to try fastwebsockets (from deno project), atm fastwebsockets doesn't seem to work with axum 0.8, and i don't want to put in the effort to try to make it work, or worse, try to make it work with http2 or tls.

