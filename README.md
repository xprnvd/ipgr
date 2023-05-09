# ipgr
ge
`ipgr` (short for "IP grep") is a command-line program that extracts IPv4 and IPv6 addresses from standard input and prints them to standard output. It can be useful, where you need to quickly extract IP addresses from various sources such as log files, network traffic captures, and DNS records. 

By using `ipgr` in combination with other command-line tools, you can easily filter, sort, and manipulate the extracted IP addresses to identify potential targets for further investigation.

## Usage

ipgr [FLAGS]


### Flags

- `-4`: filter IPv4 addresses.
- `-6`: filter IPv6 addresses.
- `-pub`: filter public IP addresses.
- `-priv`: filter private IP addresses.
- `-s`: silent. only IP addresses.

## Examples

```
➜  subfinder -silent -d hackerone.com | dnsx -silent -a -resp | ipgr -s -pub
3.208.160.136
18.66.63.51
18.66.63.94
18.66.63.96
18.66.63.105
34.232.145.146
35.170.97.33
52.70.181.109
52.201.196.83
104.16.99.52
104.16.100.52
162.159.0.31
162.159.1.31
185.199.108.153
185.199.109.153
185.199.110.153
185.199.111.153
```

```
➜  cat dummy.log
2023-05-09 08:22:20 [INFO] Connection established from public IPv4: 93.184.216.34
2023-05-09 08:22:21 [DEBUG] Assigned private IPv4: 192.168.1.2
2023-05-09 08:22:22 [INFO] Connection established from public IPv6: 2606:2800:220:1:248:1893:25c8:1946
2023-05-09 08:22:23 [DEBUG] Assigned private IPv6: fd12:3456:789a::1
2023-05-09 08:22:24 [ERROR] Connection lost from public IPv4: 93.184.216.34
2023-05-09 08:22:25 [INFO] Connection re-established from public IPv4: 93.184.216.34
2023-05-09 08:22:26 [DEBUG] Assigned private IPv4: 192.168.1.3
2023-05-09 08:22:27 [WARNING] High traffic detected from public IPv6: 2606:2800:220:1:248:1893:25c8:1946
2023-05-09 08:22:28 [INFO] Traffic normalized from public IPv6: 2606:2800:220:1:248:1893:25c8:1946
2023-05-09 08:22:29 [DEBUG] Connection closed from public IPv4:<IPAddress>
```

Extract all IP addresses from a file:

```
➜  cat dummy.log | ipgr -s
93.184.216.34
192.168.1.2
192.168.1.3
2606:2800:220:1:248:1893:25c8:1946
````

Extract only IPv4 addresses from a file:

```
➜  cat dummy.log | ipgr -4
IPv4 addresses:
93.184.216.34
192.168.1.2
192.168.1.3
```

Extract only public IP addresses from a file:

```
➜  ipgr -pub -f dummy.log
IPv4 addresses:
93.184.216.34

IPv6 addresses:
2606:2800:220:1:248:1893:25c8:1946
```

## License

This program is licensed under the [MIT License](LICENSE).
