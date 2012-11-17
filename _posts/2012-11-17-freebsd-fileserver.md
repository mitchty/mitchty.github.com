---
layout: post
title: "Lets make a fileserver"
description: ""
category: fileserver
tags: [freebsd zfs nas]
---
{% include JB/setup %}

# Lets build a FreeBSD 9 fileserver!

However, we're going to do this the hard way, you know to learn. Why you ask, well because reasons. Yes you could use FreeNAS and feel free to do so. But given I live on the commandline and I like that fuzzy feeling of having done something from scratch, lets do this the hard way.

First up though, why FreeBSD? Well ZFS basically, BTRFS is fine, but to be honest I wouldn't trust it to store anything of value. Also once we have this whole thing up and running the reasons for ZFS will become clearer.

But enough rambling. Lets do this.

## Prerequisites

Well something to install on. [Here is what I built for the task.](https://docs.google.com/spreadsheet/ccc?key=0AumiAUyA0vBLdHRXa3JzbHpqSU9iVExoc0ZNRENhY2c)

I optimized for low power use over performance. Primarily because 802.11n isn't exactly fast, there isn't much need for speed in a home fileserver.

Swap out components as you see fit.

One note on the Jetway NF9E-Q77 motherboard. Its a quirky beast, to be able to get into the bios setup you have to unplug the keyboard and plug in a USB mouse, reset the system, then plug the keyboard in before the POST finishes. Rather frustrating to figure out, but other than that the board works fine.

Also get the FreeBSD 9 release image on a usb stick. Get that [here](ftp://ftp.freebsd.org/pub/FreeBSD/releases/ia64/ia64/ISO-IMAGES/9.0/). I'll assume you can figure out how to copy the image yourself.

You'll also want one more USB stick or another drive that has a copy of the script you'll use for install.

So you've got your system ready? Great, lets start our build.

## Installation

We're going to be doing this a bit oddly compared to how you might normally go about a FreeBSD install. We're doing this in an entirely scripted fashion.

Now a few caveats. I'm going to be setting this up with 512byte sectors, even though the drives work with 4k. This is intentional, the reason is ZFS with 4k sector sizes uses copious amounts of storage for metadata. To the order of 20% with checksumming and compression on based upon my copy of an older array. I'll show how you set that up later if you wish to have a look. Note that the sector size is important, once you set this up, you're done, you can't change the sector zdev sizes again, its a destroy the array and rebuild affair.

The script I used to set this whole thing up is in this gist. <script src="https://gist.github.com/2745a06dcc4a36ea1b19.js"> </script>. You'll probably want to have a look and edit anything you don't like such as the hostname or ip addresses. Or if you don't have 6 drives and want raidz6 etc... Or you don't like the names I gave things, feel free to fork it for your own nefarious purposes!

Now copy that script to your drive and lets get to work.

Boot off the FreeBSD install media and select Live CD.

<img src="../../../../../images/2012-11-17-freebsd-fileserver-image01.png" />

Note to mount up an fat32 filesystem in FreeBSD do this:

        mkdir /tmp/usb; mount -t msdosfs -o large /dev/daNpN /tmp/usb

Assuming you've modified the script to your liking, kick things off and you're ready to go.

       /bin/sh -x /tmp/usb/freebsd9-zfs-install.sh

Once its finished reboot, add a user, and finally give root a password. Feel fre to admire the new shiny install before the new car smell wears off. I'll cover more of the setup in later posts.

## Notes on 4k sector drives

Now if you wanted to install and setup ZFS as a 4k sector size device. Use this variant instead.

<script src="https://gist.github.com/8344f7ae4c5b45065ed9.js"> </script>

Enjoy!




