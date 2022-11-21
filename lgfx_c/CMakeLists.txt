# SPDX-License-Identifier: BSL-1.0
# Copyright Kenta Ida 2022.
# Distributed under the Boost Software License, Version 1.0.
#    (See accompanying file LICENSE_1_0.txt or copy at
#          https://www.boost.org/LICENSE_1_0.txt)
# See README.md for license details.

idf_component_register(SRCS "lgfx_c.cpp"
                       INCLUDE_DIRS "."
                       REQUIRES LovyanGFX)