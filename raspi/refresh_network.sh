#!/bin/bash

sudo systemd-resolve --flush-caches
sudo systemctl restart networking
sudo systemctl status networking