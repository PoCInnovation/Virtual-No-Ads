# Wireguard

1. Install Wireguard

```
$ sudo apt-get update
$ sudo apt install wireguard
```


2. Set Up WireGuard:

Generate WireGuard keys, a public and private key:

```
$ wg genkey | tee <PRIVATE_KEY> | wg pubkey > <PUBLIC_KEY>
```
(replace <PRIVATE_KEY> and <PUBLIC_KEY> with your desired filenames)

This command generates a private key and corresponding public key.
(You can use this to generate a private and public key pair for each client you want to add to the Wireguard VPN and for the server)


3. Create the WireGuard configuration file (like /etc/wireguard/wg0.conf):

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
(Every peer is a new client added to the Wireguard server, they should include the public of the client and specify the allowed IP addresses for the client)


4. Link clients to WireGuard VPN

Create a new configuration file for each client, and add the necessary configurations:

```
[Interface]
PrivateKey = <CLIENT_PRIVATE_KEY>
Address = 10.0.0.2/32
DNS = 8.8.8.8

[Peer]
PublicKey = <SERVER_PUBLIC_KEY>
Endpoint = <SERVER_IP_ADDR>:51820
AllowedIPs = 0.0.0.0/0, ::/0
```

(On mobile, you will have to input these configurations through the WireGuard App).


5. Configure Firewall Rules
(or instead can run the bash script provided [here](<link>) at Virtual-No-Ads/raspi/iptables_conf.sh automatically):

Flush existing rules and chains

```
$ sudo iptables -F
$ sudo iptables -X
```

Set default policies to drop all traffic

```
$ sudo iptables -P INPUT DROP
$ sudo iptables -P FORWARD DROP
$ sudo iptables -P OUTPUT DROP
```

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


6. Save the rules

```
$ sudo sh -c "iptables-save > /etc/iptables/rules.v4"
```


7. Activate the Wireguard Connection on the Clients

```
$ sudo wg-quick up wg0
```

8. Enable WireGuard on Boot
```
$ sudo systemctl enable wg-quick@your-config-file
```