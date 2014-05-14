# Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
#
# The Original software, FMOD library, is provided by FIRELIGHT TECHNOLOGIES.
#
# This software is provided 'as-is', without any express or implied warranty.
# In no event will the authors be held liable for any damages arising from
# the use of this software.
#
# Permission is granted to anyone to use this software for any purpose,
# including commercial applications, and to alter it and redistribute it
# freely, subject to the following restrictions:
#
# 1. The origin of this software must not be misrepresented; you must not claim
#    that you wrote the original software. If you use this software in a product,
#    an acknowledgment in the product documentation would be appreciated but is
#    not required.
#
# 2. Altered source versions must be plainly marked as such, and must not be
#    misrepresented as being the original software.
#
# 3. This notice may not be removed or altered from any source distribution.

all: rfmod examples

rfmod:
	mkdir -p lib
	rustc --out-dir=lib src/fmod.rs

examples: rfmod
	rustc -o bin/simple_music_reader -L ./lib examples/simple_music_reader/main.rs


clean:
	rm -rf lib
	rm -rf bin/simple_music_reader