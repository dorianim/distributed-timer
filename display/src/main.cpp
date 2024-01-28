#include <Arduino.h>
#include <ArduinoJson.h>
#include <EEPROM.h>

#include <ESP32-HUB75-MatrixPanel-I2S-DMA.h>

#include "display.h"
#include "socket.h"
#include "timer.h"
#include "wifi.h"

// --- matrix conf ---
// pin the matrix is attached to
#define PIN D8

/*Display *display = Display::from(Adafruit_NeoMatrix(
    8, 8, 6, 1, PIN,
    NEO_TILE_BOTTOM + NEO_TILE_RIGHT + NEO_TILE_ROWS + NEO_TILE_PROGRESSIVE +
        NEO_MATRIX_BOTTOM + NEO_MATRIX_RIGHT + NEO_MATRIX_ROWS +
        NEO_TILE_PROGRESSIVE,
    NEO_GRB + NEO_KHZ800));*/

#define R1_PIN 25
#define G1_PIN 26
#define B1_PIN 27
#define R2_PIN 14
#define G2_PIN 12
#define B2_PIN 13
#define A_PIN 23
#define B_PIN 19
#define C_PIN 5
#define D_PIN 18
#define E_PIN \
  32 // required for 1/32 scan panels, like 64x64px. Any available pin would do,
     // i.e. IO32
#define LAT_PIN 4
#define OE_PIN 15
#define CLK_PIN 22

#define WIFI_RESET_PIN 21

HUB75_I2S_CFG::i2s_pins _pins = {R1_PIN, G1_PIN, B1_PIN, R2_PIN, G2_PIN,
                                 B2_PIN, A_PIN, B_PIN, C_PIN, D_PIN,
                                 E_PIN, LAT_PIN, OE_PIN, CLK_PIN};
HUB75_I2S_CFG mxconfig(64,   // Module width
                       64,   // Module height
                       2,    // chain length
                       _pins // pin mapping,
);

Display *display;

void setup()
{
  psramInit();
  Serial.begin(115200);

  mxconfig.latch_blanking = 1;
  mxconfig.i2sspeed = HUB75_I2S_CFG::HZ_20M;
  mxconfig.driver = HUB75_I2S_CFG::FM6124;
  mxconfig.double_buff = true;
  mxconfig.setPixelColorDepthBits(4);
  display = Display::from(new MatrixPanel_I2S_DMA(mxconfig));

  pinMode(WIFI_RESET_PIN, INPUT_PULLUP);

  if (digitalRead(WIFI_RESET_PIN) == LOW)
  {
    display->printLoading("reset!");
    wifi::reset();
    delay(5000);

    display->printLoading("release button!");
    while (digitalRead(WIFI_RESET_PIN) == LOW)
    {
      delay(100);
    }

    ESP.restart();
  }

  display->printLoading("connecting wifi");
  if (!wifi::init(display))
  {
    ESP.restart();
    delay(10000);
  }

  display->printLoading("connecting socket");
  WebSocket::init(wifi::timerId());
}

void refreshDisplay()
{
  if (WebSocket::error() > 0)
  {
    display->printError(WebSocket::error());
    return;
  }

  TIME offset = WebSocket::offset();

  if (!WebSocket::connected())
  {
    return;
  }

  if (offset == 0)
  {
    display->printLoading("awaiting data");
    return;
  }

  display->printTimer(offset);
}

void loop()
{
  wifi::loop();
  if (!wifi::connected())
  {
    // ESP.restart();
  }

  WebSocket::loop();
  if (WebSocket::error() < 0)
  {
    ESP.restart();
  }

  refreshDisplay();
  delay(10);
}
