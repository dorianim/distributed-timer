
#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <FS.h>
#include <LittleFS.h>
#include <WiFiManager.h>

#include "wifi.h"

namespace wifi {
Adafruit_NeoMatrix *_matrix;

const char *_style PROGMEM = R"CSS(
<style>
body {
  font-family: system-ui;
  background-color: #16181B;
  color: #ffffff;
}

button {
  background-color: #C4294F;
}

input {
  background-color: #292B30;
  border-color: #27292D;
  color: #ffffff !important;
}

input:focus {
  border-color: #BB274B;
  outline: 2px solid transparent;
  outline-offset: 2px;
  
}

a {
  color: #ffffff;
}

.q {
  filter: invert();
}
</style>
)CSS";

WiFiManager _wifiManager;
WiFiManagerParameter _timerIdParam("timerId", "Timer ID (requires restart)", "",
                                   32);

String _timerId;

// --- fs helpers ---
String _readFile(fs::FS &fs, const char *path) {
  Serial.printf("Reading file: %s\r\n", path);
  File file = fs.open(path, "r");
  if (!file || file.isDirectory()) {
    Serial.println("- empty file or failed to open file");
    return String();
  }
  Serial.println("- read from file:");
  String fileContent;
  while (file.available()) {
    fileContent += String((char)file.read());
  }
  file.close();
  Serial.println(fileContent);
  return fileContent;
}

void _writeFile(fs::FS &fs, const char *path, const char *message) {
  Serial.printf("Writing file: %s\r\n", path);
  File file = fs.open(path, "w");
  if (!file) {
    Serial.println("- failed to open file for writing");
    return;
  }
  if (file.print(message)) {
    Serial.println("- file written");
  } else {
    Serial.println("- write failed");
  }
  file.close();
}

// --- wifi ---
void _configModeCallback(WiFiManager *wifiManager) {
  Serial.println("Entered config mode");
  Serial.println(WiFi.softAPIP());

  String id = wifiManager->getConfigPortalSSID();
  Serial.println(id);

  // print id to display
  _matrix->clear();
  _matrix->setTextColor(_matrix->Color(255, 255, 255));
  _matrix->setCursor(9, 0);
  id.replace("display-", "");
  _matrix->print(id);
  _matrix->show();
}

String _generateId() {
  String id = WiFi.macAddress();
  id.replace(":", "");
  return id.substring(id.length() - 5);
}

void _handleSaveParams() {
  Serial.println("Saving params");
  _timerId = _timerIdParam.getValue();
  _writeFile(LittleFS, "/timerId", _timerId.c_str());
}

bool init(Adafruit_NeoMatrix *matrix, bool reset) {
  _matrix = matrix;

  _matrix->clear();
  _matrix->setTextColor(_matrix->Color(255, 255, 255));
  _matrix->setCursor(15, 0);
  _matrix->print("...");
  _matrix->show();

  if (!LittleFS.begin()) {
    Serial.println("LittleFS mount failed");
    return false;
  }

  _timerId = _readFile(LittleFS, "/timerId");
  _timerIdParam.setValue(_timerId.c_str(), 32);
  Serial.println("TimerId from FS: '" + _timerId + "'");

  if (reset) {
    Serial.println("Resetting wifi settings");
    _wifiManager.resetSettings();
  }

  _wifiManager.addParameter(&_timerIdParam);
  _wifiManager.setAPCallback(_configModeCallback);
  _wifiManager.setSaveParamsCallback(_handleSaveParams);
  _wifiManager.setParamsPage(true);
  _wifiManager.setTitle("Configure Timer Display");
  _wifiManager.setCustomHeadElement(_style);

  String id = "display-" + _generateId();
  _wifiManager.setHostname(id.c_str());
  return _wifiManager.autoConnect(id.c_str());
}

void loop() {
  if (WiFi.status() != WL_CONNECTED) {
    return;
  }

  if (!_wifiManager.getWebPortalActive()) {
    _wifiManager.startWebPortal();
  } else {
    _wifiManager.process();
  }
}
bool connected() { return WiFi.status() == WL_CONNECTED; }
String timerId() { return _timerId; }

} // namespace wifi