#pragma once

#include <Adafruit_NeoMatrix.h>
#include <ESP32-HUB75-MatrixPanel-I2S-DMA.h>

#include "timer.h"

class Display {
public:
  static Display *from(MatrixPanel_I2S_DMA *matrix);

  virtual void setup() = 0;
  virtual void loop() = 0;

  void printLoading(const char *text);
  void printWifiSetup(String id);
  void printError(int error);
  void printTimer(timer::ActiveSegment segment);

protected:
  Display();
  void _start();
  const char *_getErrorText();

  enum DisplayState { Loading, WifiSetupNeeded, Error, Timer };

  DisplayState _state;
  const char *_loadingText;
  String _wifiId;
  int _errorCode;
  timer::ActiveSegment _segment;

private:
  static void __loop(void *arg);
};

class Hub75_Display : public Display {
public:
  Hub75_Display(MatrixPanel_I2S_DMA *matrix);

  virtual void setup() override;
  virtual void loop() override;

private:
  void _setTextColor(uint32_t c);
  void _setTextColor(uint8_t r, uint8_t g, uint8_t b);
  void _printBrandAnimationLetter(char letter, uint8_t &brightness,
                                  uint16_t &delay);

  MatrixPanel_I2S_DMA *_matrix;

  const char *brand_letters = "Itsblue.de";
  uint8_t brand_letter_brightnesses[10] = {0xff};
  uint16_t brand_letter_delays[10] = {0};
  const uint8_t DIM_STEP = 6;
  const uint8_t DIM_TO = 130;
  const uint8_t DIM_DELAY = 150;
  const uint8_t DIM_DELAY_ININTIAL_STEP = (((0xff - 100) / DIM_STEP) / 3);
};
