# Welcome to rise-up
![version](https://img.shields.io/badge/version-0.1.0-blue?style=for-the-badge)
> HTTP API server for sending magic packets (wake-on-lan)

<br><br>
<p align="center">Send a magic packet by</p>
<p align="center"><img src="POST.png" align="top"/>&nbsp;&nbsp;&nbsp;<b>/rise/{mac}</b></p>
<p align="center"><i>It's as easy as that!</i></p>
<br><br>

## Description
`rise-up` is an HTTP API server for sending wake-on-lan magic packets. Once it's running, you just have to send a POST request to `/rise/{mac}` to send a magic packet to the target mac address.

Since it's written in Rust, you will need cargo to compile it.