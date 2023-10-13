# Wireguard

1. Install Wireguard

```
$ sudo apt-get update
$ sudo apt install wireguard
```

2. Set Up WireGuard:

Generate WireGuard keys, a public and private key:

```
$ wg genkey | tee privatekey | wg pubkey > publickey
```

This command generates a private key and corresponding public key.

Replace privatekey and publickey with your desired filenames.

3. Create the WireGuard configuration file (e.g., /etc/wireguard/wg0.conf):

```
$ sudo nano /etc/wireguard/wg0.conf
```

Add the following content, customizing it with your keys and network settings:

```
[Interface]
Address = 10.0.0.1/24
ListenPort = 51820
PrivateKey = <SERVER_PRIVATE_KEY>

[Peer]
AllowedIPs = 10.0.0.2/32
PublicKey = <CLIENT_PUBLIC_KEY>
```

4. Configure Firewall Rules

Flush existing rules and chains

```
$ sudo iptables -F
$ sudo iptables -X

Set default policies to drop all traffic

```
$ sudo iptables -P INPUT DROP
$ sudo iptables -P FORWARD DROP
$ sudo iptables -P OUTPUT DROP

Allow loopback traffic

```
$ sudo iptables -A INPUT -i lo -j ACCEPT
$ sudo iptables -A OUTPUT -o lo -j ACCEPT
```

Allow incoming WireGuard and VPN traffic

```
$ sudo iptables -A INPUT -i wg0 -j ACCEPT
```

Allow outgoing WireGuard and VPN traffic

```
$ sudo iptables -A OUTPUT -o wg0 -j ACCEPT
```

Allow traffic for Wi-Fi access point

```
$ sudo iptables -t nat -A POSTROUTING -o wlan0 -j MASQUERADE
$ sudo iptables -A FORWARD -i wlan0 -o eth0 -m state --state RELATED,ESTABLISHED -j ACCEPT
$ sudo iptables -A FORWARD -i eth0 -o wlan0 -j ACCEPT
```

Allow established and related connections

```
$ sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
$ sudo iptables -A OUTPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
```

Set up NAT for WireGuard interface

```
$ sudo iptables -t nat -A POSTROUTING -o wg0 -j MASQUERADE
```

Save the rules

```
$ sudo sh -c "iptables-save > /etc/iptables/rules.v4"
```

5. Activate the Wireguard Connection on the Clients

```
$ sudo wg-quick up wg0
```