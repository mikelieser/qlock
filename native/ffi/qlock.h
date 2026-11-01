// C interface of qlock-ffi (native/ffi/src/lib.rs).
#include <stdbool.h>
#include <stdint.h>

typedef struct QlockEngine QlockEngine;

typedef struct {
  uint32_t sprite;
  uint32_t group;  // 0 = screen, 1 = clipped to the clock face
  float x, y;      // sprite centre in pixels, y down
  float opacity, scale;
  float scale_y;   // extra vertical scale on top of scale
  float rotation;  // radians, clockwise
} QlockInstance;

enum {
  QLOCK_INTRO = 1,
  QLOCK_DOTS = 2,
  QLOCK_STRETCH = 4,
  QLOCK_EFFECT = 8,
  QLOCK_TRANSITION = 16,
  QLOCK_AMBIENT = 32,
  QLOCK_PROGRESS = 64,
  QLOCK_ALL = 128,
};

QlockEngine *qlock_engine_new(const char *locale, const char *theme, uint32_t width, uint32_t height,
                              uint32_t flags, const char *caption);
void qlock_engine_free(QlockEngine *engine);
uint32_t qlock_sprite_count(const QlockEngine *engine);
const uint8_t *qlock_sprite(const QlockEngine *engine, uint32_t index, uint32_t *width, uint32_t *height);
void qlock_face(const QlockEngine *engine, float *out);
uint32_t qlock_update(QlockEngine *engine, uint32_t hour, uint32_t minute, float second, double t,
                      const QlockInstance **out);
void qlock_release_pixels(QlockEngine *engine);
bool qlock_snapshot(const char *locale, const char *theme, uint32_t width, uint32_t height, uint32_t flags,
                    uint32_t hour, uint32_t minute, uint8_t *out);
