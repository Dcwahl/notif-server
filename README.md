# notif-server

Notification server running on Raspberry Pi. Responds to pings from your computer when on home wifi.

## Monitoring

Health check endpoint: `http://<pi-ip>:3000/health`

Returns 200 OK with status, uptime, and DB connection info.

## Development

```bash
cargo run
```

## Deployment to Raspberry Pi

### First-time setup

```bash
# Clone and build
cd /home/pi
git clone <your-repo-url> notif-server
cd notif-server
cargo build --release

# Install systemd service
sudo cp notif-server.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable notif-server
sudo systemctl start notif-server

# Verify it's running
sudo systemctl status notif-server
```

**One-liner:**
```bash
cd /home/pi/notif-server && cargo build --release && sudo cp notif-server.service /etc/systemd/system/ && sudo systemctl daemon-reload && sudo systemctl enable notif-server && sudo systemctl start notif-server
```

### After updates

```bash
cd /home/pi/notif-server
git pull
cargo build --release
sudo systemctl restart notif-server
```

**One-liner:**
```bash
cd /home/pi/notif-server && git pull && cargo build --release && sudo systemctl restart notif-server
```

### View logs

```bash
# Follow logs in real-time
sudo journalctl -u notif-server -f

# View recent logs
sudo journalctl -u notif-server -n 50
```

### Useful commands

```bash
# Check service status
sudo systemctl status notif-server

# Stop the service
sudo systemctl stop notif-server

# Restart the service
sudo systemctl restart notif-server

# Disable auto-start on boot
sudo systemctl disable notif-server
```
