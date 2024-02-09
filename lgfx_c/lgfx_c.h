// SPDX-License-Identifier: BSL-1.0
// Copyright Kenta Ida 2022.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

// See README.md for license details.

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum textdatum
//  0:left   1:centre   2:right
//  0:top    4:middle   8:bottom   16:baseline
{ top_left        =  0  // Top left (default)
, top_center      =  1  // Top center
, top_centre      =  1  // Top center
, top_right       =  2  // Top right
, middle_left     =  4  // Middle left
, middle_center   =  5  // Middle center
, middle_centre   =  5  // Middle center
, middle_right    =  6  // Middle right
, bottom_left     =  8  // Bottom left
, bottom_center   =  9  // Bottom center
, bottom_centre   =  9  // Bottom center
, bottom_right    = 10  // Bottom right
, baseline_left   = 16  // Baseline left (Line the 'A' character would sit on)
, baseline_center = 17  // Baseline center
, baseline_centre = 17  // Baseline center
, baseline_right  = 18  // Baseline right
} textdatum_t;

typedef enum epd_mode
{
    epd_quality = 1,
    epd_text    = 2,
    epd_fast    = 3,
    epd_fastest = 4,
} epd_mode_t;

typedef struct font_metrics
{
    int16_t width;
    int16_t x_advance;
    int16_t x_offset;
    int16_t height;
    int16_t y_advance;
    int16_t y_offset;
    int16_t baseline;
} font_metrics_t;

typedef struct lgfx_target *lgfx_target_t;

lgfx_target_t lgfx_c_setup(void);
lgfx_target_t lgfx_c_setup_with_size(int width, int height);

epd_mode_t lgfx_c_get_epd_mode(lgfx_target_t target);
void lgfx_c_set_epd_mode(lgfx_target_t target, enum epd_mode epd_mode);
bool lgfx_c_is_epd(lgfx_target_t target);
void lgfx_c_set_rotation(lgfx_target_t target, uint_fast8_t rotation);

int32_t lgfx_c_width(lgfx_target_t target);
int32_t lgfx_c_height(lgfx_target_t target);

int32_t lgfx_c_font_height(lgfx_target_t target);

void lgfx_c_clear_rgb332(lgfx_target_t target, uint8_t color);
void lgfx_c_clear_rgb888(lgfx_target_t target, uint32_t color);
void lgfx_c_fill_rect_rgb332(lgfx_target_t target, int32_t left, int32_t top, int32_t width, int32_t height, uint8_t color);
void lgfx_c_fill_rect_rgb888(lgfx_target_t target, int32_t left, int32_t top, int32_t width, int32_t height, uint32_t color);
void lgfx_c_draw_line_rgb332(lgfx_target_t target, int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint8_t color);
void lgfx_c_draw_line_rgb888(lgfx_target_t target, int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color);
void lgfx_c_draw_rect_rgb332(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, uint8_t color);
void lgfx_c_draw_rect_rgb888(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);

void lgfx_c_push_image_grayscale(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data);
void lgfx_c_push_image_rgb332(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data);
void lgfx_c_push_image_rgb888(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data);

bool lgfx_c_draw_png(lgfx_target_t target, const uint8_t *data, uint32_t len, int32_t x, int32_t y, int32_t maxWidth, int32_t maxHeight, int32_t offX, int32_t offY, float scale_x, float scale_y, textdatum_t datum);

lgfx_target_t lgfx_c_create_sprite(lgfx_target_t target, int32_t w, int32_t h);
lgfx_target_t lgfx_c_create_sprite_static(lgfx_target_t target, int32_t w, int32_t h, void* buffer, uint8_t bpp);
void lgfx_c_push_sprite(lgfx_target_t target, int32_t x, int32_t y);
void lgfx_c_delete_sprite(lgfx_target_t target);

void lgfx_c_start_write(lgfx_target_t target);
void lgfx_c_end_write(lgfx_target_t target);

size_t lgfx_c_write(lgfx_target_t target, const uint8_t* buffer, size_t length);
void lgfx_c_set_cursor(lgfx_target_t target, int32_t x, int32_t y);
void lgfx_c_set_text_size(lgfx_target_t target, float sx, float sy);
void lgfx_c_set_text_datum(lgfx_target_t target, textdatum_t datum);
size_t lgfx_c_draw_char_rgb332(lgfx_target_t target, int32_t x, int32_t y, uint16_t unicode, uint8_t color, uint8_t bg, float size_x, float size_y);
size_t lgfx_c_draw_char_rgb888(lgfx_target_t target, int32_t x, int32_t y, uint16_t unicode, uint32_t color, uint32_t bg, float size_x, float size_y);

const void* lgfx_c_get_font(lgfx_target_t target);
bool lgfx_c_set_font(lgfx_target_t target, const void* font);
void lgfx_c_font_get_default_metrics(const void* font, font_metrics_t *metrics);
bool lgfx_c_font_update_font_metrics(const void* font, font_metrics_t *metrics, uint16_t unicode);

void lgfx_c_panel_sdl_event_handler(void);

#ifdef __cplusplus
}
#endif