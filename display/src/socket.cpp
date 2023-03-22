#include "socket.h"

#include <ArduinoJson.h>
#include <WebSocketsClient.h>

namespace socket {
WebSocketsClient _socket;
StaticJsonDocument<1024> _doc;
String _timerId;
// --- state ---
TIME _timeOffset = 0;
timer::TimerData _timerData;
TIME _latestOffsets[10] = {0};
unsigned long _lastGetTimeSent = 0;
int _error = 0;

void _resetTimerData() {
  _timerData.valid = false;
  for (int i = 0; i < 10; i++) {
    _timerData.segments[i].valid = false;
  }
}

bool _isOffsetInMargin(TIME offset) {
  if (_timeOffset == 0)
    return true;

  TIME margin = _timeOffset * 0.3;
  return offset > _timeOffset - margin && offset < _timeOffset + margin;
}

void _handleNewOffset(TIME offset) {
  if (!_isOffsetInMargin(offset)) {
    return;
  }

  for (int i = 0; i < 9; i++) {
    _latestOffsets[i] = _latestOffsets[i + 1];
  }
  _latestOffsets[9] = offset;

  TIME sum = 0;
  int validOffsets = 0;

  for (int i = 0; i < 10; i++) {
    sum += _latestOffsets[i];
    if (_latestOffsets[i] != 0) {
      validOffsets++;
    }
  }

  _timeOffset = sum / validOffsets;
  Serial.printf("New offset: %llu\n", _timeOffset);
}

uint32_t _parseHexColor(const char *c) {
  uint32_t color = 0;
  // start at 1 to skip #
  for (int i = 1; i < 7; i++) {
    color <<= 4;
    if (c[i] >= '0' && c[i] <= '9') {
      color |= c[i] - '0';
    } else if (c[i] >= 'A' && c[i] <= 'F') {
      color |= c[i] - 'A' + 10;
    } else if (c[i] >= 'a' && c[i] <= 'f') {
      color |= c[i] - 'a' + 10;
    }
  }
  return color;
}

void _handleMessage(JsonDocument &doc) {
  String type = doc["type"];
  if (type == "Timer") {
    _resetTimerData();

    JsonObject data = doc["data"];
    if (data["segments"].size() > 10) {
      return;
    }
    _timerData.valid = true;
    _timerData.repeat = data["repeat"];
    _timerData.start_at = data["start_at"].as<TIME>();

    if (data["stop_at"].isNull()) {
      _timerData.stop_at = 0;
    } else {
      _timerData.stop_at = data["stop_at"].as<TIME>();
    }

    JsonArray segments = data["segments"];
    for (size_t i = 0; i < segments.size() && i < 10; i++) {
      JsonObject segment = segments[i];
      _timerData.segments[i].valid = true;
      _timerData.segments[i].time = segment["time"].as<TIME>() + 1000;
      _timerData.segments[i].sound = segment["sound"];

      if (segment["color"].isNull()) {
        _timerData.segments[i].color = 0xffffff;
      } else {
        _timerData.segments[i].color = _parseHexColor(segment["color"]);
      }
    }
  } else if (type == "Timestamp") {
    unsigned long now = millis();
    unsigned long getTimeRoundtrip = now - _lastGetTimeSent;
    long long serverTime = doc["data"];
    TIME timeOffset = serverTime - now + getTimeRoundtrip / 2;
    _handleNewOffset(timeOffset);
  } else if (type == "Error") {
    _error = doc["data"][0];
  }
}

void _webSocketEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    Serial.printf("[WSc] Disconnected!\n");
    _error = -1;
    break;
  case WStype_CONNECTED: {
    Serial.printf("[WSc] Connected to url: %s\n", payload);

    StaticJsonDocument<64> doc;
    doc["data"] = _timerId;
    doc["type"] = "Hello";

    String output;
    serializeJson(doc, output);
    // send message to server when Connected
    _socket.sendTXT(output);
    break;
  }
  case WStype_TEXT: {
    Serial.printf("[WSc] get text: %s\n", payload);

    _doc.clear();
    DeserializationError error = deserializeJson(_doc, payload);
    if (error) {
      Serial.print(F("deserializeJson() failed: "));
      Serial.println(error.c_str());
      return;
    }

    _handleMessage(_doc);

    break;
  }
  default:
    break;
  }
}

void init(String timerId) {
  timerId.replace(" ", "");
  _timerId = timerId;

  _resetTimerData();

  _socket.beginSSL("timer.itsblue.de", 443, "/api/ws");
  _socket.onEvent(_webSocketEvent);
}

void loop() {
  _socket.loop();
  if (millis() % 1000 == 0) {
    _lastGetTimeSent = millis();
    _socket.sendTXT("{\"type\": \"GetTime\"}");
  }
}

timer::TimerData timerData() { return _timerData; }
TIME offset() { return _timeOffset; }
int error() { return _error; }
} // namespace socket