/*
The file is generated using these scripts

URL="https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/linux/input-event-codes.h"

curl -s "$URL" |\
 grep -Eo "define.*?(KEY_|BTN_)+[a-zA-Z0-9_]*\s+(0x[0-9a-f]*|[0-9]+)+" |\
 awk '{ printf "%s %s\n", tolower($3), tolower($2) }' |\
 sed "s/key_//" |\
 awk '{ printf "\"%s\" => %s,\n", $2, $1 }' > /tmp/name_to_code

curl -s "$URL" |\
 grep -Eo "define.*?(KEY_|BTN_)+[a-zA-Z0-9_]*\s+(0x[0-9a-f]*|[0-9]+)+" |\
 awk '{ printf "%s %s\n", tolower($3), tolower($2) }' |\
 sed "s/key_//" |\
 awk '{ printf "%s => \"%s\",\n", $1, $2 }' > /tmp/code_to_name
 */

#[allow(unreachable_patterns)]
pub fn code_to_name(x: i32) -> &'static str {
	match x {
		0 => "reserved",
		1 => "esc",
		2 => "1",
		3 => "2",
		4 => "3",
		5 => "4",
		6 => "5",
		7 => "6",
		8 => "7",
		9 => "8",
		10 => "9",
		11 => "0",
		12 => "minus",
		13 => "equal",
		14 => "backspace",
		15 => "tab",
		16 => "q",
		17 => "w",
		18 => "e",
		19 => "r",
		20 => "t",
		21 => "y",
		22 => "u",
		23 => "i",
		24 => "o",
		25 => "p",
		26 => "leftbrace",
		27 => "rightbrace",
		28 => "enter",
		29 => "leftctrl",
		30 => "a",
		31 => "s",
		32 => "d",
		33 => "f",
		34 => "g",
		35 => "h",
		36 => "j",
		37 => "k",
		38 => "l",
		39 => "semicolon",
		40 => "apostrophe",
		41 => "grave",
		42 => "leftshift",
		43 => "backslash",
		44 => "z",
		45 => "x",
		46 => "c",
		47 => "v",
		48 => "b",
		49 => "n",
		50 => "m",
		51 => "comma",
		52 => "dot",
		53 => "slash",
		54 => "rightshift",
		55 => "kpasterisk",
		56 => "leftalt",
		57 => "space",
		58 => "capslock",
		59 => "f1",
		60 => "f2",
		61 => "f3",
		62 => "f4",
		63 => "f5",
		64 => "f6",
		65 => "f7",
		66 => "f8",
		67 => "f9",
		68 => "f10",
		69 => "numlock",
		70 => "scrolllock",
		71 => "kp7",
		72 => "kp8",
		73 => "kp9",
		74 => "kpminus",
		75 => "kp4",
		76 => "kp5",
		77 => "kp6",
		78 => "kpplus",
		79 => "kp1",
		80 => "kp2",
		81 => "kp3",
		82 => "kp0",
		83 => "kpdot",
		85 => "zenkakuhankaku",
		86 => "102nd",
		87 => "f11",
		88 => "f12",
		89 => "ro",
		90 => "katakana",
		91 => "hiragana",
		92 => "henkan",
		93 => "katakanahiragana",
		94 => "muhenkan",
		95 => "kpjpcomma",
		96 => "kpenter",
		97 => "rightctrl",
		98 => "kpslash",
		99 => "sysrq",
		100 => "rightalt",
		101 => "linefeed",
		102 => "home",
		103 => "up",
		104 => "pageup",
		105 => "left",
		106 => "right",
		107 => "end",
		108 => "down",
		109 => "pagedown",
		110 => "insert",
		111 => "delete",
		112 => "macro",
		113 => "mute",
		114 => "volumedown",
		115 => "volumeup",
		116 => "power",
		117 => "kpequal",
		118 => "kpplusminus",
		119 => "pause",
		120 => "scale",
		121 => "kpcomma",
		122 => "hangeul",
		123 => "hanja",
		124 => "yen",
		125 => "leftmeta",
		126 => "rightmeta",
		127 => "compose",
		128 => "stop",
		129 => "again",
		130 => "props",
		131 => "undo",
		132 => "front",
		133 => "copy",
		134 => "open",
		135 => "paste",
		136 => "find",
		137 => "cut",
		138 => "help",
		139 => "menu",
		140 => "calc",
		141 => "setup",
		142 => "sleep",
		143 => "wakeup",
		144 => "file",
		145 => "sendfile",
		146 => "deletefile",
		147 => "xfer",
		148 => "prog1",
		149 => "prog2",
		150 => "www",
		151 => "msdos",
		152 => "coffee",
		153 => "rotate_display",
		154 => "cyclewindows",
		155 => "mail",
		156 => "bookmarks",
		157 => "computer",
		158 => "back",
		159 => "forward",
		160 => "closecd",
		161 => "ejectcd",
		162 => "ejectclosecd",
		163 => "nextsong",
		164 => "playpause",
		165 => "previoussong",
		166 => "stopcd",
		167 => "record",
		168 => "rewind",
		169 => "phone",
		170 => "iso",
		171 => "config",
		172 => "homepage",
		173 => "refresh",
		174 => "exit",
		175 => "move",
		176 => "edit",
		177 => "scrollup",
		178 => "scrolldown",
		179 => "kpleftparen",
		180 => "kprightparen",
		181 => "new",
		182 => "redo",
		183 => "f13",
		184 => "f14",
		185 => "f15",
		186 => "f16",
		187 => "f17",
		188 => "f18",
		189 => "f19",
		190 => "f20",
		191 => "f21",
		192 => "f22",
		193 => "f23",
		194 => "f24",
		200 => "playcd",
		201 => "pausecd",
		202 => "prog3",
		203 => "prog4",
		204 => "dashboard",
		205 => "suspend",
		206 => "close",
		207 => "play",
		208 => "fastforward",
		209 => "bassboost",
		210 => "print",
		211 => "hp",
		212 => "camera",
		213 => "sound",
		214 => "question",
		215 => "email",
		216 => "chat",
		217 => "search",
		218 => "connect",
		219 => "finance",
		220 => "sport",
		221 => "shop",
		222 => "alterase",
		223 => "cancel",
		224 => "brightnessdown",
		225 => "brightnessup",
		226 => "media",
		227 => "switchvideomode",
		228 => "kbdillumtoggle",
		229 => "kbdillumdown",
		230 => "kbdillumup",
		231 => "send",
		232 => "reply",
		233 => "forwardmail",
		234 => "save",
		235 => "documents",
		236 => "battery",
		237 => "bluetooth",
		238 => "wlan",
		239 => "uwb",
		240 => "unknown",
		241 => "video_next",
		242 => "video_prev",
		243 => "brightness_cycle",
		244 => "brightness_auto",
		245 => "display_off",
		246 => "wwan",
		247 => "rfkill",
		248 => "micmute",
		0x100 => "btn_misc",
		0x100 => "btn_0",
		0x101 => "btn_1",
		0x102 => "btn_2",
		0x103 => "btn_3",
		0x104 => "btn_4",
		0x105 => "btn_5",
		0x106 => "btn_6",
		0x107 => "btn_7",
		0x108 => "btn_8",
		0x109 => "btn_9",
		0x110 => "btn_mouse",
		0x110 => "btn_left",
		0x111 => "btn_right",
		0x112 => "btn_middle",
		0x113 => "btn_side",
		0x114 => "btn_extra",
		0x115 => "btn_forward",
		0x116 => "btn_back",
		0x117 => "btn_task",
		0x120 => "btn_joystick",
		0x120 => "btn_trigger",
		0x121 => "btn_thumb",
		0x122 => "btn_thumb2",
		0x123 => "btn_top",
		0x124 => "btn_top2",
		0x125 => "btn_pinkie",
		0x126 => "btn_base",
		0x127 => "btn_base2",
		0x128 => "btn_base3",
		0x129 => "btn_base4",
		0x12a => "btn_base5",
		0x12b => "btn_base6",
		0x12f => "btn_dead",
		0x130 => "btn_gamepad",
		0x130 => "btn_south",
		0x131 => "btn_east",
		0x132 => "btn_c",
		0x133 => "btn_north",
		0x134 => "btn_west",
		0x135 => "btn_z",
		0x136 => "btn_tl",
		0x137 => "btn_tr",
		0x138 => "btn_tl2",
		0x139 => "btn_tr2",
		0x13a => "btn_select",
		0x13b => "btn_start",
		0x13c => "btn_mode",
		0x13d => "btn_thumbl",
		0x13e => "btn_thumbr",
		0x140 => "btn_digi",
		0x140 => "btn_tool_pen",
		0x141 => "btn_tool_rubber",
		0x142 => "btn_tool_brush",
		0x143 => "btn_tool_pencil",
		0x144 => "btn_tool_airbrush",
		0x145 => "btn_tool_finger",
		0x146 => "btn_tool_mouse",
		0x147 => "btn_tool_lens",
		0x148 => "btn_tool_quinttap",
		0x149 => "btn_stylus3",
		0x14a => "btn_touch",
		0x14b => "btn_stylus",
		0x14c => "btn_stylus2",
		0x14d => "btn_tool_doubletap",
		0x14e => "btn_tool_tripletap",
		0x14f => "btn_tool_quadtap",
		0x150 => "btn_wheel",
		0x150 => "btn_gear_down",
		0x151 => "btn_gear_up",
		0x160 => "ok",
		0x161 => "select",
		0x162 => "goto",
		0x163 => "clear",
		0x164 => "power2",
		0x165 => "option",
		0x166 => "info",
		0x167 => "time",
		0x168 => "vendor",
		0x169 => "archive",
		0x16a => "program",
		0x16b => "channel",
		0x16c => "favorites",
		0x16d => "epg",
		0x16e => "pvr",
		0x16f => "mhp",
		0x170 => "language",
		0x171 => "title",
		0x172 => "subtitle",
		0x173 => "angle",
		0x174 => "full_screen",
		0x175 => "mode",
		0x176 => "keyboard",
		0x177 => "aspect_ratio",
		0x178 => "pc",
		0x179 => "tv",
		0x17a => "tv2",
		0x17b => "vcr",
		0x17c => "vcr2",
		0x17d => "sat",
		0x17e => "sat2",
		0x17f => "cd",
		0x180 => "tape",
		0x181 => "radio",
		0x182 => "tuner",
		0x183 => "player",
		0x184 => "text",
		0x185 => "dvd",
		0x186 => "aux",
		0x187 => "mp3",
		0x188 => "audio",
		0x189 => "video",
		0x18a => "directory",
		0x18b => "list",
		0x18c => "memo",
		0x18d => "calendar",
		0x18e => "red",
		0x18f => "green",
		0x190 => "yellow",
		0x191 => "blue",
		0x192 => "channelup",
		0x193 => "channeldown",
		0x194 => "first",
		0x195 => "last",
		0x196 => "ab",
		0x197 => "next",
		0x198 => "restart",
		0x199 => "slow",
		0x19a => "shuffle",
		0x19b => "break",
		0x19c => "previous",
		0x19d => "digits",
		0x19e => "teen",
		0x19f => "twen",
		0x1a0 => "videophone",
		0x1a1 => "games",
		0x1a2 => "zoomin",
		0x1a3 => "zoomout",
		0x1a4 => "zoomreset",
		0x1a5 => "wordprocessor",
		0x1a6 => "editor",
		0x1a7 => "spreadsheet",
		0x1a8 => "graphicseditor",
		0x1a9 => "presentation",
		0x1aa => "database",
		0x1ab => "news",
		0x1ac => "voicemail",
		0x1ad => "addressbook",
		0x1ae => "messenger",
		0x1af => "displaytoggle",
		0x1b0 => "spellcheck",
		0x1b1 => "logoff",
		0x1b2 => "dollar",
		0x1b3 => "euro",
		0x1b4 => "frameback",
		0x1b5 => "frameforward",
		0x1b6 => "context_menu",
		0x1b7 => "media_repeat",
		0x1b8 => "10channelsup",
		0x1b9 => "10channelsdown",
		0x1ba => "images",
		0x1bc => "notification_center",
		0x1bd => "pickup_phone",
		0x1be => "hangup_phone",
		0x1c0 => "del_eol",
		0x1c1 => "del_eos",
		0x1c2 => "ins_line",
		0x1c3 => "del_line",
		0x1d0 => "fn",
		0x1d1 => "fn_esc",
		0x1d2 => "fn_f1",
		0x1d3 => "fn_f2",
		0x1d4 => "fn_f3",
		0x1d5 => "fn_f4",
		0x1d6 => "fn_f5",
		0x1d7 => "fn_f6",
		0x1d8 => "fn_f7",
		0x1d9 => "fn_f8",
		0x1da => "fn_f9",
		0x1db => "fn_f10",
		0x1dc => "fn_f11",
		0x1dd => "fn_f12",
		0x1de => "fn_1",
		0x1df => "fn_2",
		0x1e0 => "fn_d",
		0x1e1 => "fn_e",
		0x1e2 => "fn_f",
		0x1e3 => "fn_s",
		0x1e4 => "fn_b",
		0x1e5 => "fn_right_shift",
		0x1f1 => "brl_dot1",
		0x1f2 => "brl_dot2",
		0x1f3 => "brl_dot3",
		0x1f4 => "brl_dot4",
		0x1f5 => "brl_dot5",
		0x1f6 => "brl_dot6",
		0x1f7 => "brl_dot7",
		0x1f8 => "brl_dot8",
		0x1f9 => "brl_dot9",
		0x1fa => "brl_dot10",
		0x200 => "numeric_0",
		0x201 => "numeric_1",
		0x202 => "numeric_2",
		0x203 => "numeric_3",
		0x204 => "numeric_4",
		0x205 => "numeric_5",
		0x206 => "numeric_6",
		0x207 => "numeric_7",
		0x208 => "numeric_8",
		0x209 => "numeric_9",
		0x20a => "numeric_star",
		0x20b => "numeric_pound",
		0x20c => "numeric_a",
		0x20d => "numeric_b",
		0x20e => "numeric_c",
		0x20f => "numeric_d",
		0x210 => "camera_focus",
		0x211 => "wps_button",
		0x212 => "touchpad_toggle",
		0x213 => "touchpad_on",
		0x214 => "touchpad_off",
		0x215 => "camera_zoomin",
		0x216 => "camera_zoomout",
		0x217 => "camera_up",
		0x218 => "camera_down",
		0x219 => "camera_left",
		0x21a => "camera_right",
		0x21b => "attendant_on",
		0x21c => "attendant_off",
		0x21d => "attendant_toggle",
		0x21e => "lights_toggle",
		0x220 => "btn_dpad_up",
		0x221 => "btn_dpad_down",
		0x222 => "btn_dpad_left",
		0x223 => "btn_dpad_right",
		0x230 => "als_toggle",
		0x231 => "rotate_lock_toggle",
		0x240 => "buttonconfig",
		0x241 => "taskmanager",
		0x242 => "journal",
		0x243 => "controlpanel",
		0x244 => "appselect",
		0x245 => "screensaver",
		0x246 => "voicecommand",
		0x247 => "assistant",
		0x248 => "kbd_layout_next",
		0x250 => "brightness_min",
		0x251 => "brightness_max",
		0x260 => "kbdinputassist_prev",
		0x261 => "kbdinputassist_next",
		0x262 => "kbdinputassist_prevgroup",
		0x263 => "kbdinputassist_nextgroup",
		0x264 => "kbdinputassist_accept",
		0x265 => "kbdinputassist_cancel",
		0x266 => "right_up",
		0x267 => "right_down",
		0x268 => "left_up",
		0x269 => "left_down",
		0x26a => "root_menu",
		0x26b => "media_top_menu",
		0x26c => "numeric_11",
		0x26d => "numeric_12",
		0x26e => "audio_desc",
		0x26f => "3d_mode",
		0x270 => "next_favorite",
		0x271 => "stop_record",
		0x272 => "pause_record",
		0x273 => "vod",
		0x274 => "unmute",
		0x275 => "fastreverse",
		0x276 => "slowreverse",
		0x277 => "data",
		0x278 => "onscreen_keyboard",
		0x279 => "privacy_screen_toggle",
		0x27a => "selective_screenshot",
		0x290 => "macro1",
		0x291 => "macro2",
		0x292 => "macro3",
		0x293 => "macro4",
		0x294 => "macro5",
		0x295 => "macro6",
		0x296 => "macro7",
		0x297 => "macro8",
		0x298 => "macro9",
		0x299 => "macro10",
		0x29a => "macro11",
		0x29b => "macro12",
		0x29c => "macro13",
		0x29d => "macro14",
		0x29e => "macro15",
		0x29f => "macro16",
		0x2a0 => "macro17",
		0x2a1 => "macro18",
		0x2a2 => "macro19",
		0x2a3 => "macro20",
		0x2a4 => "macro21",
		0x2a5 => "macro22",
		0x2a6 => "macro23",
		0x2a7 => "macro24",
		0x2a8 => "macro25",
		0x2a9 => "macro26",
		0x2aa => "macro27",
		0x2ab => "macro28",
		0x2ac => "macro29",
		0x2ad => "macro30",
		0x2b0 => "macro_record_start",
		0x2b1 => "macro_record_stop",
		0x2b2 => "macro_preset_cycle",
		0x2b3 => "macro_preset1",
		0x2b4 => "macro_preset2",
		0x2b5 => "macro_preset3",
		0x2b8 => "kbd_lcd_menu1",
		0x2b9 => "kbd_lcd_menu2",
		0x2ba => "kbd_lcd_menu3",
		0x2bb => "kbd_lcd_menu4",
		0x2bc => "kbd_lcd_menu5",
		0x2c0 => "btn_trigger_happy",
		0x2c0 => "btn_trigger_happy1",
		0x2c1 => "btn_trigger_happy2",
		0x2c2 => "btn_trigger_happy3",
		0x2c3 => "btn_trigger_happy4",
		0x2c4 => "btn_trigger_happy5",
		0x2c5 => "btn_trigger_happy6",
		0x2c6 => "btn_trigger_happy7",
		0x2c7 => "btn_trigger_happy8",
		0x2c8 => "btn_trigger_happy9",
		0x2c9 => "btn_trigger_happy10",
		0x2ca => "btn_trigger_happy11",
		0x2cb => "btn_trigger_happy12",
		0x2cc => "btn_trigger_happy13",
		0x2cd => "btn_trigger_happy14",
		0x2ce => "btn_trigger_happy15",
		0x2cf => "btn_trigger_happy16",
		0x2d0 => "btn_trigger_happy17",
		0x2d1 => "btn_trigger_happy18",
		0x2d2 => "btn_trigger_happy19",
		0x2d3 => "btn_trigger_happy20",
		0x2d4 => "btn_trigger_happy21",
		0x2d5 => "btn_trigger_happy22",
		0x2d6 => "btn_trigger_happy23",
		0x2d7 => "btn_trigger_happy24",
		0x2d8 => "btn_trigger_happy25",
		0x2d9 => "btn_trigger_happy26",
		0x2da => "btn_trigger_happy27",
		0x2db => "btn_trigger_happy28",
		0x2dc => "btn_trigger_happy29",
		0x2dd => "btn_trigger_happy30",
		0x2de => "btn_trigger_happy31",
		0x2df => "btn_trigger_happy32",
		0x2e0 => "btn_trigger_happy33",
		0x2e1 => "btn_trigger_happy34",
		0x2e2 => "btn_trigger_happy35",
		0x2e3 => "btn_trigger_happy36",
		0x2e4 => "btn_trigger_happy37",
		0x2e5 => "btn_trigger_happy38",
		0x2e6 => "btn_trigger_happy39",
		0x2e7 => "btn_trigger_happy40",
		0x2ff => "max",
		_ => panic!(format!("Unknown keycode {}", x))
	}
}


