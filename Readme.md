progress-to-keyboard
============

![Animation](/Animation.gif)

## Info
Show a progression of current track of Winamp on Logitech RGB Keyboard, if Winamp not playing, a "k2000" effect will run. 

The effect will be visible on there keys : ESC, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, PRINT_SCREEN, SCROLL_LOCK, PAUSE_BREAK. All other will continue the same effect setup in you Logitech G Hub software.

This project use the Logitech SDK & this rust binding:
-  https://github.com/henninglive/logitech-led

Winapi is use to communicate with Winamp and get the basic info needed for making a progress bar (if playing/track length/ actual progression)

Tested with a Logitech G910 Orion Spark, Logitech G Hub software and Winamp 5.8.

## Build
You need to manualy download the henninglive logitech-led dependency and set it path in cargo.toml.

## Color configuration

Color can be setup directly inside the Logitech G Hub Software. Need a restart of progress-to-keyboard to take effect.

## Know Issues
- changing Logitech G hub Profile (when you start a game for example) will vanish the efffect. You can restart the progress bar or disable/enable in the Logitech G Hub. If you allways use the same profile you can make a permanent profile in Logitech G Hub.
- if your color configuration don't apply, maybe you need to increase the sleeptime in src/keyboard.rs after the driver init() (line20)

