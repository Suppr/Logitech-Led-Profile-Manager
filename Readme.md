Logitech Led Profile Manager
============

## Info
Logitech Led Profile Manager let you control keyboard led device via a config file for Windows written in Rust. Multiple effects can be done, and assign an effect to a specific program.

Current effects :
- MultipleWave
- K2000
- Progress bar (only for Winamp for the moment)
![Animation](/Animation.gif)
- Breathe
- Fill

This project use the Logitech SDK & this rust binding:
-  https://github.com/henninglive/logitech-led

Winapi is use to : 
- communicate with Winamp and get the basic info needed for making a progress bar (if playing/track length/ actual progression)
- get program CLASS and TITLE

Tested with a Logitech G910 Orion Spark, Logitech G Hub software and Winamp 5.8.

## Configuration

Make your profile permanant in Logitech GHUB. 

Put the config.yml in your working directory or same directory if you double clic on the exe.

For the color you can use, please refer to https://github.com/sharkdp/pastel.

## Know Issues
- changing Logitech G hub Profile (when you start a game for example) will vanish the efffect. You can restart the progress bar or disable/enable in the Logitech G Hub. If you allways use the same profile you can make a permanent profile in Logitech G Hub.

## TODO
- Config file wiki
- load different keyboard layout (currently FR)
- add more effects