pub fn name_to_code(x: &str) -> i32 {
	match x {
		"reserved" => 0,
		"esc" => 1,
		"1" => 2,
		"2" => 3,
		"3" => 4,
		"4" => 5,
		"5" => 6,
		"6" => 7,
		"7" => 8,
		"8" => 9,
		"9" => 10,
		"0" => 11,
		"minus" => 12,
		"equal" => 13,
		"backspace" => 14,
		"tab" => 15,
		"q" => 16,
		"w" => 17,
		"e" => 18,
		"r" => 19,
		"t" => 20,
		"y" => 21,
		"u" => 22,
		"i" => 23,
		"o" => 24,
		"p" => 25,
		"leftbrace" => 26,
		"rightbrace" => 27,
		"enter" => 28,
		"leftctrl" => 29,
		"a" => 30,
		"s" => 31,
		"d" => 32,
		"f" => 33,
		"g" => 34,
		"h" => 35,
		"j" => 36,
		"k" => 37,
		"l" => 38,
		"semicolon" => 39,
		"apostrophe" => 40,
		"grave" => 41,
		"leftshift" => 42,
		"backslash" => 43,
		"z" => 44,
		"x" => 45,
		"c" => 46,
		"v" => 47,
		"b" => 48,
		"n" => 49,
		"m" => 50,
		"comma" => 51,
		"dot" => 52,
		"slash" => 53,
		"rightshift" => 54,
		"kpasterisk" => 55,
		"leftalt" => 56,
		"space" => 57,
		"capslock" => 58,
		"f1" => 59,
		"f2" => 60,
		"f3" => 61,
		"f4" => 62,
		"f5" => 63,
		"f6" => 64,
		"f7" => 65,
		"f8" => 66,
		"f9" => 67,
		"f10" => 68,
		"numlock" => 69,
		"scrolllock" => 70,
		"kp7" => 71,
		"kp8" => 72,
		"kp9" => 73,
		"kpminus" => 74,
		"kp4" => 75,
		"kp5" => 76,
		"kp6" => 77,
		"kpplus" => 78,
		"kp1" => 79,
		"kp2" => 80,
		"kp3" => 81,
		"kp0" => 82,
		"kpdot" => 83,
		"zenkakuhankaku" => 85,
		"102nd" => 86,
		"f11" => 87,
		"f12" => 88,
		"ro" => 89,
		"katakana" => 90,
		"hiragana" => 91,
		"henkan" => 92,
		"katakanahiragana" => 93,
		"muhenkan" => 94,
		"kpjpcomma" => 95,
		"kpenter" => 96,
		"rightctrl" => 97,
		"kpslash" => 98,
		"sysrq" => 99,
		"rightalt" => 100,
		"linefeed" => 101,
		"home" => 102,
		"up" => 103,
		"pageup" => 104,
		"left" => 105,
		"right" => 106,
		"end" => 107,
		"down" => 108,
		"pagedown" => 109,
		"insert" => 110,
		"delete" => 111,
		"macro" => 112,
		"mute" => 113,
		"volumedown" => 114,
		"volumeup" => 115,
		"power" => 116,
		"kpequal" => 117,
		"kpplusminus" => 118,
		"pause" => 119,
		"scale" => 120,
		"kpcomma" => 121,
		"hangeul" => 122,
		"hanja" => 123,
		"yen" => 124,
		"leftmeta" => 125,
		"rightmeta" => 126,
		"compose" => 127,
		"stop" => 128,
		"again" => 129,
		"props" => 130,
		"undo" => 131,
		"front" => 132,
		"copy" => 133,
		"open" => 134,
		"paste" => 135,
		"find" => 136,
		"cut" => 137,
		"help" => 138,
		"menu" => 139,
		"calc" => 140,
		"setup" => 141,
		"sleep" => 142,
		"wakeup" => 143,
		"file" => 144,
		"sendfile" => 145,
		"deletefile" => 146,
		"xfer" => 147,
		"prog1" => 148,
		"prog2" => 149,
		"www" => 150,
		"msdos" => 151,
		"coffee" => 152,
		"rotate_display" => 153,
		"cyclewindows" => 154,
		"mail" => 155,
		"bookmarks" => 156,
		"computer" => 157,
		"back" => 158,
		"forward" => 159,
		"closecd" => 160,
		"ejectcd" => 161,
		"ejectclosecd" => 162,
		"nextsong" => 163,
		"playpause" => 164,
		"previoussong" => 165,
		"stopcd" => 166,
		"record" => 167,
		"rewind" => 168,
		"phone" => 169,
		"iso" => 170,
		"config" => 171,
		"homepage" => 172,
		"refresh" => 173,
		"exit" => 174,
		"move" => 175,
		"edit" => 176,
		"scrollup" => 177,
		"scrolldown" => 178,
		"kpleftparen" => 179,
		"kprightparen" => 180,
		"new" => 181,
		"redo" => 182,
		"f13" => 183,
		"f14" => 184,
		"f15" => 185,
		"f16" => 186,
		"f17" => 187,
		"f18" => 188,
		"f19" => 189,
		"f20" => 190,
		"f21" => 191,
		"f22" => 192,
		"f23" => 193,
		"f24" => 194,
		"playcd" => 200,
		"pausecd" => 201,
		"prog3" => 202,
		"prog4" => 203,
		"dashboard" => 204,
		"suspend" => 205,
		"close" => 206,
		"play" => 207,
		"fastforward" => 208,
		"bassboost" => 209,
		"print" => 210,
		"hp" => 211,
		"camera" => 212,
		"sound" => 213,
		"question" => 214,
		"email" => 215,
		"chat" => 216,
		"search" => 217,
		"connect" => 218,
		"finance" => 219,
		"sport" => 220,
		"shop" => 221,
		"alterase" => 222,
		"cancel" => 223,
		"brightnessdown" => 224,
		"brightnessup" => 225,
		"media" => 226,
		"switchvideomode" => 227,
		"kbdillumtoggle" => 228,
		"kbdillumdown" => 229,
		"kbdillumup" => 230,
		"send" => 231,
		"reply" => 232,
		"forwardmail" => 233,
		"save" => 234,
		"documents" => 235,
		"battery" => 236,
		"bluetooth" => 237,
		"wlan" => 238,
		"uwb" => 239,
		"unknown" => 240,
		"video_next" => 241,
		"video_prev" => 242,
		"brightness_cycle" => 243,
		"brightness_auto" => 244,
		"display_off" => 245,
		"wwan" => 246,
		"rfkill" => 247,
		"micmute" => 248,
		"btn_misc" => 0x100,
		"btn_0" => 0x100,
		"btn_1" => 0x101,
		"btn_2" => 0x102,
		"btn_3" => 0x103,
		"btn_4" => 0x104,
		"btn_5" => 0x105,
		"btn_6" => 0x106,
		"btn_7" => 0x107,
		"btn_8" => 0x108,
		"btn_9" => 0x109,
		"btn_mouse" => 0x110,
		"btn_left" => 0x110,
		"btn_right" => 0x111,
		"btn_middle" => 0x112,
		"btn_side" => 0x113,
		"btn_extra" => 0x114,
		"btn_forward" => 0x115,
		"btn_back" => 0x116,
		"btn_task" => 0x117,
		"btn_joystick" => 0x120,
		"btn_trigger" => 0x120,
		"btn_thumb" => 0x121,
		"btn_thumb2" => 0x122,
		"btn_top" => 0x123,
		"btn_top2" => 0x124,
		"btn_pinkie" => 0x125,
		"btn_base" => 0x126,
		"btn_base2" => 0x127,
		"btn_base3" => 0x128,
		"btn_base4" => 0x129,
		"btn_base5" => 0x12a,
		"btn_base6" => 0x12b,
		"btn_dead" => 0x12f,
		"btn_gamepad" => 0x130,
		"btn_south" => 0x130,
		"btn_east" => 0x131,
		"btn_c" => 0x132,
		"btn_north" => 0x133,
		"btn_west" => 0x134,
		"btn_z" => 0x135,
		"btn_tl" => 0x136,
		"btn_tr" => 0x137,
		"btn_tl2" => 0x138,
		"btn_tr2" => 0x139,
		"btn_select" => 0x13a,
		"btn_start" => 0x13b,
		"btn_mode" => 0x13c,
		"btn_thumbl" => 0x13d,
		"btn_thumbr" => 0x13e,
		"btn_digi" => 0x140,
		"btn_tool_pen" => 0x140,
		"btn_tool_rubber" => 0x141,
		"btn_tool_brush" => 0x142,
		"btn_tool_pencil" => 0x143,
		"btn_tool_airbrush" => 0x144,
		"btn_tool_finger" => 0x145,
		"btn_tool_mouse" => 0x146,
		"btn_tool_lens" => 0x147,
		"btn_tool_quinttap" => 0x148,
		"btn_stylus3" => 0x149,
		"btn_touch" => 0x14a,
		"btn_stylus" => 0x14b,
		"btn_stylus2" => 0x14c,
		"btn_tool_doubletap" => 0x14d,
		"btn_tool_tripletap" => 0x14e,
		"btn_tool_quadtap" => 0x14f,
		"btn_wheel" => 0x150,
		"btn_gear_down" => 0x150,
		"btn_gear_up" => 0x151,
		"ok" => 0x160,
		"select" => 0x161,
		"goto" => 0x162,
		"clear" => 0x163,
		"power2" => 0x164,
		"option" => 0x165,
		"info" => 0x166,
		"time" => 0x167,
		"vendor" => 0x168,
		"archive" => 0x169,
		"program" => 0x16a,
		"channel" => 0x16b,
		"favorites" => 0x16c,
		"epg" => 0x16d,
		"pvr" => 0x16e,
		"mhp" => 0x16f,
		"language" => 0x170,
		"title" => 0x171,
		"subtitle" => 0x172,
		"angle" => 0x173,
		"full_screen" => 0x174,
		"mode" => 0x175,
		"keyboard" => 0x176,
		"aspect_ratio" => 0x177,
		"pc" => 0x178,
		"tv" => 0x179,
		"tv2" => 0x17a,
		"vcr" => 0x17b,
		"vcr2" => 0x17c,
		"sat" => 0x17d,
		"sat2" => 0x17e,
		"cd" => 0x17f,
		"tape" => 0x180,
		"radio" => 0x181,
		"tuner" => 0x182,
		"player" => 0x183,
		"text" => 0x184,
		"dvd" => 0x185,
		"aux" => 0x186,
		"mp3" => 0x187,
		"audio" => 0x188,
		"video" => 0x189,
		"directory" => 0x18a,
		"list" => 0x18b,
		"memo" => 0x18c,
		"calendar" => 0x18d,
		"red" => 0x18e,
		"green" => 0x18f,
		"yellow" => 0x190,
		"blue" => 0x191,
		"channelup" => 0x192,
		"channeldown" => 0x193,
		"first" => 0x194,
		"last" => 0x195,
		"ab" => 0x196,
		"next" => 0x197,
		"restart" => 0x198,
		"slow" => 0x199,
		"shuffle" => 0x19a,
		"break" => 0x19b,
		"previous" => 0x19c,
		"digits" => 0x19d,
		"teen" => 0x19e,
		"twen" => 0x19f,
		"videophone" => 0x1a0,
		"games" => 0x1a1,
		"zoomin" => 0x1a2,
		"zoomout" => 0x1a3,
		"zoomreset" => 0x1a4,
		"wordprocessor" => 0x1a5,
		"editor" => 0x1a6,
		"spreadsheet" => 0x1a7,
		"graphicseditor" => 0x1a8,
		"presentation" => 0x1a9,
		"database" => 0x1aa,
		"news" => 0x1ab,
		"voicemail" => 0x1ac,
		"addressbook" => 0x1ad,
		"messenger" => 0x1ae,
		"displaytoggle" => 0x1af,
		"spellcheck" => 0x1b0,
		"logoff" => 0x1b1,
		"dollar" => 0x1b2,
		"euro" => 0x1b3,
		"frameback" => 0x1b4,
		"frameforward" => 0x1b5,
		"context_menu" => 0x1b6,
		"media_repeat" => 0x1b7,
		"10channelsup" => 0x1b8,
		"10channelsdown" => 0x1b9,
		"images" => 0x1ba,
		"notification_center" => 0x1bc,
		"pickup_phone" => 0x1bd,
		"hangup_phone" => 0x1be,
		"del_eol" => 0x1c0,
		"del_eos" => 0x1c1,
		"ins_line" => 0x1c2,
		"del_line" => 0x1c3,
		"fn" => 0x1d0,
		"fn_esc" => 0x1d1,
		"fn_f1" => 0x1d2,
		"fn_f2" => 0x1d3,
		"fn_f3" => 0x1d4,
		"fn_f4" => 0x1d5,
		"fn_f5" => 0x1d6,
		"fn_f6" => 0x1d7,
		"fn_f7" => 0x1d8,
		"fn_f8" => 0x1d9,
		"fn_f9" => 0x1da,
		"fn_f10" => 0x1db,
		"fn_f11" => 0x1dc,
		"fn_f12" => 0x1dd,
		"fn_1" => 0x1de,
		"fn_2" => 0x1df,
		"fn_d" => 0x1e0,
		"fn_e" => 0x1e1,
		"fn_f" => 0x1e2,
		"fn_s" => 0x1e3,
		"fn_b" => 0x1e4,
		"fn_right_shift" => 0x1e5,
		"brl_dot1" => 0x1f1,
		"brl_dot2" => 0x1f2,
		"brl_dot3" => 0x1f3,
		"brl_dot4" => 0x1f4,
		"brl_dot5" => 0x1f5,
		"brl_dot6" => 0x1f6,
		"brl_dot7" => 0x1f7,
		"brl_dot8" => 0x1f8,
		"brl_dot9" => 0x1f9,
		"brl_dot10" => 0x1fa,
		"numeric_0" => 0x200,
		"numeric_1" => 0x201,
		"numeric_2" => 0x202,
		"numeric_3" => 0x203,
		"numeric_4" => 0x204,
		"numeric_5" => 0x205,
		"numeric_6" => 0x206,
		"numeric_7" => 0x207,
		"numeric_8" => 0x208,
		"numeric_9" => 0x209,
		"numeric_star" => 0x20a,
		"numeric_pound" => 0x20b,
		"numeric_a" => 0x20c,
		"numeric_b" => 0x20d,
		"numeric_c" => 0x20e,
		"numeric_d" => 0x20f,
		"camera_focus" => 0x210,
		"wps_button" => 0x211,
		"touchpad_toggle" => 0x212,
		"touchpad_on" => 0x213,
		"touchpad_off" => 0x214,
		"camera_zoomin" => 0x215,
		"camera_zoomout" => 0x216,
		"camera_up" => 0x217,
		"camera_down" => 0x218,
		"camera_left" => 0x219,
		"camera_right" => 0x21a,
		"attendant_on" => 0x21b,
		"attendant_off" => 0x21c,
		"attendant_toggle" => 0x21d,
		"lights_toggle" => 0x21e,
		"btn_dpad_up" => 0x220,
		"btn_dpad_down" => 0x221,
		"btn_dpad_left" => 0x222,
		"btn_dpad_right" => 0x223,
		"als_toggle" => 0x230,
		"rotate_lock_toggle" => 0x231,
		"buttonconfig" => 0x240,
		"taskmanager" => 0x241,
		"journal" => 0x242,
		"controlpanel" => 0x243,
		"appselect" => 0x244,
		"screensaver" => 0x245,
		"voicecommand" => 0x246,
		"assistant" => 0x247,
		"kbd_layout_next" => 0x248,
		"brightness_min" => 0x250,
		"brightness_max" => 0x251,
		"kbdinputassist_prev" => 0x260,
		"kbdinputassist_next" => 0x261,
		"kbdinputassist_prevgroup" => 0x262,
		"kbdinputassist_nextgroup" => 0x263,
		"kbdinputassist_accept" => 0x264,
		"kbdinputassist_cancel" => 0x265,
		"right_up" => 0x266,
		"right_down" => 0x267,
		"left_up" => 0x268,
		"left_down" => 0x269,
		"root_menu" => 0x26a,
		"media_top_menu" => 0x26b,
		"numeric_11" => 0x26c,
		"numeric_12" => 0x26d,
		"audio_desc" => 0x26e,
		"3d_mode" => 0x26f,
		"next_favorite" => 0x270,
		"stop_record" => 0x271,
		"pause_record" => 0x272,
		"vod" => 0x273,
		"unmute" => 0x274,
		"fastreverse" => 0x275,
		"slowreverse" => 0x276,
		"data" => 0x277,
		"onscreen_keyboard" => 0x278,
		"privacy_screen_toggle" => 0x279,
		"selective_screenshot" => 0x27a,
		"macro1" => 0x290,
		"macro2" => 0x291,
		"macro3" => 0x292,
		"macro4" => 0x293,
		"macro5" => 0x294,
		"macro6" => 0x295,
		"macro7" => 0x296,
		"macro8" => 0x297,
		"macro9" => 0x298,
		"macro10" => 0x299,
		"macro11" => 0x29a,
		"macro12" => 0x29b,
		"macro13" => 0x29c,
		"macro14" => 0x29d,
		"macro15" => 0x29e,
		"macro16" => 0x29f,
		"macro17" => 0x2a0,
		"macro18" => 0x2a1,
		"macro19" => 0x2a2,
		"macro20" => 0x2a3,
		"macro21" => 0x2a4,
		"macro22" => 0x2a5,
		"macro23" => 0x2a6,
		"macro24" => 0x2a7,
		"macro25" => 0x2a8,
		"macro26" => 0x2a9,
		"macro27" => 0x2aa,
		"macro28" => 0x2ab,
		"macro29" => 0x2ac,
		"macro30" => 0x2ad,
		"macro_record_start" => 0x2b0,
		"macro_record_stop" => 0x2b1,
		"macro_preset_cycle" => 0x2b2,
		"macro_preset1" => 0x2b3,
		"macro_preset2" => 0x2b4,
		"macro_preset3" => 0x2b5,
		"kbd_lcd_menu1" => 0x2b8,
		"kbd_lcd_menu2" => 0x2b9,
		"kbd_lcd_menu3" => 0x2ba,
		"kbd_lcd_menu4" => 0x2bb,
		"kbd_lcd_menu5" => 0x2bc,
		"btn_trigger_happy" => 0x2c0,
		"btn_trigger_happy1" => 0x2c0,
		"btn_trigger_happy2" => 0x2c1,
		"btn_trigger_happy3" => 0x2c2,
		"btn_trigger_happy4" => 0x2c3,
		"btn_trigger_happy5" => 0x2c4,
		"btn_trigger_happy6" => 0x2c5,
		"btn_trigger_happy7" => 0x2c6,
		"btn_trigger_happy8" => 0x2c7,
		"btn_trigger_happy9" => 0x2c8,
		"btn_trigger_happy10" => 0x2c9,
		"btn_trigger_happy11" => 0x2ca,
		"btn_trigger_happy12" => 0x2cb,
		"btn_trigger_happy13" => 0x2cc,
		"btn_trigger_happy14" => 0x2cd,
		"btn_trigger_happy15" => 0x2ce,
		"btn_trigger_happy16" => 0x2cf,
		"btn_trigger_happy17" => 0x2d0,
		"btn_trigger_happy18" => 0x2d1,
		"btn_trigger_happy19" => 0x2d2,
		"btn_trigger_happy20" => 0x2d3,
		"btn_trigger_happy21" => 0x2d4,
		"btn_trigger_happy22" => 0x2d5,
		"btn_trigger_happy23" => 0x2d6,
		"btn_trigger_happy24" => 0x2d7,
		"btn_trigger_happy25" => 0x2d8,
		"btn_trigger_happy26" => 0x2d9,
		"btn_trigger_happy27" => 0x2da,
		"btn_trigger_happy28" => 0x2db,
		"btn_trigger_happy29" => 0x2dc,
		"btn_trigger_happy30" => 0x2dd,
		"btn_trigger_happy31" => 0x2de,
		"btn_trigger_happy32" => 0x2df,
		"btn_trigger_happy33" => 0x2e0,
		"btn_trigger_happy34" => 0x2e1,
		"btn_trigger_happy35" => 0x2e2,
		"btn_trigger_happy36" => 0x2e3,
		"btn_trigger_happy37" => 0x2e4,
		"btn_trigger_happy38" => 0x2e5,
		"btn_trigger_happy39" => 0x2e6,
		"btn_trigger_happy40" => 0x2e7,
		"max" => 0x2ff,
		_ => -1
	}
}