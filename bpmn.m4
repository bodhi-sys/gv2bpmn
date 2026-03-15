changequote(`[[', `]]')dnl
define([[check_node]], [[ifdef([[NODE_$1]], , [[ERROR: no such node '$1'
]])]])dnl
define([[start]], [[START_NODE [label="$1", shape=circle]
define([[NODE_START_NODE]], [[1]])]])dnl
define([[end]], [[END_NODE [label="$1", shape=doublecircle]
define([[NODE_END_NODE]], [[1]])]])dnl
define([[t]], [[ifdef([[NODE_$1]], [[ERROR: duplicate task node '$1'
]], [[$1 [label="$2"]
define([[NODE_$1]], [[1]])]])]])dnl
define([[g_xor]], [[$1 [label="$2", shape=diamond]
define([[NODE_$1]], [[1]])]])dnl
define([[g_or]], [[$1 [label="$2", shape=diamond]
define([[NODE_$1]], [[1]])]])dnl
define([[g_and]], [[$1 [label="$2", shape=diamond]
define([[NODE_$1]], [[1]])]])dnl
define([[to]], [[check_node([[$1]])check_node([[$2]])$1 -> $2]])dnl
define([[g_to]], [[check_node([[$1]])check_node([[$2]])$1 -> $2 [label="$3"]]])dnl
