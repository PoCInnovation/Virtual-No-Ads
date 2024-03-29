# Virtual No Ads

Virtual No Ads is a project aimed at providing a more streamlined and pleasant browsing experience by filtering out unwanted ads from websites. It consists of two main parts:

1) **Private WiFi network with Raspberry Pi:** Using a Raspberry Pi, Virtual No Ads creates a private WiFi network that filters out ad packages from websites. This means that any device connected to this network will have ads removed before they even reach the user's device, resulting in a faster and more enjoyable browsing experience.

2) **Adblock software:** Virtual No Ads also provides a software component that can be installed on devices directly. This software functions as a traditional adblocker, filtering out ads as they are loaded on websites. The software is designed to be lightweight and easy to use, and can be customized to allow or block specific types of ads.

## How does it work?

As we connect a Raspberry Pi to Wi-Fi, it will intercept packets using the WireGuard protocol while filtering these packets to remove or sort ads; eventually emitting a Wi-Fi signal. Users can connect to this Wi-Fi network with any device, ensuring a secure network (thanks to WireGuard) and ad-free/ filtered content.

## Getting Started

### Installation

# Raspberry Pi

Clone this repository

```
git clone git@github.com:PoCInnovation/Virtual-No-Ads.git
```

Follow the instructions within the raspi [directory](<https://github.com/PoCInnovation/Virtual-No-Ads/tree/main/raspi>)

Then, if you want to create a executable, simply run the following command:

```
cargo build
```

The executable can be found at the following path: `target/debug/virtual-no-ads(.exe)`

### Quickstart

If you simply want to start the application without compiling it, run the following command:

```
cargo run <arguments>
```

### Usage

```
./virtual-no-ads <interface> <domain_list>...
```

Once Virtual No Ads is up and running, you can start browsing the web with fewer ads. If you encounter any issues or have questions about the project, please refer to the documentation or contact the project manager.

## Get involved

You're invited to join this project ! Check out the [contributing guide](./CONTRIBUTING.md).

If you're interested in how the project is organized at a higher level, please contact the current project manager.

## Our PoC team ❤️

Developers
| [<img src="https://github.com/Baragouin.png?size=85" width=85><br><sub>Timothé MEDICO</sub>](https://github.com/Baragouin) | [<img src="https://github.com/louonezime.png?size=85" width=85><br><sub>Lou ONÉZIME</sub>](https://github.com/louonezime) | [<img src="https://github.com/Igoatyouu.png?size=85" width=85><br><sub>Axel TACHEAU</sub>](https://github.com/Igoatyouu)
| :---: | :---: | :---: |

Manager
| [<img src="https://github.com/lennyvong.png?size=85" width=85><br><sub>Lenny VONGPHOUTHONE</sub>](https://github.com/lennyvong)
| :---: |

<h2 align=center>
Organization
</h2>

<p align='center'>
    <a href="https://www.linkedin.com/company/pocinnovation/mycompany/">
        <img src="https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn logo">
    </a>
    <a href="https://www.instagram.com/pocinnovation/">
        <img src="https://img.shields.io/badge/Instagram-E4405F?style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram logo"
>
    </a>
    <a href="https://twitter.com/PoCInnovation">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white" alt="Twitter logo">
    </a>
    <a href="https://discord.com/invite/Yqq2ADGDS7">
        <img src="https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white" alt="Discord logo">
    </a>
</p>
<p align=center>
    <a href="https://www.poc-innovation.fr/">
        <img src="https://img.shields.io/badge/WebSite-1a2b6d?style=for-the-badge&logo=GitHub Sponsors&logoColor=white" alt="Website logo">
    </a>
</p>

> 🚀 Don't hesitate to follow us on our different networks, and put a star 🌟 on `PoC's` repositories

> Made with ❤️ by PoC
