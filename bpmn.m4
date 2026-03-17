changequote(`[[', `]]')dnl
define([[check_node]], [[ifdef([[NODE_$1]], , [[ERROR: no such node '$1'
]])]])dnl
define([[get_id]], [["ifdef([[PREFIX_$1]], [[defn([[PREFIX_$1]])]], [[$1]])"]])dnl
define([[start]], [[define([[NODE_START_NODE]], [[1]])dnl
define([[PREFIX_START_NODE]], [[start__START_NODE]])dnl
get_id([[START_NODE]]) [label="$1", shape=circle]]])dnl
define([[end]], [[define([[NODE_END_NODE]], [[1]])dnl
define([[PREFIX_END_NODE]], [[end__END_NODE]])dnl
get_id([[END_NODE]]) [label="$1", shape=doublecircle]]])dnl
define([[t]], [[ifdef([[NODE_$1]], [[ERROR: duplicate task node '$1'
]], [[define([[NODE_$1]], [[1]])dnl
define([[PREFIX_$1]], [[t__$1]])dnl
get_id([[$1]]) [label="$2"]]])]])dnl
define([[g_xor]], [[define([[NODE_$1]], [[1]])dnl
define([[PREFIX_$1]], [[g_xor__$1]])dnl
get_id([[$1]]) [label="$2", shape=diamond]]])dnl
define([[g_or]], [[define([[NODE_$1]], [[1]])dnl
define([[PREFIX_$1]], [[g_or__$1]])dnl
get_id([[$1]]) [label="$2", shape=diamond]]])dnl
define([[g_and]], [[define([[NODE_$1]], [[1]])dnl
define([[PREFIX_$1]], [[g_and__$1]])dnl
get_id([[$1]]) [label="$2", shape=diamond]]])dnl
define([[to]], [[check_node([[$1]])check_node([[$2]])get_id([[$1]]) -> get_id([[$2]])]])dnl
define([[g_to]], [[check_node([[$1]])check_node([[$2]])get_id([[$1]]) -> get_id([[$2]]) [label="$3"]]])dnl

