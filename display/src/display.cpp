#include "display.h"

Display *Display::from(Adafruit_NeoMatrix matrix) {
  return new NeoMatrix_Display(matrix);
}

Display *Display::from(MatrixPanel_I2S_DMA matrix) {
  return new Hub75_Display(matrix);
}

/*
 * Hub75_Display
 */

Hub75_Display::Hub75_Display(MatrixPanel_I2S_DMA matrix) : matrix(matrix) {}

void Hub75_Display::setup() {
  matrix.begin();
  matrix.setBrightness8(250);
}
void Hub75_Display::printLoading() {
  matrix.clearScreen();
  setTextColor(0xffffff);
  matrix.print("...");
  matrix.flipDMABuffer();
}
void Hub75_Display::printWifiSetup(String id) {
  matrix.clearScreen();
  setTextColor(0xffffff);
  matrix.println("Setup WiFi!");
  matrix.print(id);
  matrix.flipDMABuffer();
}
void Hub75_Display::printError(int error) {
  matrix.clearScreen();
  setTextColor(0xffffff);
  matrix.print("E:");
  matrix.print(error);
  matrix.flipDMABuffer();
}
void Hub75_Display::printTimer(timer::ActiveSegment segment) {
  matrix.clearScreen();
  setTextColor(segment.color);
  matrix.setTextSize(2);
  matrix.setTextWrap(false);
  matrix.setCursor(2, 24);
  _printPadded(segment.seconds / 60);

  matrix.print(":");
  _printPadded(segment.seconds % 60);
  matrix.flipDMABuffer();
}

void Hub75_Display::setTextColor(uint32_t c) {
  setTextColor(c >> 16, c >> 8, c);
}
void Hub75_Display::setTextColor(uint8_t r, uint8_t g, uint8_t b) {
  uint16_t packed =
      ((uint16_t)(r & 0xF8) << 8) | ((uint16_t)(g & 0xFC) << 3) | (b >> 3);

  matrix.setTextColor(packed);
}

void Hub75_Display::_printPadded(unsigned long number) {
  if (number < 10) {
    matrix.print("0");
  }

  matrix.print(number);
}

/*
 * NeoMatrix_Display
 */

NeoMatrix_Display::NeoMatrix_Display(Adafruit_NeoMatrix matrix)
    : matrix(matrix) {}

void NeoMatrix_Display::setup() {
  matrix.begin();
  matrix.setTextWrap(false);
  matrix.setBrightness(100);
  matrix.setTextColor(matrix.Color(0, 0, 255));
}

void NeoMatrix_Display::printLoading() {
  matrix.clear();
  matrix.setTextColor(matrix.Color(255, 255, 255));
  matrix.setCursor(15, 0);
  matrix.print("...");
  matrix.show();
}

void NeoMatrix_Display::printWifiSetup(String id) {
  matrix.clear();
  setTextColor(0xffffff);
  matrix.setCursor(9, 0);
  matrix.print(id);
  matrix.show();
}

void NeoMatrix_Display::printError(int error) {
  matrix.clear();
  matrix.setCursor(12, 0);
  matrix.setTextColor(matrix.Color(255, 0, 0));
  matrix.print("E");
  matrix.print(error);
  matrix.show();
}

void NeoMatrix_Display::printTimer(timer::ActiveSegment segment) {
  matrix.clear();
  matrix.setCursor(9, 0);
  setTextColor(segment.color);
  _printPadded(segment.seconds / 60);
  matrix.print(":");
  _printPadded(segment.seconds % 60);
  matrix.show();
}

void NeoMatrix_Display::setTextColor(uint32_t c) {
  setTextColor(c >> 16, c >> 8, c);
}

void NeoMatrix_Display::setTextColor(uint8_t r, uint8_t g, uint8_t b) {
  matrix.setTextColor(matrix.Color(r, g, b));
}

void NeoMatrix_Display::_printPadded(unsigned long number) {
  if (number < 10) {
    matrix.print("0");
  }

  matrix.print(number);
}