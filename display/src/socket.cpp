#include "socket.h"

#include "esp_websocket_client.h"

#include <ArduinoJson.h>

static const char *TAG = "WEBSOCKET";
#define NO_DATA_TIMEOUT_SEC 10

struct SpiRamAllocator
{
  void *allocate(size_t size)
  {
    return heap_caps_malloc(size, MALLOC_CAP_SPIRAM);
  }

  void deallocate(void *pointer)
  {
    heap_caps_free(pointer);
  }

  void *reallocate(void *ptr, size_t new_size)
  {
    return heap_caps_realloc(ptr, new_size, MALLOC_CAP_SPIRAM);
  }
};

using SpiRamJsonDocument = BasicJsonDocument<SpiRamAllocator>;

namespace WebSocket
{

  SpiRamJsonDocument *_doc;
  String _timerId;

  esp_websocket_client_config_t _websocketConfig;
  esp_websocket_client_handle_t _websocketClient;

  // --- state ---
  TIME _currentOffset = 0;
  TIME _currentFluctuation = 0;
  TIME _latestOffsets[10] = {0};
  TIME _latestFluctuations[10] = {0};

  unsigned long _lastGetTimeSent = 0;
  unsigned long _lastGetTimeReceived = 0;
  int _error = 0;
  timer::TimerData *_timerData;

  void _resetTimerData()
  {
    _timerData->valid = false;
    for (int i = 0; i < 10; i++)
    {
      _timerData->segments[i].valid = false;
    }
  }

  TIME _pushValueAndCaluclateAverage(TIME values[10], TIME newValue)
  {
    for (int i = 0; i < 9; i++)
    {
      values[i] = values[i + 1];
    }
    values[9] = newValue;

    TIME sum = 0;
    int validOffsets = 0;

    for (int i = 0; i < 10; i++)
    {
      sum += values[i];
      if (values[i] != 0)
      {
        validOffsets++;
      }
    }

    return sum / validOffsets;
  };

  int _countValidValues(TIME values[10])
  {
    int validValues = 0;
    for (int i = 0; i < 10; i++)
    {
      if (values[i] != 0)
      {
        validValues++;
      }
    }
    return validValues;
  }

  bool _isOffsetInMargin(TIME offset)
  {
    if (_currentOffset == 0)
      return true;

    TIME fluctuation = abs((long long)_currentOffset - (long long)offset);

    if (_currentFluctuation != 0 && _countValidValues(_latestFluctuations) > 5 &&
        fluctuation > _currentFluctuation * 4)
    {
      return false;
    }

    _currentFluctuation =
        _pushValueAndCaluclateAverage(_latestFluctuations, fluctuation);

    if (_countValidValues(_latestFluctuations) < 10)
    {
      return true;
    }

    return fluctuation < _currentFluctuation * 2;
  }

  void _handleNewOffset(TIME offset)
  {
    if (!_isOffsetInMargin(offset))
    {
      return;
    }

    _currentOffset = _pushValueAndCaluclateAverage(_latestOffsets, offset);
  }

  uint32_t _parseHexColor(const char *c)
  {
    uint32_t color = 0;
    // start at 1 to skip #
    for (int i = 1; i < 7; i++)
    {
      color <<= 4;
      if (c[i] >= '0' && c[i] <= '9')
      {
        color |= c[i] - '0';
      }
      else if (c[i] >= 'A' && c[i] <= 'F')
      {
        color |= c[i] - 'A' + 10;
      }
      else if (c[i] >= 'a' && c[i] <= 'f')
      {
        color |= c[i] - 'a' + 10;
      }
    }
    return color;
  }

  void _loadDisplayOptions(JsonObject &data)
  {
    String pre_start_behaviour = data["pre_start_behaviour"];
    if (pre_start_behaviour == "ShowLastSegment")
    {
      _timerData->display_options.pre_start_behaviour =
          timer::PreStartBehaviour::SHOW_LAST_SEGMENT;
    }
    else if (pre_start_behaviour == "RunNormally")
    {
      _timerData->display_options.pre_start_behaviour =
          timer::PreStartBehaviour::RUN_NORMALLY;
    }
    else
    {
      _timerData->display_options.pre_start_behaviour =
          timer::PreStartBehaviour::SHOW_FIRST_SEGMENT;
    }

    _timerData->display_options.clock = data["clock"];
  }

  void _loadTimerData(JsonObject &data)
  {
    _resetTimerData();

    if (data["segments"].size() > 10)
    {
      return;
    }
    _timerData->valid = true;
    _timerData->repeat = data["repeat"];
    _timerData->start_at = data["start_at"].as<TIME>();

    if (data["stop_at"].isNull())
    {
      _timerData->stop_at = 0;
    }
    else
    {
      _timerData->stop_at = data["stop_at"].as<TIME>();
    }

    JsonObject display_options = data["display_options"];
    _loadDisplayOptions(display_options);

    JsonArray segments = data["segments"];
    for (size_t i = 0; i < segments.size() && i < 10; i++)
    {
      JsonObject segment = segments[i];
      _timerData->segments[i].valid = true;
      _timerData->segments[i].time = segment["time"].as<TIME>();
      _timerData->segments[i].count_to = segment["count_to"].as<TIME>();
      _timerData->segments[i].sound = segment["sound"];
      strncpy(_timerData->segments[i].label, segment["label"].as<const char *>(),
              32);

      if (segment["color"].isNull())
      {
        _timerData->segments[i].color = 0xffffff;
      }
      else
      {
        _timerData->segments[i].color = _parseHexColor(segment["color"]);
      }
    }
  }

