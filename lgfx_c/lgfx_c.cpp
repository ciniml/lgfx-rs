// SPDX-License-Identifier: BSL-1.0
// Copyright Kenta Ida 2022.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

// See README.md for license details.

#include "lgfx_c.h"

#define LGFX_USE_V1
#define LGFX_AUTODETECT
#include <LovyanGFX.hpp>
#include <stdint.h>

static LGFX gfx;
using namespace lgfx::v1;


lgfx_target_t lgfx_c_setup(void) 
{
    gfx.init();
    return reinterpret_cast<lgfx_target_t>(static_cast<LovyanGFX*>(&gfx));
}

::epd_mode_t lgfx_c_get_epd_mode(lgfx_target_t target) {
    auto gfx = static_cast<LGFX*>(reinterpret_cast<LovyanGFX*>(target));
    return static_cast<::epd_mode_t>(gfx->getEpdMode());
}
void lgfx_c_set_epd_mode(lgfx_target_t target, ::epd_mode_t epd_mode) {
    auto gfx = static_cast<LGFX*>(reinterpret_cast<LovyanGFX*>(target));
    gfx->setEpdMode(static_cast<lgfx::v1::epd_mode_t>(epd_mode));
}
bool lgfx_c_is_epd(lgfx_target_t target) {
    auto gfx = static_cast<LGFX*>(reinterpret_cast<LovyanGFX*>(target));
    return gfx->isEPD();
}
void lgfx_c_set_rotation(lgfx_target_t target, uint_fast8_t rotation) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->setRotation(rotation);
}

int32_t lgfx_c_width(lgfx_target_t target) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->width();
}
int32_t lgfx_c_height(lgfx_target_t target) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->height();
}

int32_t lgfx_c_font_height(lgfx_target_t target) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->fontHeight();
}

void lgfx_c_start_write(lgfx_target_t target) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->startWrite();
}
void lgfx_c_end_write(lgfx_target_t target) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->endWrite();
}

void lgfx_c_clear_rgb332(lgfx_target_t target, uint8_t color) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->clear(color);
}
void lgfx_c_clear_rgb888(lgfx_target_t target, uint32_t color) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->clear(color);
}

void lgfx_c_fill_rect_rgb332(lgfx_target_t target, int32_t left, int32_t top, int32_t width, int32_t height, uint8_t color) { 
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->fillRect(left, top, width, height, rgb332_t(color));
}
void lgfx_c_fill_rect_rgb888(lgfx_target_t target, int32_t left, int32_t top, int32_t width, int32_t height, uint32_t color) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->fillRect(left, top, width, height, rgb888_t(color));
}

void lgfx_c_draw_line_rgb332(lgfx_target_t target, int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint8_t color){
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->drawLine(x0, y0, x1, y1, color);
}
void lgfx_c_draw_line_rgb888(lgfx_target_t target, int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color){
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->drawLine(x0, y0, x1, y1, color);
}

void lgfx_c_push_image_grayscale(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->pushGrayscaleImage(x, y, w, h, data, color_depth_t::grayscale_8bit, TFT_WHITE, TFT_BLACK);
}
void lgfx_c_push_image_rgb332(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->pushImage(x, y, w, h, reinterpret_cast<const rgb332_t*>(data));
}
void lgfx_c_push_image_rgb888(lgfx_target_t target, int32_t x, int32_t y, int32_t w, int32_t h, const uint8_t* data) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->pushImage(x, y, w, h, reinterpret_cast<const rgb888_t*>(data));
}

bool lgfx_c_draw_png(lgfx_target_t target, const uint8_t *data, uint32_t len, int32_t x, int32_t y, int32_t maxWidth, int32_t maxHeight, int32_t offX, int32_t offY, float scale_x, float scale_y, ::textdatum_t datum) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->drawPng(data, len, x, y, maxWidth, maxHeight, offX, offY, scale_x, scale_y, static_cast<datum_t>(datum));
}

lgfx_target_t lgfx_c_create_sprite(lgfx_target_t target, int32_t w, int32_t h) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    auto sprite = new LGFX_Sprite(gfx);
    if( sprite == nullptr ) return nullptr;
    if( sprite->createSprite(w, h) == nullptr ) {
        delete sprite;
        return nullptr;
    }
    return reinterpret_cast<lgfx_target_t>(static_cast<LovyanGFX*>(sprite));
}
lgfx_target_t lgfx_c_create_sprite_static(lgfx_target_t target, int32_t w, int32_t h, void* buffer, uint8_t bpp) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    auto sprite = new LGFX_Sprite(gfx);
    if( sprite == nullptr ) return nullptr;
    sprite->setBuffer(buffer, w, h, bpp);
    return reinterpret_cast<lgfx_target_t>(static_cast<LovyanGFX*>(sprite));
}
void lgfx_c_push_sprite(lgfx_target_t target, int32_t x, int32_t y) {
    auto sprite = static_cast<LGFX_Sprite*>(reinterpret_cast<LovyanGFX*>(target));
    sprite->pushSprite(x, y);
}
void lgfx_c_delete_sprite(lgfx_target_t target) {
    if( target != nullptr ) {
        auto sprite = static_cast<LGFX_Sprite*>(reinterpret_cast<LovyanGFX*>(target));
        delete sprite;
    }
}

size_t lgfx_c_write(lgfx_target_t target, const uint8_t* buffer, size_t length) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->write(buffer, length);
}
void lgfx_c_set_cursor(lgfx_target_t target, int32_t x, int32_t y) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->setCursor(x, y);
}
void lgfx_c_set_text_size(lgfx_target_t target, float sx, float sy) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->setTextSize(sx, sy);
}
size_t lgfx_c_draw_char_rgb332(lgfx_target_t target, int32_t x, int32_t y, uint16_t unicode, uint8_t color, uint8_t bg, float size_x, float size_y) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->drawChar(x, y, unicode, color, bg, size_x, size_y);
}
size_t lgfx_c_draw_char_rgb888(lgfx_target_t target, int32_t x, int32_t y, uint16_t unicode, uint32_t color, uint32_t bg, float size_x, float size_y) {
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    return gfx->drawChar(x, y, unicode, color, bg, size_x, size_y);
}

bool lgfx_c_set_font(lgfx_target_t target, const void* font) {
    if( font == nullptr ) return false;
    // IFontじゃなくて実装型のポインタを直で受けてるけど、純粋仮想関数だけなら問題ないんだっけ…
    // 今のところは動いてるけど、キャスト済みの変数定義したほうがいいかも？
    auto ifont = reinterpret_cast<const IFont*>(font);
    auto gfx = reinterpret_cast<LovyanGFX*>(target);
    gfx->setFont(ifont);
    return true;
}
