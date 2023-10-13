#!/bin/bash
function config_wireguard() {
    echo "Flushing existing rules and chains..."
    sudo iptables -F
    sudo iptables -X

    echo "Setting default policies to drop all traffic..."
    sudo iptables -P INPUT DROP
    sudo iptables -P FORWARD DROP
    sudo iptables -P OUTPUT DROP

    echo "Allowing loopback traffic..."
    sudo iptables -A INPUT -i lo -j ACCEPT
    sudo iptables -A OUTPUT -o lo -j ACCEPT

    echo "Allowing incoming WireGuard and VPN traffic..."
    sudo iptables -A INPUT -i wg0 -j ACCEPT

    echo "Allowing outgoing WireGuard and VPN traffic..."
    sudo iptables -A OUTPUT -o wg0 -j ACCEPT

    echo "Allowing traffic for Wi-Fi access point..."
    sudo iptables -t nat -A POSTROUTING -o wlan0 -j MASQUERADE
    sudo iptables -A FORWARD -i wlan0 -o eth0 -m state --state RELATED,ESTABLISHED -j ACCEPT
    sudo iptables -A FORWARD -i eth0 -o wlan0 -j ACCEPT

    echo "Allowing established and related connections..."
    sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
    sudo iptables -A OUTPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT

    echo "Setting up NAT for WireGuard interface..."
    sudo iptables -t nat -A POSTROUTING -o wg0 -j MASQUERADE

}

if ["$1" == "wireguard"]; then
    config_wireguard
elif ["$2" == "signal-only"]; then
    echo "Enabling IP Masquerading..."
    sudo iptables -t nat -A POSTROUTING -o wlan0 -j MASQUERADE
    sudo iptables -A FORWARD -i wlan0 -o eth0 -m state --state RELATED,ESTABLISHED -j ACCEPT
    sudo iptables -A FORWARD -i eth0 -o wlan0 -j ACCEPT
else
    echo "USAGE:\tsudo bash iptables_conf.sh [opt]"
    echo "\n[opt] = wireguard or signal-only depending if you want to configure WireGuard"
    exit 1
fi

echo "sudo sh -c "iptables-save > /etc/iptables/rules.v4"