  void _handleMessage(JsonDocument &doc)
  {
    String type = doc["type"];
    if (type == "Timer")
    {
      JsonObject data = doc["data"];
      _loadTimerData(data);
    }
    else if (type == "Timestamp")
    {
      _lastGetTimeReceived = millis();
      unsigned long getTimeRoundtrip = _lastGetTimeReceived - _lastGetTimeSent;
      Serial.printf("Roundtrip: %dms\n", getTimeRoundtrip);

      long long serverTime = doc["data"];
      // TODO: FIXME!
      TIME timeOffset =
          // (serverTime - 250) + (getTimeRoundtrip / 2) - _lastGetTimeReceived;
          (serverTime - 125) + (getTimeRoundtrip / 2) - _lastGetTimeReceived;
      _handleNewOffset(timeOffset);
    }
    else if (type == "Error")
    {
      _error = doc["data"][0];
    }
  }

  static void handleNoDataTimeout(TimerHandle_t xTimer)
  {
    ESP_LOGI(TAG, "No data received for %d seconds, signaling shutdown",
             NO_DATA_TIMEOUT_SEC);

    esp_websocket_client_close(_websocketClient, portTICK_PERIOD_MS * 2000);
    esp_websocket_client_stop(_websocketClient);

    esp_websocket_client_start(_websocketClient);
  }

  static void websocket_event_handler(void *handler_args, esp_event_base_t base,
                                      int32_t event_id, void *event_data)
  {
    esp_websocket_event_data_t *data = (esp_websocket_event_data_t *)event_data;
    switch (event_id)
    {
    case WEBSOCKET_EVENT_CONNECTED:
    {
      Serial.println("Connected!");
      StaticJsonDocument<64> doc;
      doc["data"] = _timerId;
      doc["type"] = "Hello";

      String output;
      serializeJson(doc, output);

      esp_websocket_client_send_text(_websocketClient, output.c_str(),
                                     output.length(), portMAX_DELAY);
      break;
    }
    case WEBSOCKET_EVENT_DISCONNECTED:
    {
      Serial.println("WEBSOCKET_EVENT_DISCONNECTED");
      break;
    }
    case WEBSOCKET_EVENT_DATA:
    {
      if (data->op_code != 1)
      {
        return;
      }

      // ESP_LOGI(TAG, "Received %s", data->data_ptr);

      _doc->clear();
      DeserializationError error =
          deserializeJson(*_doc, data->data_ptr, data->data_len);
      if (error)
      {
        Serial.print(F("deserializeJson() failed: "));
        Serial.println(error.c_str());
        return;
      }

      _handleMessage(*_doc);
      break;
    }
    case WEBSOCKET_EVENT_ERROR:
    {
      Serial.println("WEBSOCKET_EVENT_ERROR");
      break;
    }
    }
  }

  void init(String timerId)
  {
    _timerData = timer::timerData();
    timerId.replace(" ", "");
    _timerId = timerId;

    _doc = new SpiRamJsonDocument(4096);

    _resetTimerData();

    _websocketConfig.host = "timer.itsblue.de";
    _websocketConfig.path = "/api/ws";
    _websocketConfig.port = 80;
    _websocketConfig.disable_auto_reconnect = false;
    _websocketConfig.disable_pingpong_discon = true;
    _websocketConfig.keep_alive_enable = false;
    _websocketConfig.transport = WEBSOCKET_TRANSPORT_OVER_TCP;
    _websocketConfig.cert_pem = NULL;
    _websocketConfig.cert_len = 0;
    _websocketConfig.buffer_size = 4096;

    _websocketClient = esp_websocket_client_init(&_websocketConfig);
    esp_websocket_register_events(_websocketClient, WEBSOCKET_EVENT_ANY,
                                  websocket_event_handler,
                                  (void *)_websocketClient);
    esp_websocket_client_start(_websocketClient);
  }

  void loop()
  {
    bool lastGetTimeReceivedOneSecondAgo =
        _lastGetTimeReceived > _lastGetTimeSent &&
        millis() - _lastGetTimeReceived >= 1000;
    bool lastGetTimeReceivedTimeout = _lastGetTimeReceived < _lastGetTimeSent &&
                                      millis() - _lastGetTimeSent >= 10000;

    if (connected() &&
        (_lastGetTimeSent == 0 || lastGetTimeReceivedOneSecondAgo ||
         lastGetTimeReceivedTimeout))
    {
      _lastGetTimeSent = millis();

      String payload = "{\"type\": \"GetTime\"}";
      esp_websocket_client_send_text(_websocketClient, payload.c_str(),
                                     payload.length(), portMAX_DELAY);
    }
  }

  bool connected() { return esp_websocket_client_is_connected(_websocketClient); }
  TIME offset() { return _currentOffset; }
  int error() { return _error; }
} // namespace WebSocket