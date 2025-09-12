#include <mpv/client.h>
#include <stdlib.h>
#include <string.h>

mpv_event_property *get_event_property() {
  mpv_event_property *property = malloc(sizeof(mpv_event_property));

  property->data = malloc(10);
  property->name = malloc(14);

  strcpy(property->data, "some-data");
  strcpy((char*)property->name, "some-property");

  property->format = MPV_FORMAT_NODE;

  return property;
}

mpv_event *get_event() {
  mpv_event *event = malloc(sizeof(mpv_event));

  event->event_id = MPV_EVENT_FILE_LOADED;
  event->data = get_event_property();
  event->reply_userdata = 727;
  event->error = MPV_ERROR_COMMAND;

  return event;
}

void free_property(mpv_event_property *ptr) {
  free(ptr->data);
  free(ptr->name);
  free(ptr);
}

void free_event(mpv_event *ptr) {
  free_property(ptr->data);
  free(ptr);
}