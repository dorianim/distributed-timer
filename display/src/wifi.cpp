
#include <Arduino.h>
#include <FS.h>
#include <LittleFS.h>
#include <WiFiManager.h>

#include "wifi.h"

namespace wifi {
Display *_display;

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
  if (!fs.exists(path)) {
    Serial.println("- file does not exist");
    return String();
  }
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
  id.replace("display-", "");
  _display->printWifiSetup(id);
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

bool init(Display *display, bool reset) {
  _display = display;

  display->printLoading();

  if (reset) {
    Serial.println("Resetting wifi settings");
    _wifiManager.resetSettings();
    LittleFS.format();
  }

  if (!LittleFS.begin(true)) {
    Serial.println("LittleFS mount failed");
    return false;
  }

  _timerId = _readFile(LittleFS, "/timerId");
  _timerIdParam.setValue(_timerId.c_str(), 32);
  Serial.println("TimerId from FS: '" + _timerId + "'");

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