#include <Adafruit_GFX.h>
#include <Adafruit_NeoMatrix.h>
#include <Adafruit_NeoPixel.h>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <EEPROM.h>

#include "socket.h"
#include "timer.h"
#include "wifi.h"

// --- matrix conf ---
// pin the matrix is attached to
#define PIN D8

Adafruit_NeoMatrix matrix = Adafruit_NeoMatrix(
    8, 8, 6, 1, PIN,
    NEO_TILE_BOTTOM + NEO_TILE_RIGHT + NEO_TILE_ROWS + NEO_TILE_PROGRESSIVE +
        NEO_MATRIX_BOTTOM + NEO_MATRIX_RIGHT + NEO_MATRIX_ROWS +
        NEO_TILE_PROGRESSIVE,
    NEO_GRB + NEO_KHZ800);

void setup() {
  Serial.begin(115200);

  pinMode(D5, INPUT_PULLUP);

  matrix.begin();
  matrix.setTextWrap(false);
  matrix.setBrightness(100);
  matrix.setTextColor(matrix.Color(0, 0, 255));

  if (!wifi::init(&matrix, digitalRead(D5) == LOW)) {
    ESP.restart();
    delay(10000);
  }

  socket::init(wifi::timerId());
}

void printPadded(unsigned long number) {
  if (number < 10) {
    matrix.print("0");
  }

  matrix.print(number);
}

void refreshDisplay() {
  if (socket::error() > 0) {
    matrix.clear();
    matrix.setCursor(12, 0);
    matrix.setTextColor(matrix.Color(255, 0, 0));
    matrix.print("E");
    matrix.print(socket::error());
    matrix.show();
    return;
  }

  timer::ActiveSegment currentSegment =
      timer::calculateCurrentSegment(socket::timerData(), socket::offset());
  matrix.clear();
  matrix.setCursor(9, 0);
  matrix.setTextColor(matrix.Color(currentSegment.color >> 16,
                                   currentSegment.color >> 8,
                                   currentSegment.color));
  printPadded(currentSegment.seconds / 60);
  matrix.print(":");
  printPadded(currentSegment.seconds % 60);
  matrix.show();
}

void loop() {
  // put your main code here, to run repeatedly:

  wifi::loop();
  if (!wifi::connected()) {
    ESP.restart();
  }

  socket::loop();

  if (socket::error() < 0) {
    ESP.restart();
  }

  if (millis() % 100 == 0) {
    refreshDisplay();
  }
}