# Caddy metrics
My naive caddy metrics exporter for prometheus.

## Why
[this](https://github.com/caddyserver/caddy/issues/3390), [this](https://github.com/caddyserver/caddy/issues/3784), and [this](https://github.com/caddyserver/caddy/issues/4016)

## Helpers
Simulate a log entry append from caddy
```bash
echo "{\"level\":\"info\",\"ts\":1692530118.2489564,\"logger\":\"http.log.access.log1\",\"msg\":\"handled request\",\"request\":{\"method\":\"POST\",\"host\":\"i.waalrus.xyz\",\"uri\":\"/upload\"},\"bytes_read\":0,\"user_id\":\"\",\"duration\":0.0013632,\"size\":12640,\"status\":404}" >> sample.log
```

## Defining a matcher
go to [matchers.rs](./src/matchers.rs) and define them like shown in `init`.


## Setup
### Cloning
- `git clone https://github.com/mWalrus/caddy-metrics /usr/local/caddy-metrics`

### Logging
Add this to each of your host blocks in your Caddyfile:
```
log {
  output file /var/log/caddy/requests.log
  format json
}  
```

## Run
Run `./install.sh` in the project root.
