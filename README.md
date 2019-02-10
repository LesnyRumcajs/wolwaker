# Wolwaker
[![Build Status](https://travis-ci.com/LesnyRumcajs/wolwaker.svg?branch=master)](https://travis-ci.com/LesnyRumcajs/wolwaker)

Simple service for waking up specified machine via WOL packet with GET request. 

Typical use case when you have a single low-energy server (Raspberry PI) online and some more demanding machines (NAS) you wish to wake up remotely.

### Fedora Server 29 instructions
#### Build it
1. Clone this repo and run `cargo build --release`
1. Copy the binary from `/target/release/wolwaker` to `/usr/bin/wolwaker`

#### Create service
Create new systemd service `/etc/systemd/system/wolwaker.service` with the following content:
```
[Unit]
Description=Wolwaker service
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=<YOUR USER>
ExecStart=/usr/bin/wolwaker -v -m <MAC YOU WANT TO WAKE>

[Install]
WantedBy=multi-user.target
``` 

Then start and enable it with:
```
systemctl start wolwaker
systemctl enable wolwaker
```

#### Open the wolwaker port
Create new service in `/etc/firewalld/services/wolwaker.xml` with the following content:
```
<?xml version="1.0" encoding="utf-8"?>
<service>
  <short>Wolwaker service</short>
  <description>This option allows Wolwaker to use tcp port 3333</description>
  <port protocol="tcp" port="3333"/>
</service>
```

Then apply it with:
```
firewall-cmd --permanent --add-service=my-service
firewall-cmd --reload
```

#### Wake
`http://YOUR_IP:3333/wake` will send the magic packet.