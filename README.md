<p align="center">
	<img src="docs/images/ungeckit.png" alt="drawing" width="400"/>
</p>
	
[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#work-in-progress) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![workflow-status](https://github.com/github/docs/actions/workflows/rust.yml/badge.svg)


A naive webdriver implementation that will get you page you want both asyncronously and syncronously. Right now, you can only retrieve your page and save screenshot, yet the implementation is to come. 

The architecture works as follows. A user can create a DriverBuilder object which creates Firefox driver on build() call. In case of the user calls get() on the webpage, it initiates the session on firefox on headless mode and retrieves the webpage. You can check examples for more information.

The architecture is as follows.
![](docs/images/arch.png)

## What is to come?
Currently, I am busy with the CDP (Chrome Driver Protocol) implementation which runs with websockets to run commands on DOM objects directly.
