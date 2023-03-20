#include <Adafruit_GFX.h>
#include <Adafruit_NeoMatrix.h>
#include <Adafruit_NeoPixel.h>
#include <Arduino.h>
#include <ArduinoJson.h>
#include <EEPROM.h>
#include <ESP8266WiFi.h>

#include <WebSocketsClient.h>

// --- matrix conf ---
// pin the matrix is attached to
#define PIN D8
#define TIME unsigned long long

Adafruit_NeoMatrix matrix = Adafruit_NeoMatrix(
    8, 8, 6, 1, PIN,
    NEO_TILE_BOTTOM + NEO_TILE_RIGHT + NEO_TILE_ROWS + NEO_TILE_PROGRESSIVE +
        NEO_MATRIX_BOTTOM + NEO_MATRIX_RIGHT + NEO_MATRIX_ROWS +
        NEO_TILE_PROGRESSIVE,
    NEO_GRB + NEO_KHZ800);

StaticJsonDocument<1024> doc;

struct TimerSegment {
  bool valid;
  unsigned long time;
  bool sound;
  uint32_t color;
};

struct TimerData {
  bool valid;
  bool repeat;
  TIME start_at;
  TimerSegment segments[10];
};

const char *loading_symbols[4] = {"|", "/", "-", "\\"};

// --- state ---
TIME timeOffset = 0;
TimerData timerData;
unsigned long lastGetTimeSent = 0;
TIME latestOffsets[10] = {0};
WebSocketsClient socket;

// --- socket and offset helpers ---
void resetTimerData() {
  timerData.valid = false;
  for (int i = 0; i < 10; i++) {
    timerData.segments[i].valid = false;
  }
}

bool isOffsetInMargin(TIME offset) {
  if (timeOffset == 0)
    return true;

  TIME margin = timeOffset * 0.3;
  return offset > timeOffset - margin && offset < timeOffset + margin;
}

void handleNewOffset(TIME offset) {
  if (!isOffsetInMargin(offset)) {
    return;
  }

  for (int i = 0; i < 9; i++) {
    latestOffsets[i] = latestOffsets[i + 1];
  }
  latestOffsets[9] = offset;

  TIME sum = 0;
  int validOffsets = 0;

  for (int i = 0; i < 10; i++) {
    sum += latestOffsets[i];
    if (latestOffsets[i] != 0) {
      validOffsets++;
    }
  }

  timeOffset = sum / validOffsets;
  Serial.printf("New offset: %llu\n", timeOffset);
}

uint32_t parseHexColor(const char *c) {
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

void handleMessage(JsonDocument &doc) {
  String type = doc["type"];
  if (type == "Timer") {
    resetTimerData();

    JsonObject data = doc["data"];
    if (data["segments"].size() > 10) {
      return;
    }
    timerData.valid = true;
    timerData.repeat = data["repeat"];
    timerData.start_at = data["start_at"].as<TIME>();

    JsonArray segments = data["segments"];
    for (size_t i = 0; i < segments.size() && i < 10; i++) {
      JsonObject segment = segments[i];
      timerData.segments[i].valid = true;
      timerData.segments[i].time = segment["time"].as<TIME>() + 1000;
      timerData.segments[i].sound = segment["sound"];

      if (segment["color"].isNull()) {
        timerData.segments[i].color = 0xffffff;
      } else {
        timerData.segments[i].color = parseHexColor(segment["color"]);
      }
    }
  } else if (type == "Timestamp") {
    unsigned long now = millis();
    unsigned long getTimeRoundtrip = now - lastGetTimeSent;
    long long serverTime = doc["data"];
    TIME timeOffset = serverTime - now + getTimeRoundtrip / 2;
    handleNewOffset(timeOffset);
  }
}

void webSocketEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    Serial.printf("[WSc] Disconnected!\n");
    break;
  case WStype_CONNECTED: {
    Serial.printf("[WSc] Connected to url: %s\n", payload);

    // send message to server when Connected
    socket.sendTXT("{\"data\": \"test\", \"type\": \"Hello\"}");
  } break;
  case WStype_TEXT:
    Serial.printf("[WSc] get text: %s\n", payload);

    doc.clear();
    DeserializationError error = deserializeJson(doc, payload);
    if (error) {
      Serial.print(F("deserializeJson() failed: "));
      Serial.println(error.c_str());
      return;
    }

    handleMessage(doc);

    break;
  }
}

// --- time helpers ---

struct ActiveSegment {
  TIME seconds;
  uint32_t color;
};

ActiveSegment calculateCurrentSegment() {
  if (!timerData.valid || timeOffset == 0) {
    return {0, matrix.Color(255, 255, 255)};
  }

  TIME currentTime = (TIME)millis() + timeOffset;
  if (currentTime < timerData.start_at) {
    return {0, matrix.Color(255, 255, 255)};
  }

  long elapsedTime = currentTime - timerData.start_at;

  long totalTimePerRound = 0;
  for (int i = 0; i < 10 && timerData.segments[i].valid; i++) {
    totalTimePerRound += timerData.segments[i].time;
  }

  if (!timerData.repeat && elapsedTime > totalTimePerRound) {
    return {0, matrix.Color(255, 255, 255)};
  }

  long timeInCurrentRound = elapsedTime % totalTimePerRound;

  int currentSegmentIndex = 0;
  long timeInCurrentSegment = 0;
  while (timeInCurrentRound > 0 &&
         timerData.segments[currentSegmentIndex].valid) {
    timeInCurrentSegment =
        timerData.segments[currentSegmentIndex].time - timeInCurrentRound;
    timeInCurrentRound -= timerData.segments[currentSegmentIndex].time;
    currentSegmentIndex++;
  }

  return {timeInCurrentSegment / 1000,
          timerData.segments[currentSegmentIndex - 1].color};
}

void setup() {
  Serial.begin(115200);

  resetTimerData();

  matrix.begin();
  matrix.setTextWrap(false);
  matrix.setBrightness(100);
  matrix.setTextColor(matrix.Color(0, 0, 255));

  WiFi.disconnect(true);
  WiFi.mode(WIFI_STA);
  WiFi.persistent(false);
  WiFi.begin("bla", "12345678");

  socket.beginSSL("timer.itsblue.de", 443, "/api/ws");
  socket.onEvent(webSocketEvent);
}

void printPadded(unsigned long number) {
  if (number < 10) {
    matrix.print("0");
  }

  matrix.print(number);
}

void loop() {
  // put your main code here, to run repeatedly:
  if (WiFi.status() != WL_CONNECTED) {
    // print | / - \ to show that the device is still alive using millis
    matrix.clear();
    matrix.setCursor(21, 0);
    matrix.print(loading_symbols[(millis() / 1000) % 4]);
    matrix.show();
    delay(100);
    return;
  }

  socket.loop();

  if (millis() % 1000 == 0) {
    lastGetTimeSent = millis();
    socket.sendTXT("{\"type\": \"GetTime\"}");
  }

  if (millis() % 100 == 0) {
    ActiveSegment currentSegment = calculateCurrentSegment();
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
}