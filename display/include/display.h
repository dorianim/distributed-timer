#pragma once

#include <Adafruit_NeoMatrix.h>
#include <ESP32-HUB75-MatrixPanel-I2S-DMA.h>

#include "timer.h"

class Display {
public:
  static Display *from(Adafruit_NeoMatrix matrix);
  static Display *from(MatrixPanel_I2S_DMA matrix);

  virtual void setup() = 0;

  virtual void printLoading() = 0;
  virtual void printWifiSetup(String id) = 0;
  virtual void printError(int error) = 0;
  virtual void printTimer(timer::ActiveSegment segment) = 0;

  virtual void setTextColor(uint32_t c) = 0;
  virtual void setTextColor(uint8_t r, uint8_t g, uint8_t b) = 0;
};

class Hub75_Display : public Display {
public:
  Hub75_Display(MatrixPanel_I2S_DMA matrix);

  virtual void setup() override;

  void printLoading() override;
  void printWifiSetup(String id) override;
  void printError(int error) override;
  void printTimer(timer::ActiveSegment segment) override;

  void setTextColor(uint32_t c) override;
  void setTextColor(uint8_t r, uint8_t g, uint8_t b) override;

private:
  void _printPadded(unsigned long number);
  MatrixPanel_I2S_DMA matrix;
};

class NeoMatrix_Display : public Display {
public:
  NeoMatrix_Display(Adafruit_NeoMatrix matrix);

  virtual void setup() override;

  void printLoading() override;
  void printWifiSetup(String id) override;
  void printError(int error) override;
  void printTimer(timer::ActiveSegment segment) override;

  void setTextColor(uint32_t c) override;
  void setTextColor(uint8_t r, uint8_t g, uint8_t b) override;

private:
  void _printPadded(unsigned long number);
  Adafruit_NeoMatrix matrix;
};