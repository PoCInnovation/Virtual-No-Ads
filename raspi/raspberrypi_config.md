# Raspberry Pi Config

1. Install Rasbian on your Raspberry Pi if you haven't already.


2. Install the necessary packages: (You will need internet access)

```
$ sudo apt-get update
$ sudo apt-get install hostapd dnsmasq
```


3. dnsmasq

Create a backup of the original configuration file:

```
$ sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
```

Open the dnsmasq configuration file for editing with:

```
$ sudo nano /etc/dnsmasq.conf
```

And add the following lines to the file and save the file before exiting:

```
interface=wlan0
dhcp-range=192.168.4.2,192.168.4.20,255.255.255.0,24h
domain=wlan
address=/gw.wlan/192.168.4.1
addn-hosts=/etc/hosts
```


4. hostapd

Create a new configuration file for hostapd and add the following configuration:

```
$ sudo nano /etc/hostapd/hostapd.conf
```

(Replace SSID and SSID_PWD with the desired SSID and desired passphrase (minimum of 7 characters) for the access point).
```
interface=wlan0
driver=nl80211
ssid=<SSID>
hw_mode=g
channel=7
wmm_enabled=0
macaddr_acl=0
auth_algs=1
ignore_broadcast_ssid=0
wpa=2
wpa_passphrase=<SSID_PWD>
wpa_key_mgmt=WPA-PSK
wpa_pairwise=TKIP
rsn_pairwise=CCMP
```

Update the hostapd file:

```
$ sudo nano /etc/default/hostapd
```

Find the line #DAEMON_CONF="" and update it to:
```
DAEMON_CONF="/etc/hostapd/hostapd.conf"
```


5. Network Interfaces:

Open the network interfaces file:

```
$ sudo nano /etc/network/interfaces
```

Add the following lines to set up the wlan0 interface as a static IP address:

```
auto wlan0
iface wlan0 inet static
    address 192.168.4.1
    netmask 255.255.255.0
    dns-nameservers 8.8.8.8 8.8.4.4
    wireless-channel 7
    wireless-essid <SSID>
```
(last 3 lines aren't necessary, added to avoid troubleshooting)


6. Enable IP Forwarding:

Edit the sysctl configuration file:

```
$ sudo nano /etc/sysctl.conf
```

And uncomment the line net.ipv4.ip_forward=1. (remove the # at the beginning of the line)


7. Enable NAT (Network Address Translation):

Run the following iptables commands to enable NAT (or instead run the bash script provided [here](<link>) at Virtual-No-Ads/raspi/iptables_conf.sh automatically)

```
$ sudo iptables -t nat -A POSTROUTING -o wlan0 -j MASQUERADE
$ sudo iptables -A FORWARD -i wlan0 -o eth0 -m state --state RELATED,ESTABLISHED -j ACCEPT
$ sudo iptables -A FORWARD -i eth0 -o wlan0 -j ACCEPT
```

Save the rules for reboots:
```
$ sudo sh -c "iptables-save > /etc/iptables.ipv4.nat"
```

8. Setup IP Tables

```
$ sudo nano /etc/rc.local
```

Add this line before the *exit 0* line:
```
iptables-restore < /etc/iptables.ipv4.nat
```


9. Start Services and Enable at Boot:

```
$ sudo systemctl start hostapd
$ sudo systemctl start dnsmasq
```

```
$ sudo systemctl enable hostapd
$ sudo systemctl enable dnsmasq
```

Reboot your Raspberry Pi to apply the changes:

```
$ sudo reboot
```

After, it should create a Wi-Fi access point with the SSID and passphrase you specified.