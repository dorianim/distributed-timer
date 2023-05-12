#include "WiFi.h"

#include "display.h"
#include "time.h"
#include "wifi.h"

#include "advent_pro_regular.h"

Display::Display() : _state(Loading), _loadingText("starting up") {}

void Display::_start() {
  xTaskCreate(Display::__loop, "display", 10000, this, 10, NULL);
}

const char *Display::_getErrorText() {
  switch (this->_errorCode) {
  case 404:
    return "Timer not found";
  default:
    return "";
  }
}

void Display::__loop(void *arg) {
  Display *that = (Display *)arg;
  that->setup();

  for (;;) {
    that->loop();
  }
}

Display *Display::from(MatrixPanel_I2S_DMA *matrix) {
  return new Hub75_Display(matrix);
}

void Display::printLoading(const char *text) {
  this->_loadingText = text;
  this->_state = Loading;
}
void Display::printWifiSetup(String id) {
  this->_wifiId = id;
  this->_state = WifiSetupNeeded;
}
void Display::printError(int error) {
  this->_errorCode = error;
  this->_state = Error;
}
void Display::printTimer(timer::ActiveSegment segment) {
  this->_segment = segment;
  this->_state = Timer;
}

/*
 * Hub75_Display
 */

Hub75_Display::Hub75_Display(MatrixPanel_I2S_DMA *matrix)
    : Display(), _matrix(matrix) {

  _matrix->begin();
  _matrix->clearScreen();
  _matrix->flipDMABuffer();
  _matrix->setBrightness8(250);

  for (int i = 0; i < 10; i++) {
    brand_letter_delays[i] = i * DIM_DELAY_ININTIAL_STEP;
  }

  this->_start();
}

void Hub75_Display::setup() {}

void Hub75_Display::loop() {
  _matrix->clearScreen();
  _matrix->setCursor(0, 0);

  switch (this->_state) {
  case Loading: {
    _matrix->setFont(&AdventPro_Regular18pt7b);
    _matrix->setTextSize(1);
    _matrix->setCursor(4, 40);
    for (int i = 0; i < 10; i++) {
      _printBrandAnimationLetter(brand_letters[i], brand_letter_brightnesses[i],
                                 brand_letter_delays[i]);
    }

    _matrix->setFont();
    _setTextColor(0xffffff);
    uint8_t textlen = strlen(_loadingText);
    _matrix->setCursor((128 - textlen * 6) / 2, 54);
    _matrix->print(_loadingText);
    break;
  }
  case WifiSetupNeeded:
    _setTextColor(0xaaaaaa);
    _matrix->setTextSize(1);
    _matrix->setCursor(0, 0);
    _matrix->println("Please setup:");
    _matrix->println("1. Connect to WiFi");
    _setTextColor(0x0000ff);
    _matrix->println("   display-" + this->_wifiId);
    _setTextColor(0xBB274B);
    _matrix->println("2. Setup");
    _setTextColor(0xaaaaaa);
    _matrix->println("3. enter Timer ID");
    _setTextColor(0xBB274B);
    _matrix->println("4. Go back");
    _matrix->println("5. Configure WiFi");
    break;
  case Error: {
    _setTextColor(0xffffff);
    _matrix->setCursor(49, 3);
    _matrix->print("ERROR");

    _setTextColor(0xff0000);
    _matrix->setTextSize(4);
    _matrix->setCursor(28, 15);
    _matrix->print(this->_errorCode);

    _matrix->setTextSize(1);
    _setTextColor(0xffffff);
    uint8_t textlen = strlen(_getErrorText());
    _matrix->setCursor((128 - textlen * 6) / 2, 47);
    _matrix->print(_getErrorText());

    String ip = WiFi.localIP().toString();
    _matrix->setCursor((128 - ip.length() * 6) / 2, 56);
    _matrix->print(WiFi.localIP().toString());
    break;
  }
  case Timer: {
    _matrix->setFont();

    _matrix->setTextSize(1);
    _setTextColor(0xff, 0xff, 0xff);
    uint8_t textlen = strlen(_segment.label);
    _matrix->setCursor((128 - textlen * 6) / 2, 3);
    _matrix->print(_segment.label);

    _setTextColor(_segment.color);
    _matrix->setTextSize(5);
    _matrix->setTextWrap(false);
    _matrix->setCursor(1, 15);
    _matrix->printf("%02d", _segment.seconds / 60);

    _matrix->setCursor(51, 15);
    _matrix->print(":");
    _matrix->setCursor(71, 15);
    _matrix->printf("%02d", _segment.seconds % 60);

    if (timer::timerData()->display_options.clock) {
      time_t timer =
          (_segment.currentTime / 1000) + ((60 * 60) * wifi::timezoneOffset());
      tm *time = localtime(&timer);
      _matrix->setTextSize(1);
      _setTextColor(0xff, 0xff, 0xff);
      _matrix->setCursor(49, 54);

      _matrix->printf("%02d:%02d", time->tm_hour, time->tm_min);
    }

    break;
  }
  }

  // if ((millis() / 1000) % 2)
  //   _matrix->drawPixel(0, 0, _matrix->color333(0xff, 0xff, 0xff));

  _matrix->flipDMABuffer();
  vTaskDelay(10);
}

void Hub75_Display::_printBrandAnimationLetter(char letter, uint8_t &brightness,
                                               uint16_t &delay) {
  _setTextColor(0, 0, brightness);
  _matrix->print(letter);
  if (brightness > DIM_TO && delay == 0) {
    brightness -= DIM_STEP;
  } else if (brightness <= DIM_TO && delay == 0) {
    delay = DIM_DELAY;
  } else if (brightness < 0xff && delay == DIM_DELAY &&
             0xff - brightness > DIM_STEP) {
    brightness += DIM_STEP;
  } else if (brightness < 0xff && delay == DIM_DELAY) {
    brightness = 0xff;
  } else {
    delay--;
  }
}

void Hub75_Display::_setTextColor(uint32_t c) {
  _setTextColor(c >> 16, c >> 8, c);
}
void Hub75_Display::_setTextColor(uint8_t r, uint8_t g, uint8_t b) {
  uint16_t packed =
      ((uint16_t)(r & 0xF8) << 8) | ((uint16_t)(g & 0xFC) << 3) | (b >> 3);

  _matrix->setTextColor(packed);
}