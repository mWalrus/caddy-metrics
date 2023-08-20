echo "Compiling Exporter"
cargo build --release

echo "-> Copying service file"
cp "./caddy-metrics.service" /etc/systemd/system/

echo "-> Reloading systemd"
systemctl daemon-reload

echo "-> Starting api service"
systemctl start caddy-metrics && systemctl enable caddy-metrics