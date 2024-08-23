# PCD8544 and Nokia 5110 Display Hello World

This project uses an SPI driver to send graphical bits to the display from an ESP32 MCU.

## Components

|Part|Link|
|----|----|
|ESP32-H2-DevKitM-1|[Espressif](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32h2/esp32-h2-devkitm-1/user_guide.html)|
|Nokia 5110 Display with PCD8544 Driver|[sparkfun/Nokia5110.pdf](https://www.sparkfun.com/datasheets/LCD/Monochrome/Nokia5110.pdf)|

## Schematic

Logic and power level is 3v3 throughout so gpio pins are connected directly to LCD pins without protection resistors.

MCU can source up to 100mA sustained. LCD pack draws up to 10mA.

## ESP32-H2-DevKitM-1 PinOut

|J1 Pin|Name           |PCD8544|
|------|---------------|-------|
|1     |3v3            |VCC    |
|4     |GPIO1/FSPICS0  |SCE/CS0|
|9     |GPIO4/FSPICLK  |SCLK   |
|10    |GPIO5/FSPID    |MOSI   |

|J2 Pin|Name           |Use    |
|------|---------------|-------|
|7     |GPIO12         |RST    |
|9     |GPIO22         |D/C    |
|?     |               |LED    |

