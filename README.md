# logi
Unofficial Logitech Options for Linux

!!! THIS SOFTWARE IS UNDER DEVELOPMENT, PLEASE DO NOT USE !!!

## Intro
I created this project to learn `systemd`, Rust, and Linux services in general.
Please be welcome to give your thoughts and help with the project.
I hope this will be usefull for people.


## Config
Simple config using TOML file located at `.config/logi/<DEVICE-NAME>.toml`

### Format
```

```


## Compatible Devices
|     Device     | Compatible? |              Config Name               |
| -------------- | :---------: | -------------------------------------- |
|  MX Master 3   |     Yes     |      `Wireless Mouse MX Master 3`      |
|  MX Master 2S  |     Yes     |     `Wireless Mouse MX Master 2S`      |
|   MX Master    |     Yes     |       `Wireless Mouse MX Master`       |
| MX Anywhere S2 |     Yes     | `Wireless Mobile Mouse MX Anywhere 2S` |
| MX Anywhere 3  |     Yes     |            `MX Anywhere 3`             |
|  MX Vertical   |     Yes     | `MX Vertical Advanced Ergonomic Mouse` |
|    MX Ergo     |     Yes     |   `MX Ergo Multi-Device Trackball `    |
|      M720      |     Yes     |  `M720 Triathlon Multi-Device Mouse`   |
|      M590      |     Yes     |     `M585/M590 Multi-Device Mouse`     |
|      T400      |     Yes     |        `Zone Touch Mouse T400`         |
|    MX Keys     |     Yes     |      `MX Keys Wireless Keyboard`       |
|      M500s     |     Yes     |     `Advanced Corded Mouse M500s`      |


## MacOS and Windows users
Please use the official driver and app from Logitech: [Logi Options+](https://www.logitech.com/en-us/software/logi-options-plus.html).

## HID++
- [Logitech Specification for HID++ 2.0](https://lekensteyn.nl/files/logitech/logitech_hidpp_2.0_specification_draft_2012-06-04.pdf)



## Credits
Thank you [PixlOne](https://github.com/PixlOne) for amazing [logiops](https://github.com/PixlOne/logiops).










```systemctl restart logid.service```

- https://danishshakeel.me/configure-logitech-mx-master-3-on-linux-logiops/
- https://github.com/PixlOne/logiops/wiki/Configuration
- [control id from linux](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h